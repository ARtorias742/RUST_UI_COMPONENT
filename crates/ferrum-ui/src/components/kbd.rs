use ferrum_ui_core::ssr::Element;
use ferrum_ui_core::style::{Class, Style};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KbdProps {
    pub label: String,
    pub class: Class,
    pub style: Style,
}

impl Default for KbdProps {
    fn default() -> Self {
        Self {
            label: "⌘K".to_owned(),
            class: Class::new(),
            style: Style::new(),
        }
    }
}

impl KbdProps {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            ..Self::default()
        }
    }

    pub fn class(&self) -> Class {
        let mut class = Class::from("ferrum-kbd");
        class.merge(&self.class);
        class
    }

    pub fn element(&self) -> Element {
        Element::new("kbd")
            .class(self.class())
            .style(self.style.clone())
            .text(self.label.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kbd_renders_shortcut() {
        assert_eq!(
            KbdProps::new("⌘K").element().render(),
            "<kbd class=\"ferrum-kbd\">⌘K</kbd>"
        );
    }
}
