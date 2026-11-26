use ferrum_ui_core::ssr::Element;
use ferrum_ui_core::style::{Class, Style};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SliderProps {
    pub value: i64,
    pub min: i64,
    pub max: i64,
    pub step: i64,
    pub label: String,
    pub disabled: bool,
    pub class: Class,
    pub style: Style,
}

impl Default for SliderProps {
    fn default() -> Self {
        Self {
            value: 50,
            min: 0,
            max: 100,
            step: 1,
            label: "Slider".to_owned(),
            disabled: false,
            class: Class::new(),
            style: Style::new(),
        }
    }
}

impl SliderProps {
    pub fn new(value: i64) -> Self {
        Self {
            value,
            ..Self::default()
        }
    }

    pub fn with_range(mut self, min: i64, max: i64) -> Self {
        self.min = min;
        self.max = max;
        self
    }

    pub fn with_step(mut self, step: i64) -> Self {
        self.step = step.max(1);
        self
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = label.into();
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn clamped(&self) -> i64 {
        self.value.clamp(self.min, self.max)
    }

    pub fn class(&self) -> Class {
        let mut class = Class::from("ferrum-slider");
        class.merge(&self.class);
        class
    }

    pub fn element(&self) -> Element {
        let mut input = Element::new("input")
            .class("ferrum-slider-input")
            .attr("type", "range")
            .attr("min", self.min.to_string())
            .attr("max", self.max.to_string())
            .attr("step", self.step.to_string())
            .attr("value", self.clamped().to_string())
            .attr("aria-label", self.label.clone());
        if self.disabled {
            input = input.attr("disabled", "");
        }
        Element::new("label")
            .class(self.class())
            .style(self.style.clone())
            .child(Element::new("span").text(self.label.clone()))
            .child(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slider_clamps_and_exposes_range() {
        let props = SliderProps::new(150).with_range(0, 100);
        let html = props.element().render();
        assert!(html.contains("type=\"range\""));
        assert!(html.contains("value=\"100\""));
        assert!(html.contains("aria-label=\"Slider\""));
    }
}
