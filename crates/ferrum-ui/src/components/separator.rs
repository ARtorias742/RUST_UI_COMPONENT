use ferrum_ui_core::a11y::{Aria, Role};
use ferrum_ui_core::ssr::Element;
use ferrum_ui_core::style::{Class, Style};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[non_exhaustive]
pub enum SeparatorOrientation {
    #[default]
    Horizontal,
    Vertical,
}

impl SeparatorOrientation {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
        }
    }

    pub fn class(self) -> &'static str {
        match self {
            Self::Horizontal => "ferrum-separator--horizontal",
            Self::Vertical => "ferrum-separator--vertical",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SeparatorProps {
    pub orientation: SeparatorOrientation,
    pub decorative: bool,
    pub class: Class,
    pub style: Style,
}

impl Default for SeparatorProps {
    fn default() -> Self {
        Self {
            orientation: SeparatorOrientation::Horizontal,
            decorative: false,
            class: Class::new(),
            style: Style::new(),
        }
    }
}

impl SeparatorProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn vertical(mut self) -> Self {
        self.orientation = SeparatorOrientation::Vertical;
        self
    }

    pub fn decorative(mut self, decorative: bool) -> Self {
        self.decorative = decorative;
        self
    }

    pub fn class(&self) -> Class {
        let mut class = Class::from("ferrum-separator");
        class.add(self.orientation.class());
        class.merge(&self.class);
        class
    }

    pub fn aria(&self) -> Aria {
        if self.decorative {
            Aria::new().hidden(true)
        } else {
            Aria::new()
                .role(Role::Separator)
                .set("aria-orientation", self.orientation.as_str())
        }
    }

    pub fn element(&self) -> Element {
        let mut element = Element::new("div")
            .class(self.class())
            .style(self.style.clone());
        for (name, value) in self.aria().to_attrs() {
            element = element.attr(name.clone(), value.clone());
        }
        element
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decorative_separator_is_hidden_from_at() {
        let props = SeparatorProps::new().vertical().decorative(true);
        let html = props.element().render();
        assert!(html.contains("aria-hidden=\"true\""));
        assert!(html.contains("ferrum-separator--vertical"));
    }

    #[test]
    fn semantic_separator_reports_orientation() {
        let html = SeparatorProps::new().element().render();
        assert!(html.contains("role=\"separator\""));
        assert!(html.contains("aria-orientation=\"horizontal\""));
    }
}
