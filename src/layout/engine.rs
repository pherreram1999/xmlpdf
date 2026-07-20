use crate::haru::{HaruDocument, HaruPage, HPDF_TextAlignment};
use crate::xml::ast::*;

pub struct LayoutEngine {
    doc: HaruDocument,
    ast: PdfDocumentAST,
}

impl LayoutEngine {
    pub fn new(ast: PdfDocumentAST) -> Result<Self, String> {
        let mut doc = HaruDocument::new()?;

        // Register custom TrueType fonts
        for font in &ast.fonts {
            if let Err(e) = doc.load_ttf_font(&font.name, &font.path) {
                eprintln!("[Warning] Could not load custom TTF font '{}' from '{}': {}", font.name, font.path, e);
            }
        }

        Ok(Self { doc, ast })
    }

    pub fn render(mut self) -> Result<Vec<u8>, String> {
        for page_def in &self.ast.pages.clone() {
            self.render_page(page_def)?;
        }

        self.doc.save_to_vec()
    }

    fn render_page(&mut self, page_def: &PageDef) -> Result<(), String> {
        let page_size_str = page_def
            .page_size
            .as_ref()
            .unwrap_or(&self.ast.default_page_size);

        let orientation_str = page_def
            .orientation
            .as_ref()
            .unwrap_or(&self.ast.default_orientation);

        let margin = page_def.margin.unwrap_or(self.ast.default_margin);

        let (page_width, page_height) = get_page_dimensions(page_size_str, orientation_str);

        let mut page = self.doc.add_page();
        page.set_dimensions(page_width, page_height);

        let mut cursor_top_y = margin;
        let printable_width = page_width - (2.0 * margin);

        for element in &page_def.elements {
            self.render_element(
                &mut page,
                element,
                &mut cursor_top_y,
                margin,
                printable_width,
                page_height,
            )?;
        }

        Ok(())
    }

    fn render_element(
        &mut self,
        page: &mut HaruPage,
        element: &Element,
        cursor_top_y: &mut f32,
        margin: f32,
        printable_width: f32,
        page_height: f32,
    ) -> Result<(), String> {
        match element {
            Element::Text(text_elem) => {
                self.render_text(page, text_elem, cursor_top_y, margin, page_height)?;
            }
            Element::Paragraph(para_elem) => {
                self.render_paragraph(
                    page,
                    para_elem,
                    cursor_top_y,
                    margin,
                    printable_width,
                    page_height,
                )?;
            }
            Element::Rect(rect_elem) => {
                self.render_rect(page, rect_elem, page_height);
            }
            Element::Line(line_elem) => {
                self.render_line(page, line_elem, page_height);
            }
            Element::Image(img_elem) => {
                self.render_image(page, img_elem, cursor_top_y, margin, page_height);
            }
            Element::Grid(grid_elem) => {
                self.render_grid(
                    page,
                    grid_elem,
                    cursor_top_y,
                    margin,
                    printable_width,
                    page_height,
                )?;
            }
            Element::Div(div_elem) => {
                self.render_div(
                    page,
                    div_elem,
                    cursor_top_y,
                    margin,
                    printable_width,
                    page_height,
                )?;
            }
            Element::Spacer(height) => {
                *cursor_top_y += height;
            }
            Element::PageBreak => {
                *page = self.doc.add_page();
                page.set_dimensions(printable_width + 2.0 * margin, page_height);
                *cursor_top_y = margin;
            }
        }
        Ok(())
    }

    fn render_div(
        &mut self,
        page: &mut HaruPage,
        div: &DivElement,
        cursor_top_y: &mut f32,
        margin: f32,
        printable_width: f32,
        page_height: f32,
    ) -> Result<(), String> {
        let box_w = div.width.unwrap_or(printable_width);
        let padding = div.padding;

        if let Some(opacity) = div.opacity {
            page.set_opacity(opacity);
        }

        let start_y = *cursor_top_y;
        let mut inner_cursor_y = start_y + padding;
        let inner_margin = margin + padding;
        let inner_printable_width = box_w - (2.0 * padding);

        for child in &div.children {
            self.render_element(
                page,
                child,
                &mut inner_cursor_y,
                inner_margin,
                inner_printable_width,
                page_height,
            )?;
        }

        let inner_height = inner_cursor_y - start_y + padding;
        let box_h = div.height.unwrap_or(inner_height);

        let pdf_y = page_height - start_y - box_h;

        if let Some(ref bg) = div.background {
            page.set_fill_color(bg.r, bg.g, bg.b);
        }
        if let Some(ref border) = div.border_color {
            page.set_stroke_color(border.r, border.g, border.b);
        }
        page.set_line_width(div.border_width);

        if div.border_radius > 0.0 {
            page.draw_rounded_rectangle(
                margin,
                pdf_y,
                box_w,
                box_h,
                div.border_radius,
                div.background.is_some(),
                div.border_color.is_some(),
            );
        } else if div.background.is_some() || div.border_color.is_some() {
            page.draw_rectangle(
                margin,
                pdf_y,
                box_w,
                box_h,
                div.background.is_some(),
                div.border_color.is_some(),
            );
        }

        *cursor_top_y = start_y + box_h + div.margin_bottom;
        Ok(())
    }

    fn render_text(
        &mut self,
        page: &mut HaruPage,
        text_elem: &TextElement,
        cursor_top_y: &mut f32,
        margin: f32,
        page_height: f32,
    ) -> Result<(), String> {
        let font_name = text_elem
            .font
            .as_ref()
            .unwrap_or(&self.ast.default_font);
        let font_size = text_elem.size.unwrap_or(self.ast.default_size);
        let font_handle = self.doc.get_font(font_name)?;

        let color = text_elem.color.as_ref().unwrap_or(&self.ast.default_color);
        page.set_fill_color(color.r, color.g, color.b);

        if let (Some(abs_x), Some(abs_y)) = (text_elem.x, text_elem.y) {
            let pdf_y = page_height - abs_y;
            page.draw_text(abs_x, pdf_y, &font_handle, font_size, &text_elem.content);
        } else {
            let x = match text_elem.align {
                Align::Left => margin,
                Align::Center => margin + ((page_height - 2.0 * margin) / 2.0),
                Align::Right => page_height - margin - 100.0,
                Align::Justify => margin,
            };
            let pdf_y = page_height - *cursor_top_y - font_size;
            page.draw_text(x, pdf_y, &font_handle, font_size, &text_elem.content);
            *cursor_top_y += font_size + 4.0;
        }

        Ok(())
    }

    fn render_paragraph(
        &mut self,
        page: &mut HaruPage,
        para_elem: &ParagraphElement,
        cursor_top_y: &mut f32,
        margin: f32,
        printable_width: f32,
        page_height: f32,
    ) -> Result<(), String> {
        let font_name = para_elem
            .font
            .as_ref()
            .unwrap_or(&self.ast.default_font);
        let font_size = para_elem.size.unwrap_or(self.ast.default_size);
        let font_handle = self.doc.get_font(font_name)?;

        let color = para_elem.color.as_ref().unwrap_or(&self.ast.default_color);
        page.set_fill_color(color.r, color.g, color.b);

        let line_height = para_elem.line_height.unwrap_or(font_size * 1.3);
        let align = match para_elem.align {
            Align::Left => HPDF_TextAlignment::Left,
            Align::Center => HPDF_TextAlignment::Center,
            Align::Right => HPDF_TextAlignment::Right,
            Align::Justify => HPDF_TextAlignment::Justify,
        };

        let est_lines = (para_elem.content.len() as f32 / 50.0).max(1.0);
        let est_height = est_lines * line_height;

        let left = margin;
        let top = page_height - *cursor_top_y;
        let right = margin + printable_width;
        let bottom = top - est_height - 20.0;

        page.draw_text_rect(
            left,
            top,
            right,
            bottom,
            &font_handle,
            font_size,
            &para_elem.content,
            align,
        );

        *cursor_top_y += est_height + para_elem.margin_bottom;
        Ok(())
    }

    fn render_rect(&self, page: &HaruPage, rect: &RectElement, page_height: f32) {
        if let Some(opacity) = rect.opacity {
            page.set_opacity(opacity);
        }
        if let Some(ref bg) = rect.fill_color {
            page.set_fill_color(bg.r, bg.g, bg.b);
        }
        if let Some(ref border) = rect.stroke_color {
            page.set_stroke_color(border.r, border.g, border.b);
        }
        page.set_line_width(rect.line_width);

        let pdf_y = page_height - rect.y - rect.height;

        if rect.border_radius > 0.0 {
            page.draw_rounded_rectangle(
                rect.x,
                pdf_y,
                rect.width,
                rect.height,
                rect.border_radius,
                rect.fill_color.is_some(),
                rect.stroke_color.is_some(),
            );
        } else {
            page.draw_rectangle(
                rect.x,
                pdf_y,
                rect.width,
                rect.height,
                rect.fill_color.is_some(),
                rect.stroke_color.is_some(),
            );
        }
    }

    fn render_line(&self, page: &HaruPage, line: &LineElement, page_height: f32) {
        if let Some(ref color) = line.color {
            page.set_stroke_color(color.r, color.g, color.b);
        }
        page.set_line_width(line.width);

        let pdf_y1 = page_height - line.y1;
        let pdf_y2 = page_height - line.y2;
        page.draw_line(line.x1, pdf_y1, line.x2, pdf_y2);
    }

    fn render_image(
        &mut self,
        page: &mut HaruPage,
        img_elem: &ImageElement,
        cursor_top_y: &mut f32,
        margin: f32,
        page_height: f32,
    ) {
        if let Ok(img) = self.doc.load_jpeg_image(&img_elem.src) {
            let x = img_elem.x.unwrap_or(margin);
            let top_y = img_elem.y.unwrap_or(*cursor_top_y);
            let pdf_y = page_height - top_y - img_elem.height;

            page.draw_image(img, x, pdf_y, img_elem.width, img_elem.height);

            if img_elem.y.is_none() {
                *cursor_top_y += img_elem.height + 10.0;
            }
        } else {
            eprintln!("[Warning] Could not load image: {}", img_elem.src);
        }
    }

    fn render_grid(
        &mut self,
        page: &mut HaruPage,
        grid: &GridElement,
        cursor_top_y: &mut f32,
        margin: f32,
        printable_width: f32,
        page_height: f32,
    ) -> Result<(), String> {
        let num_cols = if !grid.columns.is_empty() {
            grid.columns.len()
        } else {
            grid.rows.first().map(|r| r.cells.len()).unwrap_or(1)
        };

        let col_widths: Vec<f32> = if !grid.columns.is_empty() {
            grid.columns.clone()
        } else {
            let equal_w = printable_width / (num_cols as f32);
            vec![equal_w; num_cols]
        };

        for row in &grid.rows {
            let font_name = row
                .font
                .as_ref()
                .unwrap_or(&self.ast.default_font);
            let font_size = row.size.unwrap_or(self.ast.default_size);

            let row_height = font_size + (grid.cell_padding * 2.0) + 4.0;
            let mut col_x = margin;

            for (col_idx, cell) in row.cells.iter().enumerate() {
                let cell_w = cell.width.unwrap_or_else(|| {
                    *col_widths.get(col_idx).unwrap_or(&(printable_width / num_cols as f32))
                });

                let pdf_cell_y = page_height - *cursor_top_y - row_height;

                // Background fill
                let bg_color = cell.background.as_ref().or(row.background.as_ref());
                if let Some(bg) = bg_color {
                    page.set_fill_color(bg.r, bg.g, bg.b);
                    if cell.border_radius > 0.0 {
                        page.draw_rounded_rectangle(col_x, pdf_cell_y, cell_w, row_height, cell.border_radius, true, false);
                    } else {
                        page.draw_rectangle(col_x, pdf_cell_y, cell_w, row_height, true, false);
                    }
                }

                // Border draw
                if cell.border {
                    let border_c = cell.border_color.as_ref().or(grid.border_color.as_ref());
                    if let Some(bc) = border_c {
                        page.set_stroke_color(bc.r, bc.g, bc.b);
                    } else {
                        page.set_stroke_color(0.8, 0.8, 0.8);
                    }
                    page.set_line_width(grid.border_width);
                    if cell.border_radius > 0.0 {
                        page.draw_rounded_rectangle(col_x, pdf_cell_y, cell_w, row_height, cell.border_radius, false, true);
                    } else {
                        page.draw_rectangle(col_x, pdf_cell_y, cell_w, row_height, false, true);
                    }
                }

                // Text draw inside cell
                let cell_font_name = cell.font.as_ref().unwrap_or(font_name);
                let cell_font_size = cell.size.unwrap_or(font_size);
                let cell_font_handle = self.doc.get_font(cell_font_name)?;

                let text_color = cell.color.as_ref().unwrap_or(&self.ast.default_color);
                page.set_fill_color(text_color.r, text_color.g, text_color.b);

                let align_hpdf = match cell.align {
                    Align::Left => HPDF_TextAlignment::Left,
                    Align::Center => HPDF_TextAlignment::Center,
                    Align::Right => HPDF_TextAlignment::Right,
                    Align::Justify => HPDF_TextAlignment::Justify,
                };

                let left = col_x + grid.cell_padding;
                let top = page_height - *cursor_top_y - grid.cell_padding;
                let right = col_x + cell_w - grid.cell_padding;
                let bottom = pdf_cell_y + grid.cell_padding;

                page.draw_text_rect(
                    left,
                    top,
                    right,
                    bottom,
                    &cell_font_handle,
                    cell_font_size,
                    &cell.content,
                    align_hpdf,
                );

                col_x += cell_w;
            }

            *cursor_top_y += row_height;
        }

        *cursor_top_y += grid.margin_bottom;
        Ok(())
    }
}

fn get_page_dimensions(size: &str, orientation: &str) -> (f32, f32) {
    let (w, h) = match size.to_uppercase().as_str() {
        "LETTER" => (612.0, 792.0),
        "LEGAL" => (612.0, 1008.0),
        "A3" => (841.89, 1190.55),
        "A4" => (595.276, 841.89),
        "A5" => (419.53, 595.276),
        "B5" => (498.89, 708.66),
        _ => (595.276, 841.89),
    };

    if orientation.eq_ignore_ascii_case("landscape") {
        (h, w)
    } else {
        (w, h)
    }
}
