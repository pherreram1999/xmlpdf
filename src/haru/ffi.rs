#![allow(non_camel_case_types, dead_code)]

use std::os::raw::{c_char, c_int, c_uint, c_ulong, c_void};

pub type HPDF_Doc = *mut c_void;
pub type HPDF_Page = *mut c_void;
pub type HPDF_Font = *mut c_void;
pub type HPDF_Image = *mut c_void;
pub type HPDF_ExtGState = *mut c_void;
pub type HPDF_STATUS = c_ulong;
pub type HPDF_REAL = f32;
pub type HPDF_UINT = c_uint;
pub type HPDF_INT = c_int;
pub type HPDF_UINT16 = u16;
pub type HPDF_BYTE = u8;
pub type HPDF_BOOL = c_int;

pub type HPDF_Error_Handler = Option<
    unsafe extern "C" fn(
        error_no: HPDF_STATUS,
        detail_no: HPDF_STATUS,
        user_data: *mut c_void,
    ),
>;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum HPDF_PageSizes {
    Letter = 0,
    Legal = 1,
    A3 = 2,
    A4 = 3,
    A5 = 4,
    B4 = 5,
    B5 = 6,
    Executive = 7,
    US4x6 = 8,
    US4x8 = 9,
    US5x7 = 10,
    Comm10 = 11,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum HPDF_PageDirection {
    Portrait = 0,
    Landscape = 1,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum HPDF_TextAlignment {
    Left = 0,
    Right = 1,
    Center = 2,
    Justify = 3,
}

pub const HPDF_COMP_NONE: HPDF_UINT = 0;
pub const HPDF_COMP_TEXT: HPDF_UINT = 1;
pub const HPDF_COMP_IMAGE: HPDF_UINT = 2;
pub const HPDF_COMP_METADATA: HPDF_UINT = 4;
pub const HPDF_COMP_ALL: HPDF_UINT = 15;

pub const HPDF_TRUE: HPDF_BOOL = 1;
pub const HPDF_FALSE: HPDF_BOOL = 0;

extern "C" {
    pub fn HPDF_New(user_error_fn: HPDF_Error_Handler, user_data: *mut c_void) -> HPDF_Doc;
    pub fn HPDF_Free(pdf: HPDF_Doc);
    pub fn HPDF_NewDoc(pdf: HPDF_Doc) -> HPDF_STATUS;
    pub fn HPDF_FreeDoc(pdf: HPDF_Doc) -> HPDF_STATUS;
    pub fn HPDF_HasDoc(pdf: HPDF_Doc) -> HPDF_BOOL;
    pub fn HPDF_SetCompressionMode(pdf: HPDF_Doc, mode: HPDF_UINT) -> HPDF_STATUS;
    pub fn HPDF_UseUTFEncodings(pdf: HPDF_Doc) -> HPDF_STATUS;

    pub fn HPDF_CreateExtGState(pdf: HPDF_Doc) -> HPDF_ExtGState;
    pub fn HPDF_ExtGState_SetAlphaStroke(ext_gstate: HPDF_ExtGState, value: HPDF_REAL) -> HPDF_STATUS;
    pub fn HPDF_ExtGState_SetAlphaFill(ext_gstate: HPDF_ExtGState, value: HPDF_REAL) -> HPDF_STATUS;
    pub fn HPDF_Page_SetExtGState(page: HPDF_Page, ext_gstate: HPDF_ExtGState) -> HPDF_STATUS;

    pub fn HPDF_AddPage(pdf: HPDF_Doc) -> HPDF_Page;
    pub fn HPDF_InsertPage(pdf: HPDF_Doc, page: HPDF_Page) -> HPDF_Page;
    pub fn HPDF_GetCurrentPage(pdf: HPDF_Doc) -> HPDF_Page;

    pub fn HPDF_Page_SetWidth(page: HPDF_Page, value: HPDF_REAL) -> HPDF_STATUS;
    pub fn HPDF_Page_SetHeight(page: HPDF_Page, value: HPDF_REAL) -> HPDF_STATUS;
    pub fn HPDF_Page_SetSize(
        page: HPDF_Page,
        size: HPDF_PageSizes,
        direction: HPDF_PageDirection,
    ) -> HPDF_STATUS;

    pub fn HPDF_GetFont(
        pdf: HPDF_Doc,
        font_name: *const c_char,
        encoding_name: *const c_char,
    ) -> HPDF_Font;
    pub fn HPDF_LoadTTFontFromFile(
        pdf: HPDF_Doc,
        file_name: *const c_char,
        embedding: HPDF_BOOL,
    ) -> *const c_char;

    pub fn HPDF_Page_BeginText(page: HPDF_Page) -> HPDF_STATUS;
    pub fn HPDF_Page_EndText(page: HPDF_Page) -> HPDF_STATUS;
    pub fn HPDF_Page_SetFontAndSize(
        page: HPDF_Page,
        font: HPDF_Font,
        size: HPDF_REAL,
    ) -> HPDF_STATUS;
    pub fn HPDF_Page_MoveTextPos(page: HPDF_Page, x: HPDF_REAL, y: HPDF_REAL) -> HPDF_STATUS;
    pub fn HPDF_Page_ShowText(page: HPDF_Page, text: *const c_char) -> HPDF_STATUS;
    pub fn HPDF_Page_TextOut(
        page: HPDF_Page,
        xpos: HPDF_REAL,
        ypos: HPDF_REAL,
        text: *const c_char,
    ) -> HPDF_STATUS;
    pub fn HPDF_Page_TextRect(
        page: HPDF_Page,
        left: HPDF_REAL,
        top: HPDF_REAL,
        right: HPDF_REAL,
        bottom: HPDF_REAL,
        text: *const c_char,
        align: HPDF_TextAlignment,
        len: *mut u32,
    ) -> HPDF_STATUS;

    pub fn HPDF_Page_SetRGBFill(
        page: HPDF_Page,
        r: HPDF_REAL,
        g: HPDF_REAL,
        b: HPDF_REAL,
    ) -> HPDF_STATUS;
    pub fn HPDF_Page_SetRGBStroke(
        page: HPDF_Page,
        r: HPDF_REAL,
        g: HPDF_REAL,
        b: HPDF_REAL,
    ) -> HPDF_STATUS;
    pub fn HPDF_Page_SetLineWidth(page: HPDF_Page, width: HPDF_REAL) -> HPDF_STATUS;

    pub fn HPDF_Page_Rectangle(
        page: HPDF_Page,
        x: HPDF_REAL,
        y: HPDF_REAL,
        width: HPDF_REAL,
        height: HPDF_REAL,
    ) -> HPDF_STATUS;
    pub fn HPDF_Page_Stroke(page: HPDF_Page) -> HPDF_STATUS;
    pub fn HPDF_Page_Fill(page: HPDF_Page) -> HPDF_STATUS;
    pub fn HPDF_Page_FillStroke(page: HPDF_Page) -> HPDF_STATUS;
    pub fn HPDF_Page_MoveTo(page: HPDF_Page, x: HPDF_REAL, y: HPDF_REAL) -> HPDF_STATUS;
    pub fn HPDF_Page_LineTo(page: HPDF_Page, x: HPDF_REAL, y: HPDF_REAL) -> HPDF_STATUS;
    pub fn HPDF_Page_CurveTo(
        page: HPDF_Page,
        x1: HPDF_REAL,
        y1: HPDF_REAL,
        x2: HPDF_REAL,
        y2: HPDF_REAL,
        x3: HPDF_REAL,
        y3: HPDF_REAL,
    ) -> HPDF_STATUS;

    pub fn HPDF_Page_TextWidth(page: HPDF_Page, text: *const c_char) -> HPDF_REAL;

    pub fn HPDF_SaveToStream(pdf: HPDF_Doc) -> HPDF_STATUS;
    pub fn HPDF_GetStreamSize(pdf: HPDF_Doc) -> u32;
    pub fn HPDF_ReadFromStream(pdf: HPDF_Doc, buf: *mut u8, size: *mut u32) -> HPDF_STATUS;
    pub fn HPDF_ResetStream(pdf: HPDF_Doc) -> HPDF_STATUS;

    pub fn HPDF_LoadJpegImageFromFile(pdf: HPDF_Doc, filename: *const c_char) -> HPDF_Image;
    pub fn HPDF_Page_DrawImage(
        page: HPDF_Page,
        image: HPDF_Image,
        x: HPDF_REAL,
        y: HPDF_REAL,
        width: HPDF_REAL,
        height: HPDF_REAL,
    ) -> HPDF_STATUS;
}
