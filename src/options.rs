//! This module contains the enum's that represent certain `fzf` cli options

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