use ferrum_ui_core::a11y::{Aria, Role};
use ferrum_ui_core::ssr::Element;
use ferrum_ui_core::style::{Class, Style};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[non_exhaustive]
pub enum AvatarSize {
    Sm,
    #[default]
    Md,
    Lg,
}

impl AvatarSize {
    pub fn class(self) -> &'static str {
        match self {
            Self::Sm => "ferrum-avatar--sm",
            Self::Md => "ferrum-avatar--md",
            Self::Lg => "ferrum-avatar--lg",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AvatarProps {
    pub initials: String,
    pub label: String,
    pub size: AvatarSize,
    pub class: Class,
    pub style: Style,
}

impl Default for AvatarProps {
    fn default() -> Self {
        Self {
            initials: String::new(),
            label: String::new(),
            size: AvatarSize::Md,
            class: Class::new(),
            style: Style::new(),
        }
    }
}

impl AvatarProps {
    pub fn new(initials: impl Into<String>) -> Self {
        Self {
            initials: initials.into(),
            ..Self::default()
        }
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = label.into();
        self
    }

    pub fn with_size(mut self, size: AvatarSize) -> Self {
        self.size = size;
        self
    }

    pub fn class(&self) -> Class {
        let mut class = Class::from("ferrum-avatar");
        class.add(self.size.class());
        class.merge(&self.class);
        class
    }

    pub fn aria(&self) -> Aria {
        let label = if self.label.is_empty() {
            self.initials.clone()
        } else {
            self.label.clone()
        };
        Aria::new().role(Role::Img).label(label)
    }

    pub fn element(&self) -> Element {
        let mut element = Element::new("span")
            .class(self.class())
            .style(self.style.clone());
        for (name, value) in self.aria().to_attrs() {
            element = element.attr(name.clone(), value.clone());
        }
        element.text(self.initials.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn avatar_exposes_an_accessible_name() {
        let props = AvatarProps::new("AB").with_size(AvatarSize::Lg);
        let html = props.element().render();
        assert!(html.contains("role=\"img\""));
        assert!(html.contains("aria-label=\"AB\""));
        assert!(html.contains("ferrum-avatar--lg"));
    }
}
