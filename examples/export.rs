use image::ImageFormat;
use pdfium_render::prelude::*;

pub fn main() -> Result<(), PdfiumError> {
    // Attempt to bind to a pdfium library in the current working directory; failing that,
    // attempt to bind to the system-provided library.

    // The library name will differ depending on the current platform. On Linux,
    // the library will be named libpdfium.so by default; on Windows, pdfium.dll; and on
    // MacOS, libpdfium.dylib. We can use the Pdfium::pdfium_platform_library_name_at_path()
    // function to append the correct library name for the current platform to a path we specify.

    let bindings = Pdfium::bind_to_library(
        // Attempt to bind to a pdfium library in the current working directory...
        Pdfium::pdfium_platform_library_name_at_path("./"),
    )
    .or_else(
        // ... and fall back to binding to a system-provided pdfium library.
        |_| Pdfium::bind_to_system_library(),
    )?;

    // First, we create a set of shared settings that we'll apply to each page in the
    // sample file when rendering. Sharing the same rendering configuration is a good way
    // to ensure homogenous output across all pages in the document.

    let render_config = PdfRenderConfig::new()
        .set_target_width(2000)
        .set_maximum_height(2000)
        .rotate_if_landscape(PdfBitmapRotation::Degrees90, true);

    for (index, page) in Pdfium::new(bindings)
        .load_pdf_from_file("test/export-test.pdf", None)? // Load the sample file...
        .pages()
        .iter() // ... get an iterator across all pages ...
        .enumerate()
    {
        // ... and export each page to a JPEG in the current working directory,
        // using the rendering configuration we created earlier.

        let result = page
            .render_with_config(&render_config)? // Initializes a bitmap with the given configuration for this page ...
            .as_image() // ... renders it to an Image::DynamicImage ...
            .as_rgba8()
            .ok_or(PdfiumError::ImageError)? // ... sets the correct color space ...
            .save_with_format(format!("export-test-page-{}.jpg", index), ImageFormat::Jpeg); // ... and exports it to a JPEG.

        assert!(result.is_ok());
    }

    Ok(())
}
