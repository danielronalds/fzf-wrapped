use std::{
    io::{self, Write},
    process::{Child, ChildStdin, Command, Stdio},
};

use derive_builder::Builder;

/// Runs the given [`Fzf`] struct and returns the user's selection as a [`String`]
///
/// # Parameters
///
/// - `items` The items to to display in `fzf`
///
/// # Returns
///
/// An option containing either the users selection as a [`String`], or `None` if the user quit `fzf`
pub fn run_with_output<T: Into<String>>(fzf: Fzf, items: Vec<T>) -> Option<String> {
    let mut fzf = fzf;
    fzf.run().ok()?;
    fzf.add_items(items).ok()?;
    fzf.output()
}

const DEFAULT_PROMPT: &str = "> ";
const DEFAULT_POINTER: &str = ">";

#[derive(Builder)]
/// Struct that represents the `fzf` program
pub struct Fzf {
    #[builder(setter(skip))]
    /// Contains the child stuct returned when the fzf command is spawned
    instance: Option<Child>,
    /// Contains the child process stdin handle after the `run` method has been called
    #[builder(setter(skip))]
    stdin: Option<ChildStdin>,

    // Options
    #[builder(setter(into, strip_option), default="DEFAULT_PROMPT.to_string()")]
    prompt: String,
    #[builder(setter(into, strip_option), default="DEFAULT_POINTER.to_string()")]
    pointer: String,
    #[builder(setter(into, strip_option), default="Layout::Default")]
    layout: Layout,
    #[builder(setter(into, strip_option), default="Color::Dark")]
    color: Color,
    #[builder(setter(into, strip_option), default="false")]
    no_bold: bool,
    #[builder(setter(into, strip_option), default="false")]
    disabled: bool,
}

impl Fzf {
    /// Creates a [`FzfBuilder`]
    pub fn builder() -> FzfBuilder {
        FzfBuilder::default()
    }

    /// Spawns `fzf` as a child proccess, and displays it to stdout
    pub fn run(&mut self) -> io::Result<()> {
        let mut args = vec![
            format!("--prompt={}", self.prompt),
            format!("--pointer={}", self.pointer),
            format!("--layout={}", self.layout.to_string()),
            format!("--color={}", self.color.to_string()),
        ];

        /// Adds the option if the value is true
        fn add_if_true<T: Into<String>>(args: &mut Vec<String>, fzf_arg: T, value: bool) {
            if value {
                args.push(fzf_arg.into());
            }
        }

        add_if_true(&mut args, "--no-bold", self.no_bold);
        add_if_true(&mut args, "--disabled", self.disabled);

        let mut fzf = Command::new("fzf")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .args(args)
            .spawn()?;

        self.stdin = fzf.stdin.take();
        self.instance = Some(fzf);
        Ok(())
    }

    /// Adds an item to the `fzf` selection ui
    ///
    /// # Parameters
    ///
    /// - `item` The item to add
    ///
    /// # Panics
    ///
    /// This function uses `expect()`, which will panic if the `run` method has not been called
    /// before adding items
    pub fn add_item<T: Into<String>>(&mut self, item: T) -> io::Result<()> {
        // Trimming the string to make sure we don't double up on newline characters
        let mut item = item.into().trim().to_string();
        item.push('\n');
        self.stdin
            .as_mut()
            .expect("Failed to unwrap stdin")
            .write_all(item.as_bytes())?;
        Ok(())
    }

    /// Adds all the items in the given vec to the `fzf` selection ui
    ///
    /// Essentially a wrapper for `add_item` for convenience
    ///
    /// # Parameters
    ///
    /// - `items` The vec of items to add
    ///
    /// # Panics
    ///
    /// This function calls `add_item`, which will panic if the `run` method has not been called
    /// before adding items
    pub fn add_items<T: Into<String>>(&mut self, items: Vec<T>) -> io::Result<()> {
        for item in items {
            self.add_item(item)?;
        }
        Ok(())
    }

    /// Gets the output of `fzf`.
    ///
    /// The stdout of fzf is converted to a string using `from_utf8_lossy`
    ///
    /// **Blocks execution until output is received**
    ///
    /// # Returns
    ///
    /// An option containing the users selection as a [`String`], or `None` if the user quit `fzf`
    pub fn output(self) -> Option<String> {
        let output = self.instance?.wait_with_output().ok()?;
        Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }
}

impl Default for Fzf {
    fn default() -> Self {
        FzfBuilder::default().build().unwrap()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_works_as_expected() {
        let _ = Fzf::builder().build();
    }
}
