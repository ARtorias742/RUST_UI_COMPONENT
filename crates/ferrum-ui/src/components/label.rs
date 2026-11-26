use ferrum_ui_core::ssr::Element;
use ferrum_ui_core::style::{Class, Style};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LabelProps {
    pub text: String,
    pub for_id: String,
    pub class: Class,
    pub style: Style,
}

impl Default for LabelProps {
    fn default() -> Self {
        Self {
            text: "Label".to_owned(),
            for_id: String::new(),
            class: Class::new(),
            style: Style::new(),
        }
    }
}

impl LabelProps {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            ..Self::default()
        }
    }

    pub fn with_for(mut self, for_id: impl Into<String>) -> Self {
        self.for_id = for_id.into();
        self
    }

    pub fn class(&self) -> Class {
        let mut class = Class::from("ferrum-label");
        class.merge(&self.class);
        class
    }

    pub fn element(&self) -> Element {
        let mut element = Element::new("label")
            .class(self.class())
            .style(self.style.clone());
        if !self.for_id.is_empty() {
            element = element.attr("for", self.for_id.clone());
        }
        element.text(self.text.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn label_binds_to_control() {
        let html = LabelProps::new("Email")
            .with_for("email")
            .element()
            .render();
        assert!(html.contains("for=\"email\""));
        assert!(html.contains("Email"));
    }
}
