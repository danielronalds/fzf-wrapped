pub struct Fzf {
    prompt: String,
}

impl Fzf {
    pub fn builder() -> FzfBuilder {
        FzfBuilder::default()
    }
}

pub struct FzfBuilder {
    prompt: String,
}

impl Default for FzfBuilder {
    fn default() -> Self {
        Self { prompt: "> ".to_string() }
    }
}

impl FzfBuilder {
    /// The prompt in the search bar
    pub fn prompt<T: Into<String>>(&mut self, prompt: T) -> &mut Self {
        self.prompt = prompt.into();
        self
    }

    pub fn build(self) -> Fzf {
        Fzf {
            prompt: self.prompt,
        }
    }
}

#[cfg(test)]
mod tests {}
