//! Defines the [PdfPageShadingObject] struct, exposing functionality related to a single
//! page object of type `PdfPageObjectType::Shading`.

use crate::bindgen::{FPDF_ANNOTATION, FPDF_PAGE, FPDF_PAGEOBJECT};
use crate::bindings::PdfiumLibraryBindings;
use crate::page_object_private::internal::PdfPageObjectPrivate;

pub struct PdfPageShadingObject<'a> {
    object_handle: FPDF_PAGEOBJECT,
    page_handle: Option<FPDF_PAGE>,
    annotation_handle: Option<FPDF_ANNOTATION>,
    bindings: &'a dyn PdfiumLibraryBindings,
}

impl<'a> PdfPageShadingObject<'a> {
    pub(crate) fn from_pdfium(
        object_handle: FPDF_PAGEOBJECT,
        page_handle: Option<FPDF_PAGE>,
        annotation_handle: Option<FPDF_ANNOTATION>,
        bindings: &'a dyn PdfiumLibraryBindings,
    ) -> Self {
        PdfPageShadingObject {
            object_handle,
            page_handle,
            annotation_handle,
            bindings,
        }
    }
}

impl<'a> PdfPageObjectPrivate<'a> for PdfPageShadingObject<'a> {
    #[inline]
    fn get_object_handle(&self) -> &FPDF_PAGEOBJECT {
        &self.object_handle
    }

    #[inline]
    fn get_page_handle(&self) -> &Option<FPDF_PAGE> {
        &self.page_handle
    }

    #[inline]
    fn set_page_handle(&mut self, page: FPDF_PAGE) {
        self.page_handle = Some(page);
    }

    #[inline]
    fn clear_page_handle(&mut self) {
        self.page_handle = None;
    }

    #[inline]
    fn get_annotation_handle(&self) -> &Option<FPDF_ANNOTATION> {
        &self.annotation_handle
    }

    #[inline]
    fn set_annotation_handle(&mut self, annotation: FPDF_ANNOTATION) {
        self.annotation_handle = Some(annotation);
    }

    #[inline]
    fn clear_annotation_handle(&mut self) {
        self.annotation_handle = None;
    }

    #[inline]
    fn bindings(&self) -> &dyn PdfiumLibraryBindings {
        self.bindings
    }
}
