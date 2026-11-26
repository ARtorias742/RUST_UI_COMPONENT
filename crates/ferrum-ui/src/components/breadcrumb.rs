use ferrum_ui_core::ssr::Element;
use ferrum_ui_core::style::{Class, Style};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BreadcrumbProps {
    pub items: Vec<String>,
    pub separator: String,
    pub class: Class,
    pub style: Style,
}

impl Default for BreadcrumbProps {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            separator: "/".to_owned(),
            class: Class::new(),
            style: Style::new(),
        }
    }
}

impl BreadcrumbProps {
    pub fn new(items: impl IntoIterator<Item = String>) -> Self {
        Self {
            items: items.into_iter().collect(),
            ..Self::default()
        }
    }

    pub fn with_separator(mut self, separator: impl Into<String>) -> Self {
        self.separator = separator.into();
        self
    }

    pub fn class(&self) -> Class {
        let mut class = Class::from("ferrum-breadcrumb");
        class.merge(&self.class);
        class
    }

    pub fn element(&self) -> Element {
        let mut list = Element::new("ol").class("ferrum-breadcrumb-list");
        for (index, item) in self.items.iter().enumerate() {
            let mut entry = Element::new("li").class("ferrum-breadcrumb-item");
            if index + 1 == self.items.len() {
                entry = entry.attr("aria-current", "page");
            }
            entry = entry.child(Element::new("span").text(item.clone()));
            if index + 1 < self.items.len() {
                entry = entry.child(
                    Element::new("span")
                        .class("ferrum-breadcrumb-separator")
                        .attr("aria-hidden", "true")
                        .text(self.separator.clone()),
                );
            }
            list = list.child(entry);
        }
        Element::new("nav")
            .class(self.class())
            .style(self.style.clone())
            .attr("aria-label", "breadcrumb")
            .child(list)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn breadcrumb_marks_current_page() {
        let props = BreadcrumbProps::new(vec!["Home".into(), "Docs".into()]);
        let html = props.element().render();
        assert!(html.contains("aria-label=\"breadcrumb\""));
        assert!(html.contains("aria-current=\"page\""));
        assert!(html.contains("Docs"));
    }
}
