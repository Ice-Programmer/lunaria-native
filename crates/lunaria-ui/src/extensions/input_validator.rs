use gpui_kit::App;

#[derive(Default)]
pub struct Validator {
    rules: Vec<Box<dyn Fn(&str) -> bool>>,
}

impl Validator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn rule(mut self, rule: impl Fn(&str) -> bool + 'static) -> Self {
        self.rules.push(Box::new(rule));
        self
    }

    pub fn required(self) -> Self {
        self.rule(|value| !value.trim().is_empty())
    }

    pub fn min_len(self, min: usize) -> Self {
        self.rule(move |value| value.chars().count() >= min)
    }

    pub fn max_len(self, max: usize) -> Self {
        self.rule(move |value| value.chars().count() <= max)
    }

    pub fn no_control_chars(self) -> Self {
        self.rule(|value| !value.chars().any(char::is_control))
    }

    pub fn forbid_chars(self, chars: impl Into<String>) -> Self {
        let chars = chars.into();

        self.rule(move |value| !value.chars().any(|ch| chars.contains(ch)))
    }

    pub fn not_in(self, values: impl IntoIterator<Item = impl Into<String>>) -> Self {
        let values: Vec<String> = values.into_iter().map(Into::into).collect();

        self.rule(move |value| !values.iter().any(|item| item == value.trim()))
    }

    pub fn build(self) -> impl Fn(&str, &mut App) -> bool + 'static {
        move |value, _| self.rules.iter().all(|rule| rule(value))
    }
}
