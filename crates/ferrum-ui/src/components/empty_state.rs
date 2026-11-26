use ferrum_ui_core::ssr::Element;
use ferrum_ui_core::style::{Class, Style};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmptyStateProps {
    pub title: String,
    pub description: String,
    pub class: Class,
    pub style: Style,
}

impl Default for EmptyStateProps {
    fn default() -> Self {
        Self {
            title: "Nothing here".to_owned(),
            description: String::new(),
            class: Class::new(),
            style: Style::new(),
        }
    }
}

impl EmptyStateProps {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            ..Self::default()
        }
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    pub fn class(&self) -> Class {
        let mut class = Class::from("ferrum-empty-state");
        class.merge(&self.class);
        class
    }

    pub fn element(&self) -> Element {
        let mut element = Element::new("section")
            .class(self.class())
            .style(self.style.clone())
            .child(Element::new("h2").text(self.title.clone()));
        if !self.description.is_empty() {
            element = element.child(Element::new("p").text(self.description.clone()));
        }
        element
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_state_renders_title_and_description() {
        let props = EmptyStateProps::new("No results").with_description("Try another filter");
        let html = props.element().render();
        assert!(html.contains("No results"));
        assert!(html.contains("Try another filter"));
    }
}
