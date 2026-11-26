use ferrum_ui_core::ssr::Element;
use ferrum_ui_core::style::{Class, Style};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[non_exhaustive]
pub enum BadgeVariant {
    #[default]
    Default,
    Secondary,
    Outline,
    Success,
    Warning,
    Danger,
}

impl BadgeVariant {
    pub fn class(self) -> &'static str {
        match self {
            Self::Default => "ferrum-badge--default",
            Self::Secondary => "ferrum-badge--secondary",
            Self::Outline => "ferrum-badge--outline",
            Self::Success => "ferrum-badge--success",
            Self::Warning => "ferrum-badge--warning",
            Self::Danger => "ferrum-badge--danger",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BadgeProps {
    pub label: String,
    pub variant: BadgeVariant,
    pub class: Class,
    pub style: Style,
}

impl Default for BadgeProps {
    fn default() -> Self {
        Self {
            label: "Badge".to_owned(),
            variant: BadgeVariant::Default,
            class: Class::new(),
            style: Style::new(),
        }
    }
}

impl BadgeProps {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            ..Self::default()
        }
    }

    pub fn with_variant(mut self, variant: BadgeVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn class(&self) -> Class {
        let mut class = Class::from("ferrum-badge");
        class.add(self.variant.class());
        class.merge(&self.class);
        class
    }

    pub fn element(&self) -> Element {
        Element::new("span")
            .class(self.class())
            .style(self.style.clone())
            .text(self.label.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn badge_variant_is_exposed_as_a_class() {
        let props = BadgeProps::new("New").with_variant(BadgeVariant::Success);
        assert_eq!(
            props.element().render(),
            "<span class=\"ferrum-badge ferrum-badge--success\">New</span>"
        );
    }
}
