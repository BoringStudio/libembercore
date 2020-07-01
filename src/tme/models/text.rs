use serde::Deserialize;
use serde::Serialize;

use crate::tme::color::color_serde;
use crate::tme::color::Color;

use super::utils;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub struct Text {
    #[serde(default = "utils::make_false")]
    pub bold:        bool,
    #[serde(with = "color_serde")]
    #[serde(default = "Color::new_black")]
    pub color:       Color,
    #[serde(rename = "fontfamily")]
    #[serde(default = "default_font_family")]
    pub font_family: String,
    #[serde(rename = "halign")]
    #[serde(default = "HorizontalAlign::default")]
    pub h_align:     HorizontalAlign,
    #[serde(default = "utils::make_false")]
    pub italic:      bool,
    #[serde(default = "utils::make_true")]
    pub kerning:     bool,
    #[serde(rename = "pixelsize")]
    #[serde(default = "default_pixel_size")]
    pub pixel_size:  i32,
    #[serde(rename = "strikeout")]
    #[serde(default = "utils::make_false")]
    pub strike_out:  bool,
    pub text:        String,
    #[serde(default = "utils::make_false")]
    pub underline:   bool,
    #[serde(rename = "valign")]
    #[serde(default = "VerticalAlign::default")]
    pub v_align:     VerticalAlign,
    #[serde(default = "utils::make_false")]
    pub wrap:        bool,
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum HorizontalAlign {
    Center,
    Right,
    Justify,
    Left,
}

impl Default for HorizontalAlign {
    fn default() -> Self {
        HorizontalAlign::Left
    }
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum VerticalAlign {
    Center,
    Bottom,
    Top,
}

impl Default for VerticalAlign {
    fn default() -> Self {
        VerticalAlign::Top
    }
}

fn default_font_family() -> String {
    "sans-serif".to_owned()
}

fn default_pixel_size() -> i32 {
    16
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde_json::json;

    #[test]
    fn deserialize_terrain() {
        let actuals: Vec<Text> = serde_json::from_value(json! {
            [
                {
                    "bold":       false,
                    "color":      "#FFFFAABB",
                    "fontfamily": "Arial",
                    "halign":     "center",
                    "italic":     false,
                    "kerning":    false,
                    "pixelsize":  26,
                    "strikeout":  true,
                    "text":       "somebody",
                    "underline":  true,
                    "valign":     "center",
                    "wrap":       true
                },
                {
                    "bold":       false,
                    "color":      "#FFFFAABB",
                    "fontfamily": "Arial",
                    "halign":     "right",
                    "italic":     false,
                    "kerning":    false,
                    "pixelsize":  26,
                    "strikeout":  true,
                    "text":       "somebody",
                    "underline":  true,
                    "valign":     "bottom",
                    "wrap":       true
                },
                {
                    "bold":       false,
                    "color":      "#FFFFAABB",
                    "fontfamily": "Arial",
                    "halign":     "justify",
                    "italic":     false,
                    "kerning":    false,
                    "pixelsize":  26,
                    "strikeout":  true,
                    "text":       "somebody",
                    "underline":  true,
                    "valign":     "top",
                    "wrap":       true
                },
                {
                    "bold":       false,
                    "color":      "#FFFFAABB",
                    "fontfamily": "Arial",
                    "halign":     "left",
                    "italic":     false,
                    "kerning":    false,
                    "pixelsize":  26,
                    "strikeout":  true,
                    "text":       "somebody",
                    "underline":  true,
                    "valign":     "top",
                    "wrap":       true
                },
                {
                    "text":       "somebody"
                }
            ]
        })
        .unwrap();

        let expecteds: Vec<Text> = vec![
            Text {
                bold:        false,
                color:       Color::new(0xFF, 0xAA, 0xBB),
                font_family: "Arial".to_string(),
                h_align:     HorizontalAlign::Center,
                italic:      false,
                kerning:     false,
                pixel_size:  26,
                strike_out:  true,
                text:        "somebody".to_string(),
                underline:   true,
                v_align:     VerticalAlign::Center,
                wrap:        true,
            },
            Text {
                bold:        false,
                color:       Color::new(0xFF, 0xAA, 0xBB),
                font_family: "Arial".to_string(),
                h_align:     HorizontalAlign::Right,
                italic:      false,
                kerning:     false,
                pixel_size:  26,
                strike_out:  true,
                text:        "somebody".to_string(),
                underline:   true,
                v_align:     VerticalAlign::Bottom,
                wrap:        true,
            },
            Text {
                bold:        false,
                color:       Color::new(0xFF, 0xAA, 0xBB),
                font_family: "Arial".to_string(),
                h_align:     HorizontalAlign::Justify,
                italic:      false,
                kerning:     false,
                pixel_size:  26,
                strike_out:  true,
                text:        "somebody".to_string(),
                underline:   true,
                v_align:     VerticalAlign::Top,
                wrap:        true,
            },
            Text {
                bold:        false,
                color:       Color::new(0xFF, 0xAA, 0xBB),
                font_family: "Arial".to_string(),
                h_align:     HorizontalAlign::Left,
                italic:      false,
                kerning:     false,
                pixel_size:  26,
                strike_out:  true,
                text:        "somebody".to_string(),
                underline:   true,
                v_align:     VerticalAlign::Top,
                wrap:        true,
            },
            Text {
                bold:        false,
                color:       Color::new_black(),
                font_family: default_font_family(),
                h_align:     HorizontalAlign::default(),
                italic:      false,
                kerning:     true,
                pixel_size:  default_pixel_size(),
                strike_out:  false,
                text:        "somebody".to_string(),
                underline:   false,
                v_align:     VerticalAlign::default(),
                wrap:        false,
            },
        ];

        for (actual, expected) in actuals.into_iter().zip(expecteds) {
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn serialize_terrain() {
        let expecteds: Vec<String> = vec![
            json! {
                {
                    "bold":       false,
                    "color":      "#FFFFAABB",
                    "fontfamily": "Arial",
                    "halign":     "center",
                    "italic":     false,
                    "kerning":    false,
                    "pixelsize":  26,
                    "strikeout":  true,
                    "text":       "somebody",
                    "underline":  true,
                    "valign":     "center",
                    "wrap":       true
                }
            },
            json! {
                {
                    "bold":       false,
                    "color":      "#FFFFAABB",
                    "fontfamily": "Arial",
                    "halign":     "right",
                    "italic":     false,
                    "kerning":    false,
                    "pixelsize":  26,
                    "strikeout":  true,
                    "text":       "somebody",
                    "underline":  true,
                    "valign":     "bottom",
                    "wrap":       true
                }
            },
            json! {
                {
                    "bold":       false,
                    "color":      "#FFFFAABB",
                    "fontfamily": "Arial",
                    "halign":     "justify",
                    "italic":     false,
                    "kerning":    false,
                    "pixelsize":  26,
                    "strikeout":  true,
                    "text":       "somebody",
                    "underline":  true,
                    "valign":     "top",
                    "wrap":       true
                }
            },
            json! {
                {
                    "bold":       false,
                    "color":      "#FFFFAABB",
                    "fontfamily": "Arial",
                    "halign":     "left",
                    "italic":     false,
                    "kerning":    false,
                    "pixelsize":  26,
                    "strikeout":  true,
                    "text":       "somebody",
                    "underline":  true,
                    "valign":     "top",
                    "wrap":       true
                }
            },
            json! {
                {
                    "bold":       false,
                    "color":      "#FF000000",
                    "fontfamily": "sans-serif",
                    "halign":     "left",
                    "italic":     false,
                    "kerning":    true,
                    "pixelsize":  16,
                    "strikeout":  false,
                    "text":       "somebody",
                    "underline":  false,
                    "valign":     "top",
                    "wrap":       false
                }
            },
        ]
        .into_iter()
        .map(|v| serde_json::to_string(&v).unwrap())
        .collect();

        let actuals: Vec<String> = vec![
            Text {
                bold:        false,
                color:       Color::new(0xFF, 0xAA, 0xBB),
                font_family: "Arial".to_string(),
                h_align:     HorizontalAlign::Center,
                italic:      false,
                kerning:     false,
                pixel_size:  26,
                strike_out:  true,
                text:        "somebody".to_string(),
                underline:   true,
                v_align:     VerticalAlign::Center,
                wrap:        true,
            },
            Text {
                bold:        false,
                color:       Color::new(0xFF, 0xAA, 0xBB),
                font_family: "Arial".to_string(),
                h_align:     HorizontalAlign::Right,
                italic:      false,
                kerning:     false,
                pixel_size:  26,
                strike_out:  true,
                text:        "somebody".to_string(),
                underline:   true,
                v_align:     VerticalAlign::Bottom,
                wrap:        true,
            },
            Text {
                bold:        false,
                color:       Color::new(0xFF, 0xAA, 0xBB),
                font_family: "Arial".to_string(),
                h_align:     HorizontalAlign::Justify,
                italic:      false,
                kerning:     false,
                pixel_size:  26,
                strike_out:  true,
                text:        "somebody".to_string(),
                underline:   true,
                v_align:     VerticalAlign::Top,
                wrap:        true,
            },
            Text {
                bold:        false,
                color:       Color::new(0xFF, 0xAA, 0xBB),
                font_family: "Arial".to_string(),
                h_align:     HorizontalAlign::Left,
                italic:      false,
                kerning:     false,
                pixel_size:  26,
                strike_out:  true,
                text:        "somebody".to_string(),
                underline:   true,
                v_align:     VerticalAlign::Top,
                wrap:        true,
            },
            Text {
                bold:        false,
                color:       Color::new_black(),
                font_family: default_font_family(),
                h_align:     HorizontalAlign::default(),
                italic:      false,
                kerning:     true,
                pixel_size:  default_pixel_size(),
                strike_out:  false,
                text:        "somebody".to_string(),
                underline:   false,
                v_align:     VerticalAlign::default(),
                wrap:        false,
            },
        ]
        .into_iter()
        .map(|v| serde_json::to_string(&v).unwrap())
        .collect();

        for (actual, expected) in actuals.into_iter().zip(expecteds) {
            assert_eq!(actual, expected);
        }
    }
}
