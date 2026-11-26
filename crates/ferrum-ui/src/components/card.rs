use ferrum_ui_core::ssr::Element;
use ferrum_ui_core::style::{Class, Style};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CardProps {
    pub title: String,
    pub description: String,
    pub footer: String,
    pub class: Class,
    pub style: Style,
}

impl Default for CardProps {
    fn default() -> Self {
        Self {
            title: "Card".to_owned(),
            description: String::new(),
            footer: String::new(),
            class: Class::new(),
            style: Style::new(),
        }
    }
}

impl CardProps {
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

    pub fn with_footer(mut self, footer: impl Into<String>) -> Self {
        self.footer = footer.into();
        self
    }

    pub fn class(&self) -> Class {
        let mut class = Class::from("ferrum-card");
        class.merge(&self.class);
        class
    }

    pub fn element(&self) -> Element {
        let mut card = Element::new("section")
            .class(self.class())
            .style(self.style.clone());
        card = card.child(
            Element::new("header")
                .class("ferrum-card-header")
                .child(Element::new("h2").text(self.title.clone()))
                .child(Element::new("p").text(self.description.clone())),
        );
        if !self.footer.is_empty() {
            card = card.child(
                Element::new("footer")
                    .class("ferrum-card-footer")
                    .text(self.footer.clone()),
            );
        }
        card
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn card_renders_header_and_optional_footer() {
        let props = CardProps::new("Project").with_description("Ship it");
        let html = props.element().render();
        assert!(html.contains("<h2>Project</h2>"));
        assert!(html.contains("Ship it"));
        assert!(!html.contains("ferrum-card-footer"));
    }
}
