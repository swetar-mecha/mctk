//! Dynamic styling of Components.
//!
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{Mutex, OnceLock};

use cosmic_text::Weight;

use crate::types::*;
use crate::{layout::*, size};

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct BorderWidth {
    pub top: f32,
    pub left: f32,
    pub bottom: f32,
    pub right: f32,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum VerticalPosition {
    Bottom,
    Center,
    Top,
}

impl Default for VerticalPosition {
    fn default() -> Self {
        Self::Bottom
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum HorizontalPosition {
    Left,
    Center,
    Right,
}

impl Default for HorizontalPosition {
    fn default() -> Self {
        Self::Right
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum FontWeight {
    Thin = 100,
    ExtraLight = 200,
    Light = 300,
    Normal = 400,
    Medium = 500,
    Semibold = 600,
    Bold = 700,
    ExtraBold = 800,
    Black = 900,
}

impl Default for FontWeight {
    fn default() -> Self {
        Self::Normal
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum StyleVal {
    Dimension(Dimension),
    Size(Size),
    Rect(Rect),
    Point(Point),
    Pos(Pos),
    Color(Color),
    Layout(Layout),
    HorizontalPosition(HorizontalPosition),
    VerticalPosition(VerticalPosition),
    BorderWidth(BorderWidth),
    FontWeight(FontWeight),
    Float(f64),
    Int(u32),
    Bool(bool),
    String(&'static str),
} // Impls below

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct StyleKey {
    struct_name: &'static str,
    parameter_name: &'static str,
    class: Option<&'static str>, // TODO should this be an array?
}

impl StyleKey {
    pub fn new(
        struct_name: &'static str,
        parameter_name: &'static str,
        class: Option<&'static str>,
    ) -> Self {
        Self {
            struct_name,
            parameter_name,
            class,
        }
    }
}

type StyleMap = HashMap<StyleKey, StyleVal>;
type StyleOverrideMap = HashMap<&'static str, StyleVal>;

/// A map between things to be styled ([`StyleKey`]s) and the style values ([`StyleVal`]s).
#[derive(Clone, Debug, PartialEq)]
pub struct Style(StyleMap);
#[derive(Clone, Default, Debug)]
pub struct StyleOverride(StyleOverrideMap);

impl Style {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add(mut self, k: StyleKey, v: StyleVal) -> Self {
        self.0.insert(k, v);
        self
    }

    pub fn get(&self, k: StyleKey) -> Option<StyleVal> {
        self.0.get(&k).cloned()
    }

    pub fn style(&self, component: &'static str, parameter_name: &'static str) -> Option<StyleVal> {
        let key = StyleKey {
            struct_name: component,
            parameter_name,
            class: None,
        };
        self.get(key)
    }

    pub fn style_for_class(
        &self,
        component: &'static str,
        parameter_name: &'static str,
        class: &'static str,
    ) -> Option<StyleVal> {
        let key = StyleKey {
            struct_name: component,
            parameter_name,
            class: Some(class),
        };
        self.get(key)
    }
}

impl Default for Style {
    fn default() -> Self {
        let map = StyleMap::from([
            // Button
            (
                StyleKey::new("Button", "text_color", None),
                Color::BLACK.into(),
            ),
            (
                StyleKey::new("Button", "text_color", Some("text-black")),
                Color::BLACK.into(),
            ),
            (
                StyleKey::new("Button", "text_color", Some("text-white")),
                Color::WHITE.into(),
            ),
            (
                StyleKey::new("Button", "text_color", Some("text-blue")),
                Color::BLUE.into(),
            ),
            (
                StyleKey::new("Button", "font", Some("font-space-mono")),
                "SpaceMono-Bold".into(),
            ),
            (
                StyleKey::new("Button", "font", Some("font-space-grotesk")),
                "Space Grotesk".into(),
            ),
            (StyleKey::new("Button", "font_size", None), 12.0.into()),
            (
                StyleKey::new("Button", "font_size", Some("text-xs")),
                14.0.into(),
            ),
            (
                StyleKey::new("Button", "font_size", Some("text-sm")),
                16.0.into(),
            ),
            (
                StyleKey::new("Button", "font_size", Some("text-md")),
                18.0.into(),
            ),
            (
                StyleKey::new("Button", "font_size", Some("text-l")),
                20.0.into(),
            ),
            (
                StyleKey::new("Button", "font_size", Some("text-xl")),
                22.0.into(),
            ),
            (
                StyleKey::new("Button", "font_size", Some("text-2xl")),
                24.0.into(),
            ),
            (
                StyleKey::new("Button", "font_size", Some("text-3xl")),
                28.0.into(),
            ),
            (
                StyleKey::new("Button", "font_weight", None),
                FontWeight::Normal.into(),
            ),
            (
                StyleKey::new("Button", "font_weight", Some("font-thin")),
                FontWeight::Thin.into(),
            ),
            (
                StyleKey::new("Button", "font_weight", Some("font-extralight")),
                FontWeight::ExtraLight.into(),
            ),
            (
                StyleKey::new("Button", "font_weight", Some("font-light")),
                FontWeight::Light.into(),
            ),
            (
                StyleKey::new("Button", "font_weight", Some("font-normal")),
                FontWeight::Normal.into(),
            ),
            (
                StyleKey::new("Button", "font_weight", Some("font-medium")),
                FontWeight::Medium.into(),
            ),
            (
                StyleKey::new("Button", "font_weight", Some("font-semibold")),
                FontWeight::Semibold.into(),
            ),
            (
                StyleKey::new("Button", "font_weight", Some("font-bold")),
                FontWeight::Bold.into(),
            ),
            (
                StyleKey::new("Button", "font_weight", Some("font-extrabold")),
                FontWeight::ExtraBold.into(),
            ),
            (
                StyleKey::new("Button", "font_weight", Some("font-black")),
                FontWeight::Black.into(),
            ),
            (
                StyleKey::new("Button", "background_color", None),
                Color::WHITE.into(),
            ),
            (
                StyleKey::new("Button", "background_color", Some("bg-black")),
                Color::BLACK.into(),
            ),
            (
                StyleKey::new("Button", "background_color", Some("bg-white")),
                Color::WHITE.into(),
            ),
            (
                StyleKey::new("Button", "background_color", Some("bg-transparent")),
                Color::TRANSPARENT.into(),
            ),
            (
                StyleKey::new("Button", "highlight_color", None),
                Color::LIGHT_GREY.into(),
            ),
            (
                StyleKey::new("Button", "active_color", None),
                Color::MID_GREY.into(),
            ),
            (
                StyleKey::new("Button", "border_color", None),
                Color::BLACK.into(),
            ),
            (StyleKey::new("Button", "border_width", None), 0.0.into()),
            (
                StyleKey::new("Button", "border_width", Some("border-0")),
                0.0.into(),
            ),
            (
                StyleKey::new("Button", "border_width", Some("border")),
                1.0.into(),
            ),
            (
                StyleKey::new("Button", "border_width", Some("border-2")),
                2.0.into(),
            ),
            (
                StyleKey::new("Button", "border_width", Some("border-4")),
                4.0.into(),
            ),
            (
                StyleKey::new("Button", "border_width", Some("border-8")),
                8.0.into(),
            ),
            (
                StyleKey::new("Button", "border_width", Some("border-16")),
                16.0.into(),
            ),
            (
                StyleKey::new("Button", "border_width", Some("border-0")),
                0.0.into(),
            ),
            (StyleKey::new("Button", "radius", None), 0.0.into()),
            (
                StyleKey::new("IconButton", "radius", Some("rounded-sm")),
                2.0.into(),
            ),
            (
                StyleKey::new("IconButton", "radius", Some("rounded")),
                4.0.into(),
            ),
            (
                StyleKey::new("IconButton", "radius", Some("rounded-md")),
                6.0.into(),
            ),
            (
                StyleKey::new("IconButton", "radius", Some("rounded-lg")),
                8.0.into(),
            ),
            (
                StyleKey::new("IconButton", "radius", Some("rounded-xl")),
                12.0.into(),
            ),
            (
                StyleKey::new("IconButton", "radius", Some("rounded-2xl")),
                16.0.into(),
            ),
            (
                StyleKey::new("IconButton", "radius", Some("rounded-3xl")),
                24.0.into(),
            ),
            (StyleKey::new("Button", "padding", None), 2.0.into()),
            (
                StyleKey::new("Button", "h_alignment", None),
                HorizontalPosition::Center.into(),
            ),
            (
                StyleKey::new("Button", "line_height", Some("leading-3")),
                12.0.into(),
            ),
            (
                StyleKey::new("Button", "line_height", Some("leading-4")),
                16.0.into(),
            ),
            (
                StyleKey::new("Button", "line_height", Some("leading-5")),
                20.0.into(),
            ),
            (
                StyleKey::new("Button", "line_height", Some("leading-6")),
                24.0.into(),
            ),
            (
                StyleKey::new("Button", "line_height", Some("leading-7")),
                28.0.into(),
            ),
            (
                StyleKey::new("Button", "line_height", Some("leading-8")),
                32.0.into(),
            ),
            (
                StyleKey::new("Button", "line_height", Some("leading-9")),
                36.0.into(),
            ),
            (
                StyleKey::new("Button", "line_height", Some("leading-10")),
                40.0.into(),
            ),
            (StyleKey::new("Button", "padding", Some("p-0")), 0.0.into()),
            (StyleKey::new("Button", "padding", Some("p-1")), 4.0.into()),
            (StyleKey::new("Button", "padding", Some("p-2")), 8.0.into()),
            (StyleKey::new("Button", "padding", Some("p-3")), 12.0.into()),
            (StyleKey::new("Button", "padding", Some("p-4")), 16.0.into()),
            (StyleKey::new("Button", "padding", Some("p-5")), 20.0.into()),
            (StyleKey::new("Button", "padding", Some("p-6")), 24.0.into()),
            (StyleKey::new("Button", "padding", Some("p-7")), 28.0.into()),
            (StyleKey::new("Button", "padding", Some("p-8")), 32.0.into()),
            (StyleKey::new("Button", "padding", Some("p-9")), 36.0.into()),
            // IconButton
            (
                StyleKey::new("IconButton", "size", None),
                size!(18., 18.).into(),
            ),
            (
                StyleKey::new("IconButton", "size", Some("btn-xs")),
                size!(18., 18.).into(),
            ),
            (
                StyleKey::new("IconButton", "size", Some("btn-sm")),
                size!(24.).into(),
            ),
            (
                StyleKey::new("IconButton", "size", Some("btn-md")),
                size!(32.).into(),
            ),
            (
                StyleKey::new("IconButton", "size", Some("btn-xl")),
                size!(48.0).into(),
            ),
            (
                StyleKey::new("IconButton", "size", Some("btn-xxl")),
                size!(88.).into(),
            ),
            (
                StyleKey::new("IconButton", "text_color", None),
                Color::BLACK.into(),
            ),
            (StyleKey::new("IconButton", "font_size", None), 12.0.into()),
            (
                StyleKey::new("IconButton", "background_color", None),
                Color::BLACK.into(),
            ),
            (
                StyleKey::new("IconButton", "background_color", Some("light")),
                Color::DARK_GREY.into(),
            ),
            (
                StyleKey::new("IconButton", "background_color", Some("bg-white")),
                Color::WHITE.into(),
            ),
            (
                StyleKey::new("IconButton", "background_color", Some("bg-transparent")),
                Color::TRANSPARENT.into(),
            ),
            (
                StyleKey::new("IconButton", "highlight_color", None),
                Color::LIGHT_GREY.into(),
            ),
            (
                StyleKey::new("IconButton", "active_color", None),
                Color::rgb(132., 132., 132.).into(),
            ),
            (
                StyleKey::new("IconButton", "border_color", None),
                Color::rgb(132., 132., 132.).into(),
            ),
            (
                StyleKey::new("IconButton", "border_width", None),
                0.0.into(),
            ),
            (
                StyleKey::new("IconButton", "border_width", Some("border-0")),
                0.0.into(),
            ),
            (
                StyleKey::new("IconButton", "border_width", Some("border")),
                1.0.into(),
            ),
            (
                StyleKey::new("IconButton", "border_width", Some("border-2")),
                2.0.into(),
            ),
            (
                StyleKey::new("IconButton", "border_width", Some("border-4")),
                4.0.into(),
            ),
            (
                StyleKey::new("IconButton", "border_width", Some("border-8")),
                8.0.into(),
            ),
            (
                StyleKey::new("IconButton", "border_width", Some("border-16")),
                16.0.into(),
            ),
            (
                StyleKey::new("IconButton", "border_width", Some("border-0")),
                0.0.into(),
            ),
            (
                StyleKey::new("IconButton", "padding", Some("p-0")),
                0.0.into(),
            ),
            (
                StyleKey::new("IconButton", "padding", Some("p-1")),
                4.0.into(),
            ),
            (
                StyleKey::new("IconButton", "padding", Some("p-2")),
                8.0.into(),
            ),
            (
                StyleKey::new("IconButton", "padding", Some("p-3")),
                12.0.into(),
            ),
            (
                StyleKey::new("IconButton", "padding", Some("p-4")),
                16.0.into(),
            ),
            (
                StyleKey::new("IconButton", "padding", Some("p-5")),
                20.0.into(),
            ),
            (
                StyleKey::new("IconButton", "padding", Some("p-6")),
                24.0.into(),
            ),
            (
                StyleKey::new("IconButton", "padding", Some("p-7")),
                28.0.into(),
            ),
            (
                StyleKey::new("IconButton", "padding", Some("p-8")),
                32.0.into(),
            ),
            (
                StyleKey::new("IconButton", "padding", Some("p-9")),
                36.0.into(),
            ),
            (StyleKey::new("IconButton", "radius", None), 0.0.into()),
            (
                StyleKey::new("IconButton", "radius", Some("rounded-sm")),
                2.0.into(),
            ),
            (
                StyleKey::new("IconButton", "radius", Some("rounded")),
                4.0.into(),
            ),
            (
                StyleKey::new("IconButton", "radius", Some("rounded-md")),
                6.0.into(),
            ),
            (
                StyleKey::new("IconButton", "radius", Some("rounded-lg")),
                8.0.into(),
            ),
            (
                StyleKey::new("IconButton", "radius", Some("rounded-xl")),
                12.0.into(),
            ),
            (
                StyleKey::new("IconButton", "radius", Some("rounded-2xl")),
                16.0.into(),
            ),
            (
                StyleKey::new("IconButton", "radius", Some("rounded-3xl")),
                24.0.into(),
            ),
            (StyleKey::new("IconButton", "padding", None), 10.0.into()),
            // RadioButton
            (
                StyleKey::new("RadioButton", "text_color", None),
                Color::BLACK.into(),
            ),
            (
                StyleKey::new("RadioButton", "font_size", None),
                Color::BLACK.into(),
            ),
            (
                StyleKey::new("RadioButton", "background_color", None),
                Color::TRANSPARENT.into(),
            ),
            (
                StyleKey::new("RadioButton", "highlight_color", None),
                Color::LIGHT_GREY.into(),
            ),
            (
                StyleKey::new("RadioButton", "active_color", None),
                Color::rgb(45., 138., 255.).into(),
            ),
            (
                StyleKey::new("RadioButton", "border_color", None),
                Color::rgb(83., 83., 83.).into(),
            ),
            (
                StyleKey::new("RadioButton", "border_width", None),
                2.0.into(),
            ),
            (StyleKey::new("RadioButton", "radius", None), 4.0.into()),
            (StyleKey::new("RadioButton", "padding", None), 2.0.into()),
            // Select
            (
                StyleKey::new("Select", "text_color", None),
                Color::BLACK.into(),
            ),
            (StyleKey::new("Select", "font_size", None), 12.0.into()),
            (
                StyleKey::new("Select", "background_color", None),
                Color::WHITE.into(),
            ),
            (
                StyleKey::new("Select", "highlight_color", None),
                Color::LIGHT_GREY.into(),
            ),
            (
                StyleKey::new("Select", "border_color", None),
                Color::BLACK.into(),
            ),
            (
                StyleKey::new("Select", "caret_color", None),
                Color::BLACK.into(),
            ),
            (StyleKey::new("Select", "border_width", None), 2.0.into()),
            (StyleKey::new("Select", "radius", None), 4.0.into()),
            (StyleKey::new("Select", "padding", None), 2.0.into()),
            (StyleKey::new("Select", "max_height", None), 250.0.into()),
            // Toggle
            (
                StyleKey::new("Toggle", "background_color", None),
                Color::LIGHT_GREY.into(),
            ),
            (
                StyleKey::new("Toggle", "highlight_color", None),
                Color::DARK_GREY.into(),
            ),
            (
                StyleKey::new("Toggle", "active_color", None),
                Color::MID_GREY.into(),
            ),
            (
                StyleKey::new("Toggle", "border_color", None),
                Color::BLACK.into(),
            ),
            (StyleKey::new("Toggle", "border_width", None), 2.0.into()),
            // ToolTip
            (
                StyleKey::new("ToolTip", "text_color", None),
                Color::BLACK.into(),
            ),
            (StyleKey::new("ToolTip", "font_size", None), 12.0.into()),
            (
                StyleKey::new("ToolTip", "background_color", None),
                Color::WHITE.into(),
            ),
            (
                StyleKey::new("ToolTip", "border_color", None),
                Color::BLACK.into(),
            ),
            (StyleKey::new("ToolTip", "border_width", None), 2.0.into()),
            (StyleKey::new("ToolTip", "padding", None), 4.0.into()),
            // TextBox
            (StyleKey::new("TextBox", "font_size", None), 12.0.into()),
            (
                StyleKey::new("TextBox", "font_size", Some("text-xs")),
                14.0.into(),
            ),
            (
                StyleKey::new("TextBox", "font_size", Some("text-sm")),
                16.0.into(),
            ),
            (
                StyleKey::new("TextBox", "font_size", Some("text-md")),
                18.0.into(),
            ),
            (
                StyleKey::new("TextBox", "font_size", Some("text-l")),
                20.0.into(),
            ),
            (
                StyleKey::new("TextBox", "font_size", Some("text-xl")),
                22.0.into(),
            ),
            (
                StyleKey::new("TextBox", "font_size", Some("text-2xl")),
                24.0.into(),
            ),
            (
                StyleKey::new("TextBox", "text_color", None),
                Color::WHITE.into(),
            ),
            (
                StyleKey::new("TextBox", "text_color", Some("light")),
                Color::rgb(14., 14., 14.).into(),
            ),
            (
                StyleKey::new("TextBox", "placeholder_color", None),
                Color::rgb(132., 132., 132.).into(),
            ),
            (
                StyleKey::new("TextBox", "placeholder_color", Some("light")),
                Color::rgb(148., 148., 148.).into(),
            ),
            (
                StyleKey::new("TextBox", "background_color", None),
                Color::rgb(49., 49., 49.).into(),
            ),
            (
                StyleKey::new("TextBox", "background_color", Some("light")),
                Color::WHITE.into(),
            ),
            (
                StyleKey::new("TextBox", "background_color", Some("bg-transparent")),
                Color::TRANSPARENT.into(),
            ),
            (
                StyleKey::new("TextBox", "selection_color", None),
                Color::MID_GREY.into(),
            ),
            (
                StyleKey::new("TextBox", "cursor_color", None),
                Color::WHITE.into(),
            ),
            (
                StyleKey::new("TextBox", "cursor_color", Some("light")),
                Color::BLACK.into(),
            ),
            (
                StyleKey::new("TextBox", "border_color", None),
                Color::rgb(132., 132., 132.).into(),
            ),
            (
                StyleKey::new("TextBox", "border_color", Some("light")),
                Color::rgb(209., 209., 209.).into(),
            ),
            (
                StyleKey::new("TextBox", "border_width", None),
                BorderWidth {
                    top: 1.,
                    left: 1.,
                    bottom: 1.,
                    right: 1.,
                }
                .into(),
            ),
            (
                StyleKey::new("TextBox", "border_width", Some("border-0")),
                BorderWidth {
                    top: 0.0,
                    left: 0.0,
                    bottom: 0.0,
                    right: 0.0,
                }
                .into(),
            ),
            (
                StyleKey::new("TextBox", "border_width", Some("border")),
                BorderWidth {
                    top: 1.0,
                    left: 1.0,
                    bottom: 1.0,
                    right: 1.0,
                }
                .into(),
            ),
            (
                StyleKey::new("TextBox", "border_width", Some("border-2")),
                BorderWidth {
                    top: 2.0,
                    left: 2.0,
                    bottom: 2.0,
                    right: 2.0,
                }
                .into(),
            ),
            (
                StyleKey::new("TextBox", "border_width", Some("border-4")),
                BorderWidth {
                    top: 4.0,
                    left: 4.0,
                    bottom: 4.0,
                    right: 4.0,
                }
                .into(),
            ),
            (
                StyleKey::new("TextBox", "border_width", Some("border-8")),
                BorderWidth {
                    top: 8.0,
                    left: 8.0,
                    bottom: 8.0,
                    right: 8.0,
                }
                .into(),
            ),
            (
                StyleKey::new("TextBox", "border_width", Some("border-x-0")),
                BorderWidth {
                    left: 0.0,
                    right: 0.0,
                    top: 1.5,
                    bottom: 1.5,
                }
                .into(),
            ),
            (
                StyleKey::new("TextBox", "border_width", Some("border-x-2")),
                BorderWidth {
                    left: 2.0,
                    right: 2.0,
                    top: 1.5,
                    bottom: 1.5,
                }
                .into(),
            ),
            (
                StyleKey::new("TextBox", "border_width", Some("border-x-4")),
                BorderWidth {
                    left: 4.0,
                    right: 4.0,
                    top: 1.5,
                    bottom: 1.5,
                }
                .into(),
            ),
            (
                StyleKey::new("TextBox", "border_width", Some("border-x-8")),
                BorderWidth {
                    left: 8.0,
                    right: 8.0,
                    top: 1.5,
                    bottom: 1.5,
                }
                .into(),
            ),
            (
                StyleKey::new("TextBox", "border_width", Some("border-y-0")),
                BorderWidth {
                    top: 0.0,
                    bottom: 0.0,
                    left: 1.5,
                    right: 1.5,
                }
                .into(),
            ),
            (
                StyleKey::new("TextBox", "border_width", Some("border-y-2")),
                BorderWidth {
                    top: 2.0,
                    bottom: 2.0,
                    left: 1.5,
                    right: 1.5,
                }
                .into(),
            ),
            (
                StyleKey::new("TextBox", "border_width", Some("border-y-4")),
                BorderWidth {
                    top: 4.0,
                    bottom: 4.0,
                    left: 1.5,
                    right: 1.5,
                }
                .into(),
            ),
            (
                StyleKey::new("TextBox", "border_width", Some("border-y-8")),
                BorderWidth {
                    top: 8.0,
                    bottom: 8.0,
                    left: 1.5,
                    right: 1.5,
                }
                .into(),
            ),
            (
                StyleKey::new("TextBox", "border_width", Some("border-t-0")),
                BorderWidth {
                    top: 0.0,
                    left: 1.5,
                    right: 1.5,
                    bottom: 1.5,
                }
                .into(),
            ),
            (
                StyleKey::new("TextBox", "border_width", Some("border-t-2")),
                BorderWidth {
                    top: 2.0,
                    left: 1.5,
                    right: 1.5,
                    bottom: 1.5,
                }
                .into(),
            ),
            (
                StyleKey::new("TextBox", "border_width", Some("border-b-0")),
                BorderWidth {
                    bottom: 0.0,
                    top: 1.5,
                    left: 1.5,
                    right: 1.5,
                }
                .into(),
            ),
            (
                StyleKey::new("TextBox", "border_width", Some("border-b-2")),
                BorderWidth {
                    bottom: 2.0,
                    top: 1.5,
                    left: 1.5,
                    right: 1.5,
                }
                .into(),
            ),
            (
                StyleKey::new("TextBox", "border_width", Some("border-l-0")),
                BorderWidth {
                    left: 0.0,
                    top: 1.5,
                    right: 1.5,
                    bottom: 1.5,
                }
                .into(),
            ),
            (
                StyleKey::new("TextBox", "border_width", Some("border-l-2")),
                BorderWidth {
                    left: 2.0,
                    top: 1.5,
                    right: 1.5,
                    bottom: 1.5,
                }
                .into(),
            ),
            (
                StyleKey::new("TextBox", "border_width", Some("border-r-0")),
                BorderWidth {
                    right: 0.0,
                    top: 1.5,
                    left: 1.5,
                    bottom: 1.5,
                }
                .into(),
            ),
            (
                StyleKey::new("TextBox", "border_width", Some("border-r-2")),
                BorderWidth {
                    right: 2.0,
                    top: 1.5,
                    left: 1.5,
                    bottom: 1.5,
                }
                .into(),
            ),
            (StyleKey::new("TextBox", "padding", None), 1.0.into()),
            (
                StyleKey::new("TextBox", "font_weight", None),
                FontWeight::Normal.into(),
            ),
            // Text
            (StyleKey::new("Text", "size", None), 12.0.into()),
            (StyleKey::new("Text", "size", Some("text-xs")), 14.0.into()),
            (StyleKey::new("Text", "size", Some("text-sm")), 16.0.into()),
            (StyleKey::new("Text", "size", Some("text-md")), 18.0.into()),
            (StyleKey::new("Text", "size", Some("text-l")), 20.0.into()),
            (StyleKey::new("Text", "size", Some("text-xl")), 22.0.into()),
            (StyleKey::new("Text", "size", Some("text-2xl")), 24.0.into()),
            (StyleKey::new("Text", "size", Some("text-3xl")), 28.0.into()),
            (
                StyleKey::new("Text", "font", Some("font-space-mono")),
                "SpaceMono-Bold".into(),
            ),
            (
                StyleKey::new("Text", "font", Some("font-space-grotesk")),
                "Space Grotesk".into(),
            ),
            (
                StyleKey::new("Text", "font_weight", None),
                FontWeight::Normal.into(),
            ),
            (
                StyleKey::new("Text", "font_weight", Some("font-thin")),
                FontWeight::Thin.into(),
            ),
            (
                StyleKey::new("Text", "font_weight", Some("font-extralight")),
                FontWeight::ExtraLight.into(),
            ),
            (
                StyleKey::new("Text", "font_weight", Some("font-light")),
                FontWeight::Light.into(),
            ),
            (
                StyleKey::new("Text", "font_weight", Some("font-normal")),
                FontWeight::Normal.into(),
            ),
            (
                StyleKey::new("Text", "font_weight", Some("font-medium")),
                FontWeight::Medium.into(),
            ),
            (
                StyleKey::new("Text", "font_weight", Some("font-semibold")),
                FontWeight::Semibold.into(),
            ),
            (
                StyleKey::new("Text", "font_weight", Some("font-bold")),
                FontWeight::Bold.into(),
            ),
            (
                StyleKey::new("Text", "font_weight", Some("font-extrabold")),
                FontWeight::ExtraBold.into(),
            ),
            (
                StyleKey::new("Text", "font_weight", Some("font-black")),
                FontWeight::Black.into(),
            ),
            (StyleKey::new("Text", "color", None), Color::BLACK.into()),
            (
                StyleKey::new("Text", "color", Some("light")),
                Color::WHITE.into(),
            ),
            (
                StyleKey::new("Text", "color", Some("text-black")),
                Color::BLACK.into(),
            ),
            (
                StyleKey::new("Text", "color", Some("text-white")),
                Color::WHITE.into(),
            ),
            (
                StyleKey::new("Text", "color", Some("text-blue")),
                Color::BLUE.into(),
            ),
            (
                StyleKey::new("Text", "h_alignment", None),
                HorizontalPosition::Left.into(),
            ),
            (
                StyleKey::new("Text", "line_height", Some("leading-3")),
                12.0.into(),
            ),
            (
                StyleKey::new("Text", "line_height", Some("leading-4")),
                16.0.into(),
            ),
            (
                StyleKey::new("Text", "line_height", Some("leading-5")),
                20.0.into(),
            ),
            (
                StyleKey::new("Text", "line_height", Some("leading-6")),
                24.0.into(),
            ),
            (
                StyleKey::new("Text", "line_height", Some("leading-7")),
                28.0.into(),
            ),
            (
                StyleKey::new("Text", "line_height", Some("leading-8")),
                32.0.into(),
            ),
            (
                StyleKey::new("Text", "line_height", Some("leading-9")),
                36.0.into(),
            ),
            (
                StyleKey::new("Text", "line_height", Some("leading-10")),
                40.0.into(),
            ),
            (
                StyleKey::new("Text", "line_height", Some("leading-none")),
                1.0.into(),
            ),
            (
                StyleKey::new("Text", "line_height", Some("leading-tight")),
                1.25.into(),
            ),
            (
                StyleKey::new("Text", "line_height", Some("leading-snug")),
                1.375.into(),
            ),
            (
                StyleKey::new("Text", "line_height", Some("leading-normal")),
                1.5.into(),
            ),
            (
                StyleKey::new("Text", "line_height", Some("leading-relaxed")),
                1.625.into(),
            ),
            (
                StyleKey::new("Text", "line_height", Some("leading-loose")),
                2.0.into(),
            ),
            // Scroll
            (StyleKey::new("Scroll", "x", None), false.into()),
            (StyleKey::new("Scroll", "y", None), false.into()),
            (
                StyleKey::new("Scroll", "x_bar_position", None),
                VerticalPosition::Bottom.into(),
            ),
            (
                StyleKey::new("Scroll", "y_bar_position", None),
                HorizontalPosition::Right.into(),
            ),
            (StyleKey::new("Scroll", "bar_width", None), 12.0.into()),
            (
                StyleKey::new("Scroll", "bar_background_color", None),
                Color::LIGHT_GREY.into(),
            ),
            (
                StyleKey::new("Scroll", "bar_color", None),
                Into::<Color>::into(0.7).into(),
            ),
            (
                StyleKey::new("Scroll", "bar_highlight_color", None),
                Into::<Color>::into(0.5).into(),
            ),
            (
                StyleKey::new("Scroll", "bar_active_color", None),
                Color::DARK_GREY.into(),
            ),
            //Image
            (StyleKey::new("Image", "radius", None), 0.0.into()),
        ]);
        Self(map)
    }
}

fn _current_style() -> &'static Mutex<Style> {
    static CURRENT_STYLE: OnceLock<Mutex<Style>> = OnceLock::new();
    CURRENT_STYLE.get_or_init(|| Mutex::new(Style::new()))
}

pub fn set_current_style(s: Style) {
    *_current_style().lock().unwrap() = s;
}

pub fn current_style(component: &'static str, parameter_name: &'static str) -> Option<StyleVal> {
    _current_style()
        .lock()
        .unwrap()
        .style(component, parameter_name)
}

fn get_current_style(k: StyleKey) -> Option<StyleVal> {
    _current_style().lock().unwrap().get(k)
}

/// Implemented by the [`component`][macro@crate::component] attribute macro, for "Styled" Components.
pub trait Styled: Sized {
    #[doc(hidden)]
    fn name() -> &'static str;
    #[doc(hidden)]
    fn class(&self) -> Option<&'static str>;
    #[doc(hidden)]
    fn class_mut(&mut self) -> &mut Option<&'static str>;
    #[doc(hidden)]
    fn style_overrides(&self) -> &StyleOverride;
    #[doc(hidden)]
    fn style_overrides_mut(&mut self) -> &mut StyleOverride;

    fn with_class(mut self, class: &'static str) -> Self {
        *self.class_mut() = Some(class);
        self
    }

    fn style<V: Into<StyleVal>>(mut self, parameter: &'static str, val: V) -> Self {
        self.style_overrides_mut().0.insert(parameter, val.into());
        self
    }

    fn maybe_style(mut self, parameter: &'static str, val: Option<StyleVal>) -> Self {
        if let Some(val) = val {
            self.style_overrides_mut().0.insert(parameter, val);
        }
        self
    }

    #[doc(hidden)]
    fn style_key(&self, parameter_name: &'static str, class: Option<&'static str>) -> StyleKey {
        StyleKey {
            struct_name: Self::name(),
            parameter_name,
            class,
        }
    }

    fn style_val(&self, param: &'static str) -> Option<StyleVal> {
        if let Some(v) = self.style_overrides().0.get(param) {
            Some(v.clone())
        } else if let Some(c) = self.class() {
            // println!("param {:?} class {:?}", param, c);
            for c in c.split(" ").collect::<Vec<&str>>() {
                if let Some(v) = get_current_style(self.style_key(param, Some(c))) {
                    return Some(v);
                }
            }
            get_current_style(self.style_key(param, None))
        } else {
            get_current_style(self.style_key(param, None))
        }
    }
}

#[macro_export]
macro_rules! style {
    // Widget.color = Color::WHITE;
    // ->
    // Style::new().add(StyleKey::new("Widget", "color", None), Color::WHITE.into())

    // class.Widget.color = Color::BLACK;
    // ->
    // Style::new().add(StyleKey::new("Widget", "color", Some("class")), Color::BLACK.into())

    //Finish it
    ( @ { } -> ($($result:tt)*) ) => (
        $crate::style::Style::new() $($result)*
    );


    ( @ { $component:ident . $param:ident = $val:expr ; $($rest:tt)* } -> ($($result:tt)*) ) => (
        style!(@ { $($rest)* } -> (
            $($result)*
            .add($crate::style::StyleKey::new(stringify!($component), stringify!($param), None), $val.into())
        ))
    );

    ( @ { $class:ident . $component:ident . $param:ident = $val:expr ; $($rest:tt)* } -> ($($result:tt)*) ) => (
        style!(@ { $($rest)* } -> (
            $($result)*
            .add($crate::style::StyleKey::new(stringify!($component), stringify!($param), Some(stringify!($class))), $val.into())
        ))
    );

    // Entry point
    ( $( $tt:tt )* ) => (
        style!(@ { $($tt)* } -> ())
    );

}

// TODO we need some way to add more context to these errors, or otherwise prevent them from happening.
// Right now, if you add the wrong type expected for a given style, the error message is terrible.

//   Froms
impl From<BorderWidth> for StyleVal {
    fn from(bw: BorderWidth) -> Self {
        Self::BorderWidth(bw)
    }
}
impl From<StyleVal> for BorderWidth {
    fn from(v: StyleVal) -> Self {
        match v {
            StyleVal::BorderWidth(c) => c,
            x => panic!("Tried to coerce {x:?} into a border width"),
        }
    }
}
impl From<Option<StyleVal>> for BorderWidth {
    fn from(v: Option<StyleVal>) -> Self {
        match v {
            Some(StyleVal::BorderWidth(c)) => c,
            x => panic!("Tried to coerce {x:?} into a border width"),
        }
    }
}

impl From<Color> for StyleVal {
    fn from(c: Color) -> Self {
        Self::Color(c)
    }
}
impl From<StyleVal> for Color {
    fn from(v: StyleVal) -> Self {
        match v {
            StyleVal::Color(c) => c,
            x => panic!("Tried to coerce {x:?} into a Color"),
        }
    }
}
impl From<Option<StyleVal>> for Color {
    fn from(v: Option<StyleVal>) -> Self {
        match v {
            Some(StyleVal::Color(c)) => c,
            x => panic!("Tried to coerce {x:?} into a Color"),
        }
    }
}
impl From<Dimension> for StyleVal {
    fn from(c: Dimension) -> Self {
        Self::Dimension(c)
    }
}
impl From<StyleVal> for Dimension {
    fn from(v: StyleVal) -> Self {
        match v {
            StyleVal::Dimension(c) => c,
            x => panic!("Tried to coerce {x:?} into a Dimension"),
        }
    }
}
impl From<Option<StyleVal>> for Dimension {
    fn from(v: Option<StyleVal>) -> Self {
        match v {
            Some(StyleVal::Dimension(c)) => c,
            x => panic!("Tried to coerce {x:?} into a Dimension"),
        }
    }
}
impl From<Size> for StyleVal {
    fn from(c: Size) -> Self {
        Self::Size(c)
    }
}
impl From<StyleVal> for Size {
    fn from(v: StyleVal) -> Self {
        match v {
            StyleVal::Size(c) => c,
            x => panic!("Tried to coerce {x:?} into a Size"),
        }
    }
}
impl From<Option<StyleVal>> for Size {
    fn from(v: Option<StyleVal>) -> Self {
        match v {
            Some(StyleVal::Size(c)) => c,
            x => panic!("Tried to coerce {x:?} into a Size"),
        }
    }
}
impl From<Pos> for StyleVal {
    fn from(c: Pos) -> Self {
        Self::Pos(c)
    }
}
impl From<StyleVal> for Pos {
    fn from(v: StyleVal) -> Self {
        match v {
            StyleVal::Pos(c) => c,
            x => panic!("Tried to coerce {x:?} into a Pos"),
        }
    }
}
impl From<Option<StyleVal>> for Pos {
    fn from(v: Option<StyleVal>) -> Self {
        match v {
            Some(StyleVal::Pos(c)) => c,
            x => panic!("Tried to coerce {x:?} into a Pos"),
        }
    }
}
impl From<Point> for StyleVal {
    fn from(c: Point) -> Self {
        Self::Point(c)
    }
}
impl From<StyleVal> for Point {
    fn from(v: StyleVal) -> Self {
        match v {
            StyleVal::Point(c) => c,
            x => panic!("Tried to coerce {x:?} into a Point"),
        }
    }
}
impl From<Option<StyleVal>> for Point {
    fn from(v: Option<StyleVal>) -> Self {
        match v {
            Some(StyleVal::Point(c)) => c,
            x => panic!("Tried to coerce {x:?} into a Point"),
        }
    }
}
impl From<Rect> for StyleVal {
    fn from(c: Rect) -> Self {
        Self::Rect(c)
    }
}
impl From<StyleVal> for Rect {
    fn from(v: StyleVal) -> Self {
        match v {
            StyleVal::Rect(c) => c,
            x => panic!("Tried to coerce {x:?} into a Rect"),
        }
    }
}
impl From<Option<StyleVal>> for Rect {
    fn from(v: Option<StyleVal>) -> Self {
        match v {
            Some(StyleVal::Rect(c)) => c,
            x => panic!("Tried to coerce {x:?} into a Rect"),
        }
    }
}
impl From<Layout> for StyleVal {
    fn from(c: Layout) -> Self {
        Self::Layout(c)
    }
}
impl From<StyleVal> for Layout {
    fn from(v: StyleVal) -> Self {
        match v {
            StyleVal::Layout(c) => c,
            x => panic!("Tried to coerce {x:?} into a Layout"),
        }
    }
}
impl From<Option<StyleVal>> for Layout {
    fn from(v: Option<StyleVal>) -> Self {
        match v {
            Some(StyleVal::Layout(c)) => c,
            x => panic!("Tried to coerce {x:?} into a Layout"),
        }
    }
}
impl From<VerticalPosition> for StyleVal {
    fn from(c: VerticalPosition) -> Self {
        Self::VerticalPosition(c)
    }
}
impl From<StyleVal> for VerticalPosition {
    fn from(v: StyleVal) -> Self {
        match v {
            StyleVal::VerticalPosition(c) => c,
            x => panic!("Tried to coerce {x:?} into a VerticalPosition"),
        }
    }
}
impl From<Option<StyleVal>> for VerticalPosition {
    fn from(v: Option<StyleVal>) -> Self {
        match v {
            Some(StyleVal::VerticalPosition(c)) => c,
            x => panic!("Tried to coerce {x:?} into a VerticalPosition"),
        }
    }
}
impl From<HorizontalPosition> for StyleVal {
    fn from(c: HorizontalPosition) -> Self {
        Self::HorizontalPosition(c)
    }
}
impl From<StyleVal> for HorizontalPosition {
    fn from(v: StyleVal) -> Self {
        match v {
            StyleVal::HorizontalPosition(c) => c,
            x => panic!("Tried to coerce {x:?} into a HorizontalPosition"),
        }
    }
}
impl From<Option<StyleVal>> for HorizontalPosition {
    fn from(v: Option<StyleVal>) -> Self {
        match v {
            Some(StyleVal::HorizontalPosition(c)) => c,
            x => panic!("Tried to coerce {x:?} into a HorizontalPosition"),
        }
    }
}
impl From<FontWeight> for StyleVal {
    fn from(c: FontWeight) -> Self {
        Self::FontWeight(c)
    }
}
impl From<StyleVal> for FontWeight {
    fn from(v: StyleVal) -> Self {
        match v {
            StyleVal::FontWeight(c) => c,
            x => panic!("Tried to coerce {x:?} into a FontWeight"),
        }
    }
}
impl From<Option<StyleVal>> for FontWeight {
    fn from(v: Option<StyleVal>) -> Self {
        match v {
            Some(StyleVal::FontWeight(c)) => c,
            x => panic!("Tried to coerce {x:?} into a FontWeight"),
        }
    }
}
impl From<f64> for StyleVal {
    fn from(c: f64) -> Self {
        Self::Float(c)
    }
}
impl From<StyleVal> for f64 {
    fn from(v: StyleVal) -> Self {
        match v {
            StyleVal::Float(c) => c,
            x => panic!("Tried to coerce {x:?} into a float"),
        }
    }
}
impl From<u32> for StyleVal {
    fn from(c: u32) -> Self {
        Self::Int(c)
    }
}
impl From<StyleVal> for u32 {
    fn from(v: StyleVal) -> Self {
        match v {
            StyleVal::Int(c) => c,
            x => panic!("Tried to coerce {x:?} into an int"),
        }
    }
}
impl From<bool> for StyleVal {
    fn from(c: bool) -> Self {
        Self::Bool(c)
    }
}
impl From<StyleVal> for bool {
    fn from(v: StyleVal) -> Self {
        match v {
            StyleVal::Bool(c) => c,
            x => panic!("Tried to coerce {x:?} into a bool"),
        }
    }
}
impl From<&'static str> for StyleVal {
    fn from(c: &'static str) -> Self {
        Self::String(c)
    }
}
impl From<StyleVal> for &str {
    fn from(v: StyleVal) -> Self {
        match v {
            StyleVal::String(c) => c,
            x => panic!("Tried to coerce {x:?} into a string"),
        }
    }
}

impl StyleVal {
    pub fn dimension(self) -> Dimension {
        self.into()
    }

    pub fn size(self) -> Size {
        self.into()
    }

    pub fn rect(self) -> Rect {
        self.into()
    }

    pub fn point(self) -> Point {
        self.into()
    }

    pub fn pos(self) -> Pos {
        self.into()
    }

    pub fn layout(self) -> Layout {
        self.into()
    }

    pub fn horizontal_position(self) -> HorizontalPosition {
        self.into()
    }

    pub fn vertical_position(self) -> VerticalPosition {
        self.into()
    }

    pub fn font_weight(self) -> FontWeight {
        self.into()
    }

    pub fn color(self) -> Color {
        self.into()
    }

    pub fn str(self) -> &'static str {
        self.into()
    }

    pub fn string(self) -> String {
        self.str().to_string()
    }

    pub fn f32(self) -> f32 {
        Into::<f64>::into(self) as f32
    }

    pub fn f64(self) -> f64 {
        self.into()
    }

    pub fn bool(self) -> bool {
        self.into()
    }

    pub fn u32(self) -> u32 {
        self.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct Widget {
        class: Option<&'static str>,
        style_overrides: StyleOverride,
    }
    impl Styled for Widget {
        fn name() -> &'static str {
            "Widget"
        }
        fn class(&self) -> Option<&'static str> {
            self.class
        }
        fn class_mut(&mut self) -> &mut Option<&'static str> {
            &mut self.class
        }
        fn style_overrides(&self) -> &StyleOverride {
            &self.style_overrides
        }
        fn style_overrides_mut(&mut self) -> &mut StyleOverride {
            &mut self.style_overrides
        }
    }

    fn test_style() -> Style {
        Style::new()
            .add(StyleKey::new("Widget", "color", None), Color::WHITE.into())
            .add(
                StyleKey::new("Widget", "color", Some("dark")),
                Color::BLACK.into(),
            )
    }

    #[test]
    fn test_base_style_val() {
        set_current_style(test_style());

        let w = Widget::default();
        let c: Color = w.style_val("color").into();
        assert_eq!(c, Color::WHITE);
    }

    #[test]
    fn test_style_val_with_class() {
        set_current_style(test_style());

        let w = Widget::default().with_class("dark");
        let c: Color = w.style_val("color").into();
        assert_eq!(c, Color::BLACK);
    }

    #[test]
    fn test_style_val_overrides() {
        set_current_style(test_style());

        let w = Widget::default().style("color", Color::BLUE);
        let c: Color = w.style_val("color").into();
        assert_eq!(c, Color::BLUE);

        let w = Widget::default()
            .with_class("dark") // Classes should not impact outcome
            .style("color", Color::BLUE);
        let c: Color = w.style_val("color").into();
        assert_eq!(c, Color::BLUE);
    }

    #[test]
    fn test_style_macro() {
        let s = style!(
            Widget.color = Color::WHITE;
            dark.Widget.color = Color::BLACK;
        );
        assert_eq!(s, test_style());
    }
}
