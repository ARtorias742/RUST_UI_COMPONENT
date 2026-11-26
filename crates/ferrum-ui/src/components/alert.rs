use ferrum_ui_core::a11y::{Aria, Role};
use ferrum_ui_core::ssr::Element;
use ferrum_ui_core::style::{Class, Style};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[non_exhaustive]
pub enum AlertVariant {
    #[default]
    Default,
    Success,
    Warning,
    Danger,
}

impl AlertVariant {
    pub fn class(self) -> &'static str {
        match self {
            Self::Default => "ferrum-alert--default",
            Self::Success => "ferrum-alert--success",
            Self::Warning => "ferrum-alert--warning",
            Self::Danger => "ferrum-alert--danger",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlertProps {
    pub variant: AlertVariant,
    pub title: String,
    pub description: String,
    pub class: Class,
    pub style: Style,
}

impl Default for AlertProps {
    fn default() -> Self {
        Self {
            variant: AlertVariant::Default,
            title: "Alert".to_owned(),
            description: String::new(),
            class: Class::new(),
            style: Style::new(),
        }
    }
}

impl AlertProps {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            ..Self::default()
        }
    }

    pub fn with_variant(mut self, variant: AlertVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    pub fn class(&self) -> Class {
        let mut class = Class::from("ferrum-alert");
        class.add(self.variant.class());
        class.merge(&self.class);
        class
    }

    pub fn aria(&self) -> Aria {
        Aria::new().role(Role::Alert)
    }

    pub fn element(&self) -> Element {
        let mut element = Element::new("div")
            .class(self.class())
            .style(self.style.clone());
        for (name, value) in self.aria().to_attrs() {
            element = element.attr(name.clone(), value.clone());
        }
        element = element.child(Element::new("strong").text(self.title.clone()));
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
    fn variant_class_and_role_are_rendered() {
        let props = AlertProps::new("Saved").with_variant(AlertVariant::Success);
        let html = props.element().render();
        assert!(html.contains("ferrum-alert--success"));
        assert!(html.contains("role=\"alert\""));
        assert!(html.contains("Saved"));
    }
}
