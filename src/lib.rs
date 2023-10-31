use std::{
    io::{self, Write},
    process::{Child, ChildStdin, Command, Stdio},
};

pub fn run_with_output<T: Into<String>>(fzf: Fzf, items: Vec<T>) -> Option<String> {
    let mut fzf = fzf;
    fzf.run().ok()?;
    for item in items {
        fzf.add_item(item).ok()?;
    }
    fzf.output()
}

pub struct Fzf {
    instance: Option<Child>,
    stdin: Option<ChildStdin>,
    prompt: String,
    layout: FzfLayout,
}

impl Fzf {
    pub fn builder() -> FzfBuilder {
        FzfBuilder::default()
    }

    pub fn run(&mut self) -> io::Result<()> {
        let args = [
            format!("--prompt={}", self.prompt),
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

    pub fn output(self) -> Option<String> {
        let output = self
            .instance
            .expect("Failed to unwrap instance")
            .wait_with_output()
            .ok()?;
        Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }
}

#[derive(Clone)]
/// An Enum to represent the possible layouts to display `fzf` with
pub enum FzfLayout {
    Default,
    Reverse,
    ReverseList,
}

impl ToString for FzfLayout {
    fn to_string(&self) -> String {
        match self {
            FzfLayout::Default => "default",
            FzfLayout::Reverse => "reverse",
            FzfLayout::ReverseList => "reverse-list",
        }
        .to_string()
    }
}

#[derive(Clone)]
pub struct FzfBuilder {
    prompt: String,
    layout: FzfLayout,
}

impl Default for FzfBuilder {
    fn default() -> Self {
        Self {
            prompt: "> ".to_string(),
            layout: FzfLayout::Default,
        }
    }
}

impl FzfBuilder {
    /// The prompt in the search bar
    pub fn prompt<T: Into<String>>(&mut self, prompt: T) -> &mut Self {
        self.prompt = prompt.into();
        self
    }

    /// The layout to display `fzf` with
    pub fn layout(&mut self, layout: FzfLayout) -> &mut Self {
        self.layout = layout;
        self
    }

    pub fn build(&self) -> Fzf {
        let builder = self.clone();
        Fzf {
            instance: None,
            stdin: None,
            prompt: builder.prompt,
            layout: builder.layout,
        }
    }
}

#[cfg(test)]
mod tests {}
