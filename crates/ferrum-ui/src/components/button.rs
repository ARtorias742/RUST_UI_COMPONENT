use ferrum_ui_core::a11y::{Aria, Role};
use ferrum_ui_core::ssr::Element;
use ferrum_ui_core::style::{Class, Style};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[non_exhaustive]
pub enum ButtonVariant {
    #[default]
    Default,
    Secondary,
    Outline,
    Ghost,
    Destructive,
    Link,
}

impl ButtonVariant {
    pub fn class(self) -> &'static str {
        match self {
            Self::Default => "ferrum-button--default",
            Self::Secondary => "ferrum-button--secondary",
            Self::Outline => "ferrum-button--outline",
            Self::Ghost => "ferrum-button--ghost",
            Self::Destructive => "ferrum-button--destructive",
            Self::Link => "ferrum-button--link",
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[non_exhaustive]
pub enum ButtonSize {
    #[default]
    Default,
    Sm,
    Lg,
    Icon,
}

impl ButtonSize {
    pub fn class(self) -> &'static str {
        match self {
            Self::Default => "ferrum-button--md",
            Self::Sm => "ferrum-button--sm",
            Self::Lg => "ferrum-button--lg",
            Self::Icon => "ferrum-button--icon",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ButtonProps {
    pub variant: ButtonVariant,
    pub size: ButtonSize,
    pub disabled: bool,
    pub loading: bool,
    pub label: String,
    pub class: Class,
    pub style: Style,
}

impl Default for ButtonProps {
    fn default() -> Self {
        Self {
            variant: ButtonVariant::Default,
            size: ButtonSize::Default,
            disabled: false,
            loading: false,
            label: "Button".to_owned(),
            class: Class::new(),
            style: Style::new(),
        }
    }
}

impl ButtonProps {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            ..Self::default()
        }
    }

    pub fn with_variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn with_size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    pub fn with_class(mut self, class: impl Into<Class>) -> Self {
        self.class = class.into();
        self
    }

    pub fn with_style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }

    pub fn class(&self) -> Class {
        let mut class = Class::from("ferrum-button");
        class.add(self.variant.class()).add(self.size.class());
        if self.disabled {
            class.add("is-disabled");
        }
        if self.loading {
            class.add("is-loading");
        }
        class.merge(&self.class);
        class
    }

    pub fn aria(&self) -> Aria {
        let mut aria = Aria::new().role(Role::Button);
        if self.disabled {
            aria = aria.set("aria-disabled", "true");
        }
        if self.loading {
            aria = aria.set("aria-busy", "true");
        }
        aria
    }

    pub fn element(&self) -> Element {
        let mut element = Element::new("button")
            .class(self.class())
            .style(self.style.clone())
            .attr("type", "button");
        for (name, value) in self.aria().to_attrs() {
            element = element.attr(name.clone(), value.clone());
        }
        if self.disabled {
            element = element.attr("disabled", "");
        }
        element.text(self.label.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_merge_user_class_and_style() {
        let mut style = Style::new();
        style.set("color", "red");
        let props = ButtonProps::new("Save")
            .with_variant(ButtonVariant::Destructive)
            .with_size(ButtonSize::Sm)
            .with_class("w-full")
            .with_style(style);
        assert_eq!(
            props.class().to_string(),
            "ferrum-button ferrum-button--destructive ferrum-button--sm w-full"
        );
        assert!(props.element().render().contains("color: red"));
    }

    #[test]
    fn disabled_and_loading_are_exposed_to_at() {
        let props = ButtonProps::new("Save").disabled(true).loading(true);
        let html = props.element().render();
        assert!(html.contains("aria-disabled=\"true\""));
        assert!(html.contains("aria-busy=\"true\""));
        assert!(html.contains("disabled=\"\""));
    }
}
