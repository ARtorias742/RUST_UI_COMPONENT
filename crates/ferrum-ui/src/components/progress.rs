use ferrum_ui_core::a11y::{Aria, Role};
use ferrum_ui_core::ssr::Element;
use ferrum_ui_core::style::{Class, Style};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProgressProps {
    pub value: u32,
    pub max: u32,
    pub label: String,
    pub class: Class,
    pub style: Style,
}

impl Default for ProgressProps {
    fn default() -> Self {
        Self {
            value: 0,
            max: 100,
            label: "Progress".to_owned(),
            class: Class::new(),
            style: Style::new(),
        }
    }
}

impl ProgressProps {
    pub fn new(value: u32) -> Self {
        Self {
            value,
            ..Self::default()
        }
    }

    pub fn with_max(mut self, max: u32) -> Self {
        self.max = max.max(1);
        self
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = label.into();
        self
    }

    pub fn class(&self) -> Class {
        let mut class = Class::from("ferrum-progress");
        class.merge(&self.class);
        class
    }

    pub fn clamped_value(&self) -> u32 {
        self.value.min(self.max.max(1))
    }

    pub fn percent(&self) -> u32 {
        let max = self.max.max(1);
        ((self.clamped_value() as u64 * 100) / max as u64) as u32
    }

    pub fn aria(&self) -> Aria {
        Aria::new()
            .role(Role::Progressbar)
            .label(self.label.clone())
            .set("aria-valuemin", "0")
            .set("aria-valuemax", self.max.max(1).to_string())
            .set("aria-valuenow", self.clamped_value().to_string())
    }

    pub fn element(&self) -> Element {
        let mut root = Element::new("div")
            .class(self.class())
            .style(self.style.clone());
        for (name, value) in self.aria().to_attrs() {
            root = root.attr(name.clone(), value.clone());
        }
        let mut bar_style = Style::new();
        bar_style.set("width", format!("{}%", self.percent()));
        root.child(
            Element::new("div")
                .class("ferrum-progress-bar")
                .style(bar_style),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn progress_clamps_and_reports_value() {
        let props = ProgressProps::new(150).with_max(120);
        assert_eq!(props.clamped_value(), 120);
        assert_eq!(props.percent(), 100);
        let html = props.element().render();
        assert!(html.contains("role=\"progressbar\""));
        assert!(html.contains("aria-valuenow=\"120\""));
        assert!(html.contains("width: 100%"));
    }
}
