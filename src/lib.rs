use std::{
    io::{self, Write},
    process::{Child, ChildStdin, Command, Stdio},
};

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

/// Struct that represents the `fzf` program
pub struct Fzf {
    /// Contains the child stuct returned when the fzf command is spawned
    instance: Option<Child>,
    /// Contains the child process stdin handle after the `run` method has been called
    stdin: Option<ChildStdin>,

    // Options
    prompt: String,
    pointer: String,
    layout: Layout,
}

impl Fzf {
    /// Creates a [`FzfBuilder`]
    pub fn builder() -> FzfBuilder {
        FzfBuilder::default()
    }

    /// Spawns `fzf` as a child proccess, and displays it to stdout
    pub fn run(&mut self) -> io::Result<()> {
        let args = [
            format!("--prompt={}", self.prompt),
            format!("--pointer={}", self.pointer),
            format!("--layout={}", self.layout.to_string()),
        ];

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

#[derive(Clone)]
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

#[derive(Clone)]
pub struct FzfBuilder {
    prompt: String,
    pointer: String,
    layout: Layout,
}

impl Default for FzfBuilder {
    fn default() -> Self {
        Self {
            prompt: "> ".to_string(),
            pointer: ">".to_string(),
            layout: Layout::Default,
        }
    }
}

impl FzfBuilder {
    /// The prompt in the search bar
    pub fn prompt<T: Into<String>>(&mut self, prompt: T) -> &mut Self {
        self.prompt = prompt.into();
        self
    }

    /// The pointer to the current line
    pub fn pointer<T: Into<String>>(&mut self, pointer: T) -> &mut Self {
        self.pointer = pointer.into();
        self
    }

    /// The layout to display `fzf` with
    pub fn layout(&mut self, layout: Layout) -> &mut Self {
        self.layout = layout;
        self
    }

    pub fn build(&self) -> Fzf {
        let builder = self.clone();
        Fzf {
            instance: None,
            stdin: None,
            prompt: builder.prompt,
            pointer: builder.pointer,
            layout: builder.layout,
        }
    }
}

#[cfg(test)]
mod tests {}
