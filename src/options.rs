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
        }.to_string()
    }
}
