use ferrum_ui_core::ssr::Element;
use ferrum_ui_core::style::{Class, Style};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkeletonProps {
    pub width: String,
    pub height: String,
    pub class: Class,
    pub style: Style,
}

impl Default for SkeletonProps {
    fn default() -> Self {
        Self {
            width: "100%".to_owned(),
            height: "1rem".to_owned(),
            class: Class::new(),
            style: Style::new(),
        }
    }
}

impl SkeletonProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_size(mut self, width: impl Into<String>, height: impl Into<String>) -> Self {
        self.width = width.into();
        self.height = height.into();
        self
    }

    pub fn class(&self) -> Class {
        let mut class = Class::from("ferrum-skeleton");
        class.merge(&self.class);
        class
    }

    pub fn element(&self) -> Element {
        let mut style = Style::new();
        style.set("width", self.width.clone());
        style.set("height", self.height.clone());
        style.merge(&self.style);
        Element::new("div")
            .class(self.class())
            .style(style)
            .attr("aria-hidden", "true")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn skeleton_is_hidden_and_sized() {
        let props = SkeletonProps::new().with_size("120px", "2rem");
        let html = props.element().render();
        assert!(html.contains("aria-hidden=\"true\""));
        assert!(html.contains("width: 120px"));
        assert!(html.contains("height: 2rem"));
    }
}
