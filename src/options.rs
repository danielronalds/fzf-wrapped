//! This module contains the enum's that represent certain `fzf` cli options

#[derive(Clone, Copy)]
/// Enum to represent the scoring schemes fzf can use
pub enum Scheme {
    Default,
    Path,
    History,
}

impl ToString for Scheme {
    fn to_string(&self) -> String {
        match self {
            Self::Default => "default",
            Self::Path => "path",
            Self::History => "history",
        }
        .to_string()
    }
}

impl From<String> for Scheme {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "default" => Self::Default,
            "path" => Self::Path,
            "history" => Self::History,
            _ => Self::Default,
        }
    }
}

#[derive(Clone, Copy)]
/// Enum to represent the different themes fzf can have
pub enum Color {
    Dark,
    Light,
    Sixteen,
    Bw,
}

impl ToString for Color {
    fn to_string(&self) -> String {
        match self {
            Self::Dark => "dark",
            Self::Light => "light",
            Self::Sixteen => "16",
            Self::Bw => "bw",
        }
        .to_string()
    }
}

impl From<String> for Color {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "dark" => Self::Dark,
            "light" => Self::Light,
            "16" | "sixteen" => Self::Sixteen,
            "bw" => Self::Bw,
            _ => Self::Dark,
        }
    }
}

#[derive(Clone, Copy)]
/// An Enum to represent the possible layouts to display `fzf` with
pub enum Layout {
    Default,
    Reverse,
    ReverseList,
}

impl ToString for Layout {
    fn to_string(&self) -> String {
        match self {
            Layout::Default => "default",
            Layout::Reverse => "reverse",
            Layout::ReverseList => "reverse-list",
        }
        .to_string()
    }
}

impl From<String> for Layout {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "default" => Layout::Default,
            "reverse" => Layout::Reverse,
            "reverse-list" => Layout::ReverseList,
            _ => Layout::Default,
        }
    }
}

#[derive(Clone, Copy)]
/// An Enum to represent the possible borders to display around the finder
pub enum Border {
    None,
    Rounded,
    Sharp,
    Horizontal,
    Vertical,
    Top,
    Bottom,
    Left,
    Right,
}

impl ToString for Border {
    fn to_string(&self) -> String {
        match self {
            Border::None => "none",
            Border::Rounded => "rounded",
            Border::Sharp => "sharp",
            Border::Horizontal => "horizontal",
            Border::Vertical => "vertical",
            Border::Top => "top",
            Border::Bottom => "bottom",
            Border::Left => "left",
            Border::Right => "right",
        }
        .to_string()
    }
}

impl From<String> for Border {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "none" => Border::None,
            "rounded" => Border::Rounded,
            "sharp" => Border::Sharp,
            "horizontal" => Border::Horizontal,
            "vertical" => Border::Vertical,
            "top" => Border::Top,
            "bottom" => Border::Bottom,
            "left" => Border::Left,
            "right" => Border::Right,
            _ => Border::None,
        }
    }
}
