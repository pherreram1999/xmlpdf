use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::os::raw::c_void;
use std::ptr::null_mut;

use crate::haru::ffi::*;

unsafe extern "C" fn error_handler(error_no: HPDF_STATUS, detail_no: HPDF_STATUS, _user_data: *mut c_void) {
    eprintln!("[Libharu Error] code: 0x{:04X}, detail: {}", error_no, detail_no);
}

#[derive(Clone)]
pub struct FontHandle {
    pub font: HPDF_Font,
    pub is_ttf: bool,
}

pub struct HaruDocument {
    doc: HPDF_Doc,
    font_cache: HashMap<String, FontHandle>,
    ttf_alias_map: HashMap<String, String>,
    image_cache: HashMap<String, HPDF_Image>,
}

impl HaruDocument {
    pub fn new() -> Result<Self, String> {
        unsafe {
            let doc = HPDF_New(Some(error_handler), null_mut());
            if doc.is_null() {
                return Err("Failed to initialize Libharu HPDF_Doc instance".to_string());
            }
            HPDF_UseUTFEncodings(doc);
            let mut haru = Self {
                doc,
                font_cache: HashMap::new(),
                ttf_alias_map: HashMap::new(),
                image_cache: HashMap::new(),
            };
            haru.enable_compression();
            Ok(haru)
        }
    }

    pub fn enable_compression(&mut self) {
        unsafe {
            HPDF_SetCompressionMode(self.doc, HPDF_COMP_ALL);
        }
    }

    pub fn add_page(&mut self) -> HaruPage {
        unsafe {
            let page = HPDF_AddPage(self.doc);
            HaruPage {
                doc: self.doc,
                page,
            }
        }
    }

    pub fn load_ttf_font(&mut self, alias: &str, file_path: &str) -> Result<String, String> {
        let c_path = CString::new(file_path).map_err(|e| e.to_string())?;
        unsafe {
            let font_name_ptr = HPDF_LoadTTFontFromFile(self.doc, c_path.as_ptr(), HPDF_TRUE);
            if font_name_ptr.is_null() {
                return Err(format!("Failed to load TrueType font from path: {}", file_path));
            }
            let font_name = CStr::from_ptr(font_name_ptr)
                .to_string_lossy()
                .into_owned();
            self.ttf_alias_map.insert(alias.to_string(), font_name.clone());
            Ok(font_name)
        }
    }

    pub fn get_font(&mut self, font_name: &str) -> Result<FontHandle, String> {
        let is_ttf = self.ttf_alias_map.contains_key(font_name);
        let actual_name = self
            .ttf_alias_map
            .get(font_name)
            .cloned()
            .unwrap_or_else(|| font_name.to_string());

        if let Some(handle) = self.font_cache.get(&actual_name) {
            return Ok(handle.clone());
        }

        let c_name = CString::new(actual_name.as_str()).map_err(|e| e.to_string())?;
        let encoding = if is_ttf {
            CString::new("UTF-8").unwrap()
        } else {
            CString::new("WinAnsiEncoding").unwrap()
        };

        unsafe {
            let font = HPDF_GetFont(self.doc, c_name.as_ptr(), encoding.as_ptr());
            if font.is_null() {
                let font_fallback = HPDF_GetFont(self.doc, c_name.as_ptr(), null_mut());
                if font_fallback.is_null() {
                    return Err(format!("Failed to get font: {}", font_name));
                }
                let handle = FontHandle { font: font_fallback, is_ttf };
                self.font_cache.insert(actual_name, handle.clone());
                Ok(handle)
            } else {
                let handle = FontHandle { font, is_ttf };
                self.font_cache.insert(actual_name, handle.clone());
                Ok(handle)
            }
        }
    }

    pub fn load_jpeg_image(&mut self, file_path: &str) -> Result<HPDF_Image, String> {
        if let Some(&img) = self.image_cache.get(file_path) {
            return Ok(img);
        }
        let c_path = CString::new(file_path).map_err(|e| e.to_string())?;
        unsafe {
            let img = HPDF_LoadJpegImageFromFile(self.doc, c_path.as_ptr());
            if img.is_null() {
                Err(format!("Failed to load JPEG image: {}", file_path))
            } else {
                self.image_cache.insert(file_path.to_string(), img);
                Ok(img)
            }
        }
    }

    pub fn save_to_vec(&mut self) -> Result<Vec<u8>, String> {
        unsafe {
            let status = HPDF_SaveToStream(self.doc);
            if status != 0 {
                return Err(format!("Failed to save PDF to stream, status: 0x{:04X}", status));
            }

            let stream_size = HPDF_GetStreamSize(self.doc);
            if stream_size == 0 {
                return Ok(Vec::new());
            }

            let mut buffer = vec![0u8; stream_size as usize];
            let mut read_bytes = stream_size;

            HPDF_ResetStream(self.doc);
            let read_status = HPDF_ReadFromStream(self.doc, buffer.as_mut_ptr(), &mut read_bytes);
            if read_status != 0 {
                return Err(format!("Failed to read stream data, status: 0x{:04X}", read_status));
            }

            buffer.truncate(read_bytes as usize);
            Ok(buffer)
        }
    }
}

impl Drop for HaruDocument {
    fn drop(&mut self) {
        unsafe {
            if !self.doc.is_null() {
                HPDF_Free(self.doc);
            }
        }
    }
}

pub fn encode_text_for_haru(text: &str, is_ttf: bool) -> CString {
    if is_ttf {
        CString::new(text).unwrap_or_default()
    } else {
        let mut bytes = Vec::with_capacity(text.len());
        for ch in text.chars() {
            let code = ch as u32;
            if code <= 0xFF {
                bytes.push(code as u8);
            } else {
                match ch {
                    '—' => bytes.push(0x97),
                    '–' => bytes.push(0x96),
                    '“' => bytes.push(0x93),
                    '”' => bytes.push(0x94),
                    '‘' => bytes.push(0x91),
                    '’' => bytes.push(0x92),
                    '•' => bytes.push(0x95),
                    '€' => bytes.push(0x80),
                    _ => bytes.push(b'?'),
                }
            }
        }
        CString::new(bytes).unwrap_or_default()
    }
}

#[derive(Clone, Copy)]
pub struct HaruPage {
    pub doc: HPDF_Doc,
    pub page: HPDF_Page,
}

impl HaruPage {
    pub fn set_dimensions(&self, width: f32, height: f32) {
        unsafe {
            HPDF_Page_SetWidth(self.page, width);
            HPDF_Page_SetHeight(self.page, height);
        }
    }

    pub fn set_fill_color(&self, r: f32, g: f32, b: f32) {
        unsafe {
            HPDF_Page_SetRGBFill(self.page, r, g, b);
        }
    }

    pub fn set_stroke_color(&self, r: f32, g: f32, b: f32) {
        unsafe {
            HPDF_Page_SetRGBStroke(self.page, r, g, b);
        }
    }

    pub fn set_line_width(&self, width: f32) {
        unsafe {
            HPDF_Page_SetLineWidth(self.page, width);
        }
    }

    pub fn set_opacity(&self, opacity: f32) {
        if opacity >= 1.0 {
            return;
        }
        unsafe {
            let gstate = HPDF_CreateExtGState(self.doc);
            if !gstate.is_null() {
                HPDF_ExtGState_SetAlphaFill(gstate, opacity);
                HPDF_ExtGState_SetAlphaStroke(gstate, opacity);
                HPDF_Page_SetExtGState(self.page, gstate);
            }
        }
    }

    pub fn draw_rectangle(&self, x: f32, y: f32, width: f32, height: f32, fill: bool, stroke: bool) {
        unsafe {
            HPDF_Page_Rectangle(self.page, x, y, width, height);
            if fill && stroke {
                HPDF_Page_FillStroke(self.page);
            } else if fill {
                HPDF_Page_Fill(self.page);
            } else if stroke {
                HPDF_Page_Stroke(self.page);
            }
        }
    }

    pub fn draw_rounded_rectangle(
        &self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        radius: f32,
        fill: bool,
        stroke: bool,
    ) {
        let r = radius.min(width / 2.0).min(height / 2.0);
        if r <= 0.0 {
            self.draw_rectangle(x, y, width, height, fill, stroke);
            return;
        }

        let k = 0.55228475 * r;

        unsafe {
            HPDF_Page_MoveTo(self.page, x + r, y);
            HPDF_Page_LineTo(self.page, x + width - r, y);
            HPDF_Page_CurveTo(
                self.page,
                x + width - r + k,
                y,
                x + width,
                y + r - k,
                x + width,
                y + r,
            );
            HPDF_Page_LineTo(self.page, x + width, y + height - r);
            HPDF_Page_CurveTo(
                self.page,
                x + width,
                y + height - r + k,
                x + width - r + k,
                y + height,
                x + width - r,
                y + height,
            );
            HPDF_Page_LineTo(self.page, x + r, y + height);
            HPDF_Page_CurveTo(
                self.page,
                x + r - k,
                y + height,
                x,
                y + height - r + k,
                x,
                y + height - r,
            );
            HPDF_Page_LineTo(self.page, x, y + r);
            HPDF_Page_CurveTo(
                self.page,
                x,
                y + r - k,
                x + r - k,
                y,
                x + r,
                y,
            );

            if fill && stroke {
                HPDF_Page_FillStroke(self.page);
            } else if fill {
                HPDF_Page_Fill(self.page);
            } else if stroke {
                HPDF_Page_Stroke(self.page);
            }
        }
    }

    pub fn draw_line(&self, x1: f32, y1: f32, x2: f32, y2: f32) {
        unsafe {
            HPDF_Page_MoveTo(self.page, x1, y1);
            HPDF_Page_LineTo(self.page, x2, y2);
            HPDF_Page_Stroke(self.page);
        }
    }

    pub fn draw_text(&self, x: f32, y: f32, font_handle: &FontHandle, size: f32, text: &str) {
        let c_text = encode_text_for_haru(text, font_handle.is_ttf);
        unsafe {
            HPDF_Page_BeginText(self.page);
            HPDF_Page_SetFontAndSize(self.page, font_handle.font, size);
            HPDF_Page_TextOut(self.page, x, y, c_text.as_ptr());
            HPDF_Page_EndText(self.page);
        }
    }

    pub fn draw_text_rect(
        &self,
        left: f32,
        top: f32,
        right: f32,
        bottom: f32,
        font_handle: &FontHandle,
        size: f32,
        text: &str,
        align: HPDF_TextAlignment,
    ) {
        let c_text = encode_text_for_haru(text, font_handle.is_ttf);
        unsafe {
            HPDF_Page_BeginText(self.page);
            HPDF_Page_SetFontAndSize(self.page, font_handle.font, size);
            let mut len = 0u32;
            HPDF_Page_TextRect(
                self.page,
                left,
                top,
                right,
                bottom,
                c_text.as_ptr(),
                align,
                &mut len,
            );
            HPDF_Page_EndText(self.page);
        }
    }

    pub fn draw_image(&self, image: HPDF_Image, x: f32, y: f32, width: f32, height: f32) {
        unsafe {
            HPDF_Page_DrawImage(self.page, image, x, y, width, height);
        }
    }

    /// Measures the rendered width (in points) of a text string using the specified font and size.
    pub fn measure_text_width(&self, font_handle: &FontHandle, font_size: f32, text: &str) -> f32 {
        let c_text = encode_text_for_haru(text, font_handle.is_ttf);
        unsafe {
            HPDF_Page_SetFontAndSize(self.page, font_handle.font, font_size);
            HPDF_Page_TextWidth(self.page, c_text.as_ptr())
        }
    }
}
