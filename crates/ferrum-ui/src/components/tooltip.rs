use ferrum_ui_core::a11y::{Aria, Role};
use ferrum_ui_core::ssr::Element;
use ferrum_ui_core::style::{Class, Style};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TooltipProps {
    pub text: String,
    pub label: String,
    pub class: Class,
    pub style: Style,
}

impl Default for TooltipProps {
    fn default() -> Self {
        Self {
            text: "Tooltip".to_owned(),
            label: String::new(),
            class: Class::new(),
            style: Style::new(),
        }
    }
}

impl TooltipProps {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            ..Self::default()
        }
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = label.into();
        self
    }

    pub fn class(&self) -> Class {
        let mut class = Class::from("ferrum-tooltip");
        class.merge(&self.class);
        class
    }

    pub fn aria(&self) -> Aria {
        let mut aria = Aria::new().role(Role::Tooltip);
        if !self.label.is_empty() {
            aria = aria.label(self.label.clone());
        }
        aria
    }

    pub fn element(&self) -> Element {
        let mut element = Element::new("span")
            .class(self.class())
            .style(self.style.clone());
        for (name, value) in self.aria().to_attrs() {
            element = element.attr(name.clone(), value.clone());
        }
        element.text(self.text.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tooltip_has_tooltip_role() {
        let props = TooltipProps::new("More info").with_label("Info");
        let html = props.element().render();
        assert!(html.contains("role=\"tooltip\""));
        assert!(html.contains("aria-label=\"Info\""));
        assert!(html.contains("More info"));
    }
}
