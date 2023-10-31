pub struct Fzf {
    prompt: String,
    layout: FzfLayout,
}

impl Fzf {
    pub fn builder() -> FzfBuilder {
        FzfBuilder::default()
    }
}

/// An Enum to represent the possible layouts to display `fzf` with
pub enum FzfLayout {
    Default,
    Reverse,
    ReverseList,
}

impl Into<String> for FzfLayout {
    fn into(self) -> String {
        match self {
            FzfLayout::Default => "default",
            FzfLayout::Reverse => "reverse",
            FzfLayout::ReverseList => "reverse-list",
        }
        .to_string()
    }
}

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

    pub fn build(self) -> Fzf {
        Fzf {
            prompt: self.prompt,
            layout: self.layout,
        }
    }
}

#[cfg(test)]
mod tests {}
