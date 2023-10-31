pub struct Fzf {}

impl Fzf {
    pub fn builder() -> FzfBuilder {
        FzfBuilder::default()
    }
}

pub struct FzfBuilder {}

impl Default for FzfBuilder {
    fn default() -> Self {
        Self {}
    }
}

impl FzfBuilder {}

#[cfg(test)]
mod tests {}
