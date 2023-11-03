//! # fzf-wrapper
//!
//! A library for integrating the `fzf` cli tool into your rust program!
//!
//! This library provides bindings for integrating the `fzf` cli program into your rust
//! projects.
//!
//! **NOTE** this does mean that the end user must have `fzf` installed on their system.
//!
//! #### fzf version
//! 
//! This crate was developed with `fzf` v0.40.0 in mind, however there should be no reason why it
//! shouldn't work above it, and most of the features should work below it. If your program relies
//! on v0.40.0 features, it might be a good idea to check the version of `fzf` your program has
//! access to
//!
//! ## Example
//!
//! Say we're wanting to get the user to select their favourite colour using the power of fuzzy
//! finding. First we'd create a list of colours, which will be a collection we'll pass to `fzf`
//!
//! ```rust
//! let colours = vec!["red", "orange", "yellow", "green", "blue", "indigo", "violet"];
//! ```
//!
//! The next step is to construct an instance of [`Fzf`] and start it:
//!
//! ```rust
//! let mut fzf = Fzf::default();
//!
//! fzf.run().expect("Failed to start fzf");
//! ```
//!
//! The code above fetches the default [`Fzf`] configuration, which runs `fzf` with no arguments,
//! and then calls the `run()` method. This displays `fzf` to the user.
//!
//! At the moment all that will be displayed is a blank screen, as we haven't actually told `fzf`
//! to display anything. There are two ways to do this, the `add_item()` method, and the
//! `add_items()` method. They are both nearly identical, with the only difference being that
//! `add_items()` takes a [`Vec`] of [`String`]'s as items, and passes them one by one to
//! `add_item()`.
//!
//! ```rust
//! fzf.add_items(colours).expect("Failed to add items");
//! ```
//!
//! The only thing left to do is to get what the user selected! This will be returned as an
//! [`Option`] containing `None` if the user exited `fzf`, or `Some(String)` with the string being
//! the item they selected. To get the output we simply call the `output()` method, which will
//! blocks execution until the user selects an item with `fzf`
//!
//! ```rust
//! let users_selection = fzf.output().expect("Failed to get the user's output");
//! ```
//!
//! The code in it's entirety looks like the following. 
//! 
//! ```rust
//! use fzf_wrapped::Fzf;
//!
//! fn main() {
//!     let colours = vec!["red", "orange", "yellow", "green", "blue", "indigo", "violet"];
//!     
//!     let mut fzf = Fzf::default();
//!     fzf.run().expect("Failed to start fzf");
//!
//!     fzf.add_items(colours).expect("Failed to add items");
//!
//!     let users_selection = fzf.output().expect("Failed to get the user's output");
//! }
//! ```
//!
//! This operation of using `fzf` to select from a predetermined [`Vec`] of items is so common that
//! a helper function exists to streamline the work involved. Using it, the code looks like the 
//! following:
//!
//! ```rust
//! use fzf_wrapped::Fzf;
//! use fzf_wrapped::run_with_output;
//!
//! fn main() {
//!     let colours = vec!["red", "orange", "yellow", "green", "blue", "indigo", "violet"];
//!     
//!     let users_selection = run_with_output(Fzf::default(), colours).expect("Something went wrong!");
//! }
//! ```
//!
//! ## Running `fzf` with arguments
//!
//! Now, our favourite color picker program is pretty cool, but our `fzf` interface is a bit
//! confusing, how are our user's supposed to know what they're picking?
//!
//! Thankfully `fzf` provides many different ways to customize it, many of which have been
//! implemented in this library. So far we have been calling the [`Fzf`] structs default
//! implementation, however we can build more complex instances using [`FzfBuilder`]. 
//!
//! We can use two different ways to get an [`FzfBuilder`], either through it's own `new()` method,
//! or the `builder()` method on [`Fzf`]. Let's switch out the default call for a builder call.
//!
//! ```rust
//! let fzf = Fzf::builder().build().unwrap();
//!
//! let users_selection = run_with_output(fzf, colours).expect("Something went wrong!");
//! ```
//!
//! **NOTE**: It is safe to unwrap the `build()` call, as every field has a default implementation.
//!
//! If we run the program now, we'll notice... nothing has changed. This is because the `default()`
//! method calls the exact line of code we just replaced it with.
//!
//! Let's make things interesting! First we should probably give the finder a label. Without a 
//! border, the label won't show so lets give it a border too! 
//!
//! ### Adding a border and border label
//!
//! If you were using `fzf` straight from the command line, you would pass it the `--border` flag
//! with the name of one of the supported types of borders, however this can lead to errors if the
//! border doesn't exist. For this reason, `fzf-wrapped` makes use of enums to ensure that these
//! choices are always valid! For example, to choose a border we'd call the `border()` method on
//! our [`FzfBuilder`], contain our chosen variant of the [`Border`] enum.
//!
//! Adding a rounded border makes our code look like this:
//!
//! ```rust
//! use fzf_wrapped::Fzf;
//! use fzf_wrapped::options::Border;
//! use fzf_wrapped::run_with_output;
//!
//! fn main() {
//!     let colours = vec!["red", "orange", "yellow", "green", "blue", "indigo", "violet"];
//!
//!     let fzf = Fzf::builder()
//!         .border(Border::Rounded)
//!         .build()
//!         .unwrap();
//!     
//!     let users_selection = run_with_output(fzf, colours);
//! }
//! ```
//!
//! And adding a label is even more simple
//!
//! ```rust
//! use fzf_wrapped::Fzf;
//! use fzf_wrapped::options::Border;
//! use fzf_wrapped::run_with_output;
//!
//! fn main() {
//!     let colours = vec!["red", "orange", "yellow", "green", "blue", "indigo", "violet"];
//!
//!     let fzf = Fzf::builder()
//!         .border(Border::Rounded)
//!         .border_label("Favourite Colour")
//!         .build()
//!         .unwrap();
//!     
//!     let users_selection = run_with_output(fzf, colours);
//! }
//! ```
//!
//! Running this should display an `fzf` finder with a rounded border, and a centered label
//! containing "Favourite Colour".
//!
//! ### Changing the layout of `fzf`
//!
//! Well, now that we've got a border, we may as well change up the layout. This is almost
//! identical to changing the border, as all possible layout's are mapped to an enum.
//!
//! All we need to add to our builder is the `layout()` method with our chosen [`Layout`] variant
//!
//! ```rust
//! use fzf_wrapped::Fzf;
//! use fzf_wrapped::options::{Border, Layout};
//! use fzf_wrapped::run_with_output;
//!
//! fn main() {
//!     let colours = vec!["red", "orange", "yellow", "green", "blue", "indigo", "violet"];
//!
//!     let fzf = Fzf::builder()
//!         .layout(Layout::Reverse)
//!         .border(Border::Rounded)
//!         .border_label("Favourite Colour")
//!         .build()
//!         .unwrap();
//!     
//!     let users_selection = run_with_output(fzf, colours);
//! }
//! ```
//!
//! ### Choosing colours
//!
//! `fzf` lets us pick from a few colour themes, but for this one we're going to keep it simple and
//! use the black and white theme. Similar to borders and the layout, this is selected using an
//! enum. Adding it to our builder results in the following code:
//!
//! ```rust
//! use fzf_wrapped::Fzf;
//! use fzf_wrapped::options::{Border, Color, Layout};
//! use fzf_wrapped::run_with_output;
//!
//! fn main() {
//!     let colours = vec!["red", "orange", "yellow", "green", "blue", "indigo", "violet"];
//!
//!     let fzf = Fzf::builder()
//!         .layout(Layout::Reverse)
//!         .border(Border::Rounded)
//!         .border_label("Favourite Colour")
//!         .color(Color::Bw)
//!         .build()
//!         .unwrap();
//!     
//!     let users_selection = run_with_output(fzf, colours);
//! }
//! ```
//!
//! If you run the program now, you'll notice that the ui is in black and white!
//!
//! ### Adding a header
//!
//! Our user might still be confused about what they're picking, so to add some more context, `fzf`
//! lets us set a header. To do this all we do is call the `header()` method on our [`FzfBuilder`] 
//! struct, and pass it anything with the `Into<String>` trait. We also want the header to appear
//! above our search field, so we'll call the `header_first()` method with `true`.
//!
//! ```rust
//! use fzf_wrapped::Fzf;
//! use fzf_wrapped::options::{Border, Color, Layout};
//! use fzf_wrapped::run_with_output;
//!
//! fn main() {
//!     let colours = vec!["red", "orange", "yellow", "green", "blue", "indigo", "violet"];
//!
//!     let fzf = Fzf::builder()
//!         .layout(Layout::Reverse)
//!         .border(Border::Rounded)
//!         .border_label("Favourite Colour")
//!         .color(Color::Bw)
//!         .header("Pick your favourite colour")
//!         .header_first(true)
//!         .build()
//!         .unwrap();
//!     
//!     let users_selection = run_with_output(fzf, colours);
//! }
//! ```
//!
//! ### Using an argument not supported by `fzf_wrapped`
//!
//! If by chance the argument you want to run with `fzf` is not a method included on the [`Fzf`]
//! struct, do not worry! The `custom_args()` command will let you pass any argument you want! For
//! example, say we wanted to make our colour picker program not take up the full screen, say only
//! 10% of it. `fzf` has the `--height=` flag, however the [`Fzf`] struct doesn't support it! To
//! add it all we'll need to do is to call the `custom_args()` command on our builder, and the
//! arguments we pass into it will be run with the `run()` method.
//!
//! Implementing it would look like:
//!
//! ```rust
//! use fzf_wrapped::Fzf;
//! use fzf_wrapped::options::{Border, Color, Layout};
//! use fzf_wrapped::run_with_output;
//!
//! fn main() {
//!     let colours = vec!["red", "orange", "yellow", "green", "blue", "indigo", "violet"];
//!
//!     let fzf = Fzf::builder()
//!         .layout(Layout::Reverse)
//!         .border(Border::Rounded)
//!         .border_label("Favourite Colour")
//!         .color(Color::Bw)
//!         .header("Pick your favourite colour")
//!         .header_first(true)
//!         .custom_args(vec!["--height=10".to_string()])
//!         .build()
//!         .unwrap();
//!     
//!     let users_selection = run_with_output(fzf, colours);
//!
//!     if let Some(colour) = users_selection {
//!         println!("{} is an awesome colour!", colour);
//!     }
//! }
//! ```
//!
//! And with that, our program is almost done!
//!
//! All we need to do is print some kind of nice message, and while we're at it, we may as well use
//! some proper error handling.
//!
//! ```rust
//! use fzf_wrapped::Fzf;
//! use fzf_wrapped::options::{Border, Color, Layout};
//! use fzf_wrapped::run_with_output;
//!
//! fn main() {
//!     let colours = vec!["red", "orange", "yellow", "green", "blue", "indigo", "violet"];
//!
//!     let fzf = Fzf::builder()
//!         .layout(Layout::Reverse)
//!         .border(Border::Rounded)
//!         .border_label("Favourite Colour")
//!         .color(Color::Bw)
//!         .header("Pick your favourite colour")
//!         .header_first(true)
//!         .custom_args(vec!["--height=10".to_string()])
//!         .build()
//!         .unwrap();
//!     
//!     let users_selection = run_with_output(fzf, colours);
//!
//!     if let Some(colour) = users_selection {
//!         println!("{} is an awesome colour!", colour);
//!     }
//! }
//! ```
//!
//! ## Adding Items at runtime of `fzf`
//!
//! With the power of this library, you can use `fzf` to select from a list of items, even if those
//! items have not been fetched yet. Using the `add_item` and `add_items` method adds items to 
//! `fzf`'s list, even while fzf is running. This means that if you're calling information from a 
//! REST api, you can display result's as they come in straight to `fzf`, or even hide the slight 
//! delay by starting up `fzf`.
//!
//! For an example of this, look at my [workflows](https://github.com/danielronalds/workflows) project

use std::{
    io::{self, Write},
    process::{Child, ChildStdin, Command, Stdio},
};

use derive_builder::Builder;

pub mod options;
use options::*;

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
    #[builder(setter(into, strip_option), default = "vec![]")]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_works_as_expected() {
        let _ = Fzf::builder().build();
    }
}
