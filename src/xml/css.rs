use std::collections::HashMap;
use crate::xml::ast::{Align, ColorRGB};

#[derive(Debug, Clone, Default)]
pub struct CssStyle {
    pub font_family: Option<String>,
    pub font_size: Option<f32>,
    pub font_weight: Option<String>,
    pub color: Option<ColorRGB>,
    pub background: Option<ColorRGB>,
    pub border_color: Option<ColorRGB>,
    pub border_width: Option<f32>,
    pub border_radius: Option<f32>,
    pub padding: Option<f32>,
    pub margin_bottom: Option<f32>,
    pub margin_top: Option<f32>,
    pub opacity: Option<f32>,
    pub text_align: Option<Align>,
    pub width: Option<f32>,
    pub height: Option<f32>,
}

impl CssStyle {
    pub fn merge(&self, other: &CssStyle) -> CssStyle {
        CssStyle {
            font_family: other.font_family.clone().or_else(|| self.font_family.clone()),
            font_size: other.font_size.or(self.font_size),
            font_weight: other.font_weight.clone().or_else(|| self.font_weight.clone()),
            color: other.color.clone().or_else(|| self.color.clone()),
            background: other.background.clone().or_else(|| self.background.clone()),
            border_color: other.border_color.clone().or_else(|| self.border_color.clone()),
            border_width: other.border_width.or(self.border_width),
            border_radius: other.border_radius.or(self.border_radius),
            padding: other.padding.or(self.padding),
            margin_bottom: other.margin_bottom.or(self.margin_bottom),
            margin_top: other.margin_top.or(self.margin_top),
            opacity: other.opacity.or(self.opacity),
            text_align: other.text_align.clone().or_else(|| self.text_align.clone()),
            width: other.width.or(self.width),
            height: other.height.or(self.height),
        }
    }
}

pub fn parse_inline_style(style_str: &str) -> CssStyle {
    let mut style = CssStyle::default();
    for declaration in style_str.split(';') {
        let parts: Vec<&str> = declaration.splitn(2, ':').map(|s| s.trim()).collect();
        if parts.len() == 2 {
            let key = parts[0].to_lowercase();
            let val = parts[1];
            match key.as_str() {
                "color" => style.color = ColorRGB::from_hex(val),
                "background" | "background-color" => style.background = ColorRGB::from_hex(val),
                "font-family" | "font" => style.font_family = Some(val.to_string()),
                "font-size" => style.font_size = parse_dim(val),
                "font-weight" => style.font_weight = Some(val.to_string()),
                "border-color" => style.border_color = ColorRGB::from_hex(val),
                "border-width" => style.border_width = parse_dim(val),
                "border-radius" | "rx" | "ry" => style.border_radius = parse_dim(val),
                "padding" => style.padding = parse_dim(val),
                "margin-bottom" => style.margin_bottom = parse_dim(val),
                "margin-top" => style.margin_top = parse_dim(val),
                "margin" => {
                    let d = parse_dim(val);
                    style.margin_bottom = d;
                    style.margin_top = d;
                }
                "opacity" => style.opacity = val.parse::<f32>().ok(),
                "text-align" | "align" => {
                    style.text_align = match val.to_lowercase().as_str() {
                        "left" => Some(Align::Left),
                        "center" => Some(Align::Center),
                        "right" => Some(Align::Right),
                        "justify" => Some(Align::Justify),
                        _ => None,
                    };
                }
                "width" => style.width = parse_dim(val),
                "height" => style.height = parse_dim(val),
                _ => {}
            }
        }
    }
    style
}

pub fn parse_stylesheet(css_content: &str) -> HashMap<String, CssStyle> {
    let mut classes = HashMap::new();
    let mut current_class = None;
    let mut current_body = String::new();

    for line in css_content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('.') && trimmed.contains('{') {
            if let Some(name) = trimmed.split('{').next() {
                let class_name = name.trim().trim_start_matches('.').to_string();
                current_class = Some(class_name);
                current_body.clear();
            }
        } else if trimmed == "}" {
            if let Some(class_name) = current_class.take() {
                let style = parse_inline_style(&current_body);
                classes.insert(class_name, style);
            }
        } else if current_class.is_some() {
            current_body.push_str(trimmed);
            current_body.push(' ');
        }
    }

    classes
}

fn parse_dim(val: &str) -> Option<f32> {
    let clean = val
        .trim()
        .trim_end_matches("px")
        .trim_end_matches("pt")
        .trim_end_matches("%");
    clean.parse::<f32>().ok()
}
