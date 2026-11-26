use ferrum_ui_core::a11y::{Aria, Role};
use ferrum_ui_core::ssr::Element;
use ferrum_ui_core::style::{Class, Style};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[non_exhaustive]
pub enum SpinnerSize {
    Sm,
    #[default]
    Md,
    Lg,
}

impl SpinnerSize {
    pub fn class(self) -> &'static str {
        match self {
            Self::Sm => "ferrum-spinner--sm",
            Self::Md => "ferrum-spinner--md",
            Self::Lg => "ferrum-spinner--lg",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpinnerProps {
    pub label: String,
    pub size: SpinnerSize,
    pub class: Class,
    pub style: Style,
}

impl Default for SpinnerProps {
    fn default() -> Self {
        Self {
            label: "Loading".to_owned(),
            size: SpinnerSize::Md,
            class: Class::new(),
            style: Style::new(),
        }
    }
}

impl SpinnerProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = label.into();
        self
    }

    pub fn with_size(mut self, size: SpinnerSize) -> Self {
        self.size = size;
        self
    }

    pub fn class(&self) -> Class {
        let mut class = Class::from("ferrum-spinner");
        class.add(self.size.class());
        class.merge(&self.class);
        class
    }

    pub fn aria(&self) -> Aria {
        Aria::new().role(Role::Status).label(self.label.clone())
    }

    pub fn element(&self) -> Element {
        let mut element = Element::new("div")
            .class(self.class())
            .style(self.style.clone());
        for (name, value) in self.aria().to_attrs() {
            element = element.attr(name.clone(), value.clone());
        }
        element.child(
            Element::new("span")
                .class("ferrum-spinner-icon")
                .attr("aria-hidden", "true"),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spinner_is_a_status_with_label() {
        let props = SpinnerProps::new().with_size(SpinnerSize::Lg);
        let html = props.element().render();
        assert!(html.contains("role=\"status\""));
        assert!(html.contains("aria-label=\"Loading\""));
        assert!(html.contains("ferrum-spinner--lg"));
    }
}
