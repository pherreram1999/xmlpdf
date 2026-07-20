use std::collections::HashMap;
use roxmltree::{Node, NodeType};
use thiserror::Error;

use crate::xml::ast::*;
use crate::xml::css::{parse_inline_style, parse_stylesheet, CssStyle};

#[derive(Error, Debug)]
pub enum XmlParseError {
    #[error("XML format error: {0}")]
    XmlFormat(#[from] roxmltree::Error),
    #[error("Missing root <pdf> tag")]
    MissingRootTag,
}

pub fn parse_xml_pdf(xml_content: &str) -> Result<PdfDocumentAST, XmlParseError> {
    let doc = roxmltree::Document::parse(xml_content)?;
    let root = doc.root_element();

    if root.tag_name().name() != "pdf" {
        return Err(XmlParseError::MissingRootTag);
    }

    let mut stylesheet: HashMap<String, CssStyle> = HashMap::new();

    // Check for top-level <style> tags
    for child in root.children() {
        if child.node_type() == NodeType::Element && child.tag_name().name() == "style" {
            let css_text = get_node_text(&child);
            stylesheet.extend(parse_stylesheet(&css_text));
        }
    }

    let root_style = get_combined_style(&root, &stylesheet);

    let default_page_size = root.attribute("page-size").unwrap_or("A4").to_string();
    let default_orientation = root.attribute("orientation").unwrap_or("portrait").to_string();
    let default_margin = parse_f32_attr(&root, "margin").unwrap_or(30.0);

    let default_font = root_style
        .font_family
        .or_else(|| root.attribute("font").map(|s| s.to_string()))
        .unwrap_or_else(|| "Helvetica".to_string());

    let default_size = root_style
        .font_size
        .or_else(|| parse_f32_attr(&root, "size"))
        .or_else(|| parse_f32_attr(&root, "font-size"))
        .unwrap_or(10.0);

    let default_color = root_style
        .color
        .or_else(|| parse_color_attr(&root, "color"))
        .unwrap_or_else(ColorRGB::black);

    let mut fonts = Vec::new();
    let mut pages = Vec::new();
    let mut implicit_elements = Vec::new();

    for child in root.children() {
        if child.node_type() != NodeType::Element {
            continue;
        }

        match child.tag_name().name() {
            "fonts" => {
                for font_node in child.children() {
                    if font_node.node_type() == NodeType::Element && font_node.tag_name().name() == "font" {
                        if let (Some(name), Some(path)) = (font_node.attribute("name"), font_node.attribute("path")) {
                            let embed = font_node.attribute("embed").map(|v| v == "true" || v == "1").unwrap_or(true);
                            fonts.push(FontDef {
                                name: name.to_string(),
                                path: path.to_string(),
                                embed,
                            });
                        }
                    }
                }
            }
            "page" => {
                let page = parse_page_node(&child, &stylesheet)?;
                pages.push(page);
            }
            "style" => {} // Handled above
            _ => {
                if let Some(elem) = parse_element_node(&child, &stylesheet)? {
                    implicit_elements.push(elem);
                }
            }
        }
    }

    if pages.is_empty() {
        pages.push(PageDef {
            page_size: None,
            orientation: None,
            margin: None,
            elements: implicit_elements,
        });
    }

    Ok(PdfDocumentAST {
        default_page_size,
        default_orientation,
        default_margin,
        default_font,
        default_size,
        default_color,
        fonts,
        pages,
    })
}

fn parse_page_node(node: &Node, stylesheet: &HashMap<String, CssStyle>) -> Result<PageDef, XmlParseError> {
    let page_size = node.attribute("page-size").map(|s| s.to_string());
    let orientation = node.attribute("orientation").map(|s| s.to_string());
    let margin = parse_f32_attr(node, "margin");

    let mut elements = Vec::new();
    for child in node.children() {
        if child.node_type() == NodeType::Element {
            if let Some(elem) = parse_element_node(&child, stylesheet)? {
                elements.push(elem);
            }
        }
    }

    Ok(PageDef {
        page_size,
        orientation,
        margin,
        elements,
    })
}

fn parse_element_node(
    node: &Node,
    stylesheet: &HashMap<String, CssStyle>,
) -> Result<Option<Element>, XmlParseError> {
    let style = get_combined_style(node, stylesheet);

    match node.tag_name().name() {
        "text" | "span" => {
            let content = get_node_text(node);
            let font = style.font_family.or_else(|| node.attribute("font").map(|s| s.to_string()));
            let size = style.font_size.or_else(|| parse_f32_attr(node, "size")).or_else(|| parse_f32_attr(node, "font-size"));
            let color = style.color.or_else(|| parse_color_attr(node, "color"));
            let align = style.text_align.or_else(|| parse_align_attr(node, "align")).unwrap_or(Align::Left);
            let x = style.width.or_else(|| parse_f32_attr(node, "x"));
            let y = style.height.or_else(|| parse_f32_attr(node, "y"));

            Ok(Some(Element::Text(TextElement {
                content,
                font,
                size,
                color,
                align,
                x,
                y,
            })))
        }
        "paragraph" | "p" => {
            let content = get_node_text(node);
            let font = style.font_family.or_else(|| node.attribute("font").map(|s| s.to_string()));
            let size = style.font_size.or_else(|| parse_f32_attr(node, "size")).or_else(|| parse_f32_attr(node, "font-size"));
            let color = style.color.or_else(|| parse_color_attr(node, "color"));
            let align = style.text_align.or_else(|| parse_align_attr(node, "align")).unwrap_or(Align::Left);
            let margin_bottom = style.margin_bottom.or_else(|| parse_f32_attr(node, "margin-bottom")).unwrap_or(10.0);
            let line_height = parse_f32_attr(node, "line-height");

            Ok(Some(Element::Paragraph(ParagraphElement {
                content,
                font,
                size,
                color,
                align,
                margin_bottom,
                line_height,
            })))
        }
        "rect" | "box" => {
            let x = parse_f32_attr(node, "x").unwrap_or(0.0);
            let y = parse_f32_attr(node, "y").unwrap_or(0.0);
            let width = style.width.or_else(|| parse_f32_attr(node, "width")).unwrap_or(100.0);
            let height = style.height.or_else(|| parse_f32_attr(node, "height")).unwrap_or(50.0);
            let border_radius = style.border_radius.or_else(|| parse_f32_attr(node, "border-radius")).or_else(|| parse_f32_attr(node, "rx")).unwrap_or(0.0);

            let fill_color = style.background.or_else(|| parse_color_attr(node, "fill")).or_else(|| parse_color_attr(node, "background"));
            let stroke_color = style.border_color.or_else(|| parse_color_attr(node, "stroke")).or_else(|| parse_color_attr(node, "border-color"));
            let line_width = style.border_width.or_else(|| parse_f32_attr(node, "line-width")).unwrap_or(1.0);
            let opacity = style.opacity.or_else(|| parse_f32_attr(node, "opacity"));

            Ok(Some(Element::Rect(RectElement {
                x,
                y,
                width,
                height,
                border_radius,
                fill_color,
                stroke_color,
                line_width,
                opacity,
            })))
        }
        "div" | "container" => {
            let width = style.width.or_else(|| parse_f32_attr(node, "width"));
            let height = style.height.or_else(|| parse_f32_attr(node, "height"));
            let background = style.background.or_else(|| parse_color_attr(node, "background")).or_else(|| parse_color_attr(node, "fill"));
            let border_color = style.border_color.or_else(|| parse_color_attr(node, "border-color")).or_else(|| parse_color_attr(node, "stroke"));
            let border_width = style.border_width.or_else(|| parse_f32_attr(node, "border-width")).unwrap_or(1.0);
            let border_radius = style.border_radius.or_else(|| parse_f32_attr(node, "border-radius")).or_else(|| parse_f32_attr(node, "rx")).unwrap_or(0.0);
            let padding = style.padding.or_else(|| parse_f32_attr(node, "padding")).unwrap_or(8.0);
            let margin_bottom = style.margin_bottom.or_else(|| parse_f32_attr(node, "margin-bottom")).unwrap_or(10.0);
            let opacity = style.opacity.or_else(|| parse_f32_attr(node, "opacity"));

            let mut children = Vec::new();
            for child in node.children() {
                if child.node_type() == NodeType::Element {
                    if let Some(child_elem) = parse_element_node(&child, stylesheet)? {
                        children.push(child_elem);
                    }
                }
            }

            Ok(Some(Element::Div(DivElement {
                width,
                height,
                background,
                border_color,
                border_width,
                border_radius,
                padding,
                margin_bottom,
                opacity,
                children,
            })))
        }
        "line" => {
            let x1 = parse_f32_attr(node, "x1").unwrap_or(0.0);
            let y1 = parse_f32_attr(node, "y1").unwrap_or(0.0);
            let x2 = parse_f32_attr(node, "x2").unwrap_or(100.0);
            let y2 = parse_f32_attr(node, "y2").unwrap_or(0.0);
            let color = style.color.or_else(|| parse_color_attr(node, "color"));
            let width = style.border_width.or_else(|| parse_f32_attr(node, "width")).or_else(|| parse_f32_attr(node, "line-width")).unwrap_or(1.0);

            Ok(Some(Element::Line(LineElement {
                x1,
                y1,
                x2,
                y2,
                color,
                width,
            })))
        }
        "image" => {
            let src = node.attribute("src").unwrap_or("").to_string();
            let x = parse_f32_attr(node, "x");
            let y = parse_f32_attr(node, "y");
            let width = style.width.or_else(|| parse_f32_attr(node, "width")).unwrap_or(100.0);
            let height = style.height.or_else(|| parse_f32_attr(node, "height")).unwrap_or(100.0);

            Ok(Some(Element::Image(ImageElement {
                src,
                x,
                y,
                width,
                height,
            })))
        }
        "grid" | "table" => {
            let columns = node
                .attribute("columns")
                .map(|s| {
                    s.split(',')
                        .filter_map(|col| col.trim().parse::<f32>().ok())
                        .collect()
                })
                .unwrap_or_default();

            let border = node.attribute("border").map(|v| v != "0" && v != "false").unwrap_or(true);
            let border_color = style.border_color.or_else(|| parse_color_attr(node, "border-color"));
            let border_width = style.border_width.or_else(|| parse_f32_attr(node, "border-width")).unwrap_or(1.0);
            let cell_padding = style.padding.or_else(|| parse_f32_attr(node, "cell-padding")).unwrap_or(5.0);
            let margin_bottom = style.margin_bottom.or_else(|| parse_f32_attr(node, "margin-bottom")).unwrap_or(10.0);

            let mut rows = Vec::new();
            for child in node.children() {
                if child.node_type() == NodeType::Element && child.tag_name().name() == "row" {
                    let row_style = get_combined_style(&child, stylesheet);
                    let row_bg = row_style.background.or_else(|| parse_color_attr(&child, "background"));
                    let row_align = row_style.text_align.or_else(|| parse_align_attr(&child, "align"));
                    let row_font = row_style.font_family.or_else(|| child.attribute("font").map(|s| s.to_string()));
                    let row_size = row_style.font_size.or_else(|| parse_f32_attr(&child, "size")).or_else(|| parse_f32_attr(&child, "font-size"));

                    let is_bold = child.attribute("bold").map(|v| v == "true" || v == "1").unwrap_or(false);
                    let row_font = if is_bold && row_font.is_none() {
                        Some("Helvetica-Bold".to_string())
                    } else {
                        row_font
                    };

                    let mut cells = Vec::new();
                    for cell_node in child.children() {
                        if cell_node.node_type() == NodeType::Element && cell_node.tag_name().name() == "cell" {
                            let cell_style = get_combined_style(&cell_node, stylesheet);
                            let cell_text = get_node_text(&cell_node);
                            let cell_width = cell_style.width.or_else(|| parse_f32_attr(&cell_node, "width"));
                            let cell_align = cell_style.text_align.or_else(|| parse_align_attr(&cell_node, "align")).unwrap_or(Align::Left);
                            let cell_font = cell_style.font_family.or_else(|| cell_node.attribute("font").map(|s| s.to_string()));
                            let cell_size = cell_style.font_size.or_else(|| parse_f32_attr(&cell_node, "size"));
                            let cell_color = cell_style.color.or_else(|| parse_color_attr(&cell_node, "color"));
                            let cell_bg = cell_style.background.or_else(|| parse_color_attr(&cell_node, "background"));
                            let cell_border = cell_node.attribute("border").map(|v| v != "0" && v != "false").unwrap_or(border);
                            let cell_border_color = cell_style.border_color.or_else(|| parse_color_attr(&cell_node, "border-color")).or(border_color.clone());
                            let cell_border_radius = cell_style.border_radius.or_else(|| parse_f32_attr(&cell_node, "border-radius")).unwrap_or(0.0);

                            cells.push(CellDef {
                                content: cell_text,
                                width: cell_width,
                                align: cell_align,
                                font: cell_font,
                                size: cell_size,
                                color: cell_color,
                                background: cell_bg,
                                border: cell_border,
                                border_color: cell_border_color,
                                border_radius: cell_border_radius,
                            });
                        }
                    }

                    rows.push(RowDef {
                        cells,
                        background: row_bg,
                        align: row_align,
                        font: row_font,
                        size: row_size,
                    });
                }
            }

            Ok(Some(Element::Grid(GridElement {
                columns,
                rows,
                border,
                border_color,
                border_width,
                cell_padding,
                margin_bottom,
            })))
        }
        "spacer" => {
            let height = style.height.or_else(|| parse_f32_attr(node, "height")).unwrap_or(10.0);
            Ok(Some(Element::Spacer(height)))
        }
        "page-break" => Ok(Some(Element::PageBreak)),
        _ => Ok(None),
    }
}

fn get_combined_style(node: &Node, stylesheet: &HashMap<String, CssStyle>) -> CssStyle {
    let mut style = CssStyle::default();

    if let Some(class_attr) = node.attribute("class") {
        for class_name in class_attr.split_whitespace() {
            let name = class_name.trim_start_matches('.');
            if let Some(class_style) = stylesheet.get(name) {
                style = style.merge(class_style);
            }
        }
    }

    if let Some(inline_str) = node.attribute("style") {
        let inline_style = parse_inline_style(inline_str);
        style = style.merge(&inline_style);
    }

    style
}

fn get_node_text(node: &Node) -> String {
    let mut result = String::new();
    for child in node.children() {
        if child.is_text() {
            if let Some(text) = child.text() {
                result.push_str(text);
            }
        } else if child.is_element() && (child.tag_name().name() == "text" || child.tag_name().name() == "span") {
            if let Some(text) = child.text() {
                result.push_str(text);
            }
        }
    }
    result.trim().to_string()
}

fn parse_f32_attr(node: &Node, attr: &str) -> Option<f32> {
    node.attribute(attr).and_then(|val| val.parse::<f32>().ok())
}

fn parse_color_attr(node: &Node, attr: &str) -> Option<ColorRGB> {
    node.attribute(attr).and_then(ColorRGB::from_hex)
}

fn parse_align_attr(node: &Node, attr: &str) -> Option<Align> {
    node.attribute(attr).and_then(|val| match val.to_lowercase().as_str() {
        "left" => Some(Align::Left),
        "center" => Some(Align::Center),
        "right" => Some(Align::Right),
        "justify" => Some(Align::Justify),
        _ => None,
    })
}
