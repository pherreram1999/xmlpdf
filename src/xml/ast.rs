#[derive(Debug, Clone)]
pub struct ColorRGB {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl ColorRGB {
    pub fn black() -> Self {
        Self { r: 0.0, g: 0.0, b: 0.0 }
    }

    pub fn from_hex(hex: &str) -> Option<Self> {
        let hex = hex.trim_start_matches('#');
        if hex.len() == 6 {
            let r = u8::from_str_radix(&hex[0..2], 16).ok()? as f32 / 255.0;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()? as f32 / 255.0;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()? as f32 / 255.0;
            Some(Self { r, g, b })
        } else if hex.len() == 3 {
            let r = u8::from_str_radix(&hex[0..1].repeat(2), 16).ok()? as f32 / 255.0;
            let g = u8::from_str_radix(&hex[1..2].repeat(2), 16).ok()? as f32 / 255.0;
            let b = u8::from_str_radix(&hex[2..3].repeat(2), 16).ok()? as f32 / 255.0;
            Some(Self { r, g, b })
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Align {
    Left,
    Center,
    Right,
    Justify,
}

#[derive(Debug, Clone)]
pub struct FontDef {
    pub name: String,
    pub path: String,
    pub embed: bool,
}

#[derive(Debug, Clone)]
pub struct TextElement {
    pub content: String,
    pub font: Option<String>,
    pub size: Option<f32>,
    pub color: Option<ColorRGB>,
    pub align: Align,
    pub x: Option<f32>,
    pub y: Option<f32>,
}

#[derive(Debug, Clone)]
pub struct ParagraphElement {
    pub content: String,
    pub font: Option<String>,
    pub size: Option<f32>,
    pub color: Option<ColorRGB>,
    pub align: Align,
    pub margin_bottom: f32,
    pub line_height: Option<f32>,
}

#[derive(Debug, Clone)]
pub struct RectElement {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub border_radius: f32,
    pub fill_color: Option<ColorRGB>,
    pub stroke_color: Option<ColorRGB>,
    pub line_width: f32,
    pub opacity: Option<f32>,
}

#[derive(Debug, Clone)]
pub struct LineElement {
    pub x1: f32,
    pub y1: f32,
    pub x2: f32,
    pub y2: f32,
    pub color: Option<ColorRGB>,
    pub width: f32,
}

#[derive(Debug, Clone)]
pub struct ImageElement {
    pub src: String,
    pub x: Option<f32>,
    pub y: Option<f32>,
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Clone)]
pub struct CellDef {
    pub content: String,
    pub width: Option<f32>,
    pub align: Align,
    pub font: Option<String>,
    pub size: Option<f32>,
    pub color: Option<ColorRGB>,
    pub background: Option<ColorRGB>,
    pub border: bool,
    pub border_color: Option<ColorRGB>,
    pub border_radius: f32,
}

#[derive(Debug, Clone)]
pub struct RowDef {
    pub cells: Vec<CellDef>,
    pub background: Option<ColorRGB>,
    pub align: Option<Align>,
    pub font: Option<String>,
    pub size: Option<f32>,
}

#[derive(Debug, Clone)]
pub struct GridElement {
    pub columns: Vec<f32>,
    pub rows: Vec<RowDef>,
    pub border: bool,
    pub border_color: Option<ColorRGB>,
    pub border_width: f32,
    pub cell_padding: f32,
    pub margin_bottom: f32,
}

#[derive(Debug, Clone)]
pub struct DivElement {
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub background: Option<ColorRGB>,
    pub border_color: Option<ColorRGB>,
    pub border_width: f32,
    pub border_radius: f32,
    pub padding: f32,
    pub margin_bottom: f32,
    pub opacity: Option<f32>,
    pub children: Vec<Element>,
}

#[derive(Debug, Clone)]
pub enum Element {
    Text(TextElement),
    Paragraph(ParagraphElement),
    Rect(RectElement),
    Line(LineElement),
    Image(ImageElement),
    Grid(GridElement),
    Div(DivElement),
    Spacer(f32),
    PageBreak,
}

#[derive(Debug, Clone)]
pub struct PageDef {
    pub page_size: Option<String>,
    pub orientation: Option<String>,
    pub margin_top: Option<f32>,
    pub margin_bottom: Option<f32>,
    pub margin_left: Option<f32>,
    pub margin_right: Option<f32>,
    pub background_image: Option<String>,
    pub elements: Vec<Element>,
}

#[derive(Debug, Clone)]
pub struct PdfDocumentAST {
    pub default_page_size: String,
    pub default_orientation: String,
    pub default_margin_top: f32,
    pub default_margin_bottom: f32,
    pub default_margin_left: f32,
    pub default_margin_right: f32,
    pub default_font: String,
    pub default_size: f32,
    pub default_color: ColorRGB,
    pub default_background_image: Option<String>,
    pub fonts: Vec<FontDef>,
    pub pages: Vec<PageDef>,
}
