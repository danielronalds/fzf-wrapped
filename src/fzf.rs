use std::{
    io::{self, Write},
    process::{Child, ChildStdin, Command, Stdio},
};

use derive_builder::Builder;

use crate::options::*;

const DEFAULT_PROMPT: &str = "> ";
const DEFAULT_POINTER: &str = ">";
const DEFAULT_BORDER_LABEL: &str = "";
const DEFAULT_HEADER: &str = "";

#[derive(Builder)]
/// Struct that represents the `fzf` program
pub struct Fzf {
    #[builder(setter(skip))]
    /// Contains the child stuct returned when the fzf command is spawned
    instance: Option<Child>,
    /// Contains the child process stdin handle after the `run` method has been called
    #[builder(setter(skip))]
    stdin: Option<ChildStdin>,

    /// Additional arguments that this library doesn't currently support as a predefined option
    #[builder(setter(custom), default = "vec![]")]
    custom_args: Vec<String>,

    // Search
    /// Scoring scheme
    #[builder(setter(into, strip_option), default = "Scheme::Default")]
    scheme: Scheme,
    /// Do not normalize latin script letters before matching
    #[builder(setter(into, strip_option), default = "false")]
    literal: bool,
    /// Track the current selection when the result is updated
    #[builder(setter(into, strip_option), default = "false")]
    track: bool,
    /// Reverse the order of the input
    #[builder(setter(into, strip_option), default = "false")]
    tac: bool,
    /// Do not perform search
    #[builder(setter(into, strip_option), default = "false")]
    disabled: bool,

    // Interface
    /// Disable mouse
    #[builder(setter(into, strip_option), default = "false")]
    no_mouse: bool,
    /// Enable cyclic scroll
    #[builder(setter(into, strip_option), default = "false")]
    cycle: bool,
    /// Keep the right end of the line visible on overflow
    #[builder(setter(into, strip_option), default = "false")]
    keep_right: bool,
    /// Disable horizontal scroll
    #[builder(setter(into, strip_option), default = "false")]
    no_hscroll: bool,
    /// Make word-wise movements respect path separators
    #[builder(setter(into, strip_option), default = "false")]
    filepath_word: bool,

    // Layout Options
    /// Choose layout
    #[builder(setter(into, strip_option), default = "Layout::Default")]
    layout: Layout,
    /// Draw border around the finder
    #[builder(setter(into, strip_option), default = "Border::None")]
    border: Border,
    /// Label to print on the border
    #[builder(
        setter(into, strip_option),
        default = "DEFAULT_BORDER_LABEL.to_string()"
    )]
    border_label: String,
    /// Hide info line separator
    #[builder(setter(into, strip_option), default = "false")]
    no_separator: bool,
    /// Hide scrollbar
    #[builder(setter(into, strip_option), default = "false")]
    no_scrollbar: bool,
    /// Input prompt (default: '> ')
    #[builder(setter(into, strip_option), default = "DEFAULT_PROMPT.to_string()")]
    prompt: String,
    /// Pointer to the current line (default: '>')
    #[builder(setter(into, strip_option), default = "DEFAULT_POINTER.to_string()")]
    pointer: String,
    /// String to print as header
    #[builder(setter(into, strip_option), default = "DEFAULT_HEADER.to_string()")]
    header: String,
    /// Print header before the prompt line
    #[builder(setter(into, strip_option), default = "false")]
    header_first: bool,

    // Display
    /// Enable processing of ANSI color codes
    #[builder(setter(into, strip_option), default = "false")]
    ansi: bool,
    /// Number of spaces for a tab character (default: 8)
    #[builder(setter(into, strip_option), default = "8")]
    tabstop: u8,
    /// Base scheme (dark|light|16|bw)
    #[builder(setter(into, strip_option), default = "Color::Dark")]
    color: Color,
    /// Do not use bold text
    #[builder(setter(into, strip_option), default = "false")]
    no_bold: bool,
}

impl FzfBuilder {
    /// Additional arguments that this library doesn't currently support as a predefined option
    #[allow(dead_code)]
    pub fn custom_args(&mut self, args: impl IntoIterator<Item = impl Into<String>>) -> &mut Self{
        self.custom_args = Some(args.into_iter().map(|x| x.into()).collect());
        self
    }
}

impl Fzf {
    /// Creates a [`FzfBuilder`]
    pub fn builder() -> FzfBuilder {
        FzfBuilder::default()
    }

    /// Spawns `fzf` as a child proccess, and displays it to stdout
    pub fn run(&mut self) -> io::Result<()> {
        let args: Vec<String> = self
            .get_fzf_args()
            .iter()
            .chain(self.custom_args.iter())
            .map(|x| x.to_owned())
            .collect();

        let mut fzf = Command::new("fzf")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .args(args)
            .spawn()?;

        self.stdin = fzf.stdin.take();
        self.instance = Some(fzf);
        Ok(())
    }

    /// Creates the vec of arguments to pass to `fzf`
    fn get_fzf_args(&self) -> Vec<String> {
        let mut args = vec![];

        /// Adds the option if the value is true
        fn add_if_true<T: Into<String>>(args: &mut Vec<String>, fzf_arg: T, value: bool) {
            if value {
                args.push(fzf_arg.into());
            }
        }

        // Search
        args.push(format!("--scheme={}", self.scheme.to_string()));
        add_if_true(&mut args, "--literal", self.literal);
        add_if_true(&mut args, "--track", self.track);
        add_if_true(&mut args, "--tac", self.tac);
        add_if_true(&mut args, "--disabled", self.disabled);

        // Interface
        add_if_true(&mut args, "--no-mouse", self.no_mouse);
        add_if_true(&mut args, "--cycle", self.cycle);
        add_if_true(&mut args, "--keep-right", self.keep_right);
        add_if_true(&mut args, "--no-hscroll", self.no_hscroll);
        add_if_true(&mut args, "--filepath-word", self.filepath_word);

        // Layout
        args.push(format!("--layout={}", self.layout.to_string()));
        args.push(format!("--border={}", self.border.to_string()));
        args.push(format!("--border-label={}", self.border_label));
        add_if_true(&mut args, "--no-separator", self.no_separator);
        add_if_true(&mut args, "--no-scrollbar", self.no_scrollbar);
        args.push(format!("--prompt={}", self.prompt));
        args.push(format!("--pointer={}", self.pointer));
        if !self.header.is_empty() {
            args.push(format!("--header={}", &self.header));
        }
        add_if_true(&mut args, "--header-first", self.header_first);

        // Display
        add_if_true(&mut args, "--ansi", self.ansi);
        args.push(format!("--tabstop={}", self.tabstop));
        args.push(format!("--color={}", self.color.to_string()));
        add_if_true(&mut args, "--no-bold", self.no_bold);

        args
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
    /// - `items` The items to add
    ///
    /// # Panics
    ///
    /// This function calls `add_item`, which will panic if the `run` method has not been called
    /// before adding items
    pub fn add_items(&mut self, items: impl IntoIterator<Item = impl Into<String>>) -> io::Result<()> {
        for item in items.into_iter() {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_works_as_expected() {
        let _ = Fzf::builder().build();
    }
}
