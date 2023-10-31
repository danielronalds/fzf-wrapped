use std::{
    io::Write,
    process::{Command, Stdio},
};

pub struct Fzf {
    prompt: String,
    layout: FzfLayout,
}

impl Fzf {
    pub fn builder() -> FzfBuilder {
        FzfBuilder::default()
    }

    /// Runs `fzf` and returns the result of the user's selection
    ///
    /// # Parameters
    ///
    /// - `items` The items to select from using fzf
    ///
    /// # Returns
    ///
    /// An option containg what the user selected, or `None` if either the user exited fzf
    pub fn run_with_output<T: Into<String>>(&self, items: Vec<T>) -> Option<String> {
        let args = [
            format!("--prompt={}", self.prompt),
            format!("--layout={}", self.layout.to_string()),
        ];

        let mut fzf = Command::new("fzf")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .args(args)
            .spawn()
            .ok()?;

        let mut stdin = fzf.stdin.take()?;

        let mut input: String = items
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<String>>()
            .join("\n");
        input.push('\n'); // So that fzf recognises the last item

        stdin.write_all(input.as_bytes()).ok()?;

        let output = fzf.wait_with_output().ok()?;

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
            prompt: builder.prompt,
            layout: builder.layout,
        }
    }
}

#[cfg(test)]
mod tests {}
