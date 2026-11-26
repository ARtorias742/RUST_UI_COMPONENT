use ferrum_ui_core::a11y::{Aria, Role};
use ferrum_ui_core::event::{FerrumEventExt, KeyEvent};
use ferrum_ui_core::ssr::Element;
use ferrum_ui_core::style::{Class, Style};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SwitchProps {
    pub label: String,
    pub checked: bool,
    pub disabled: bool,
    pub class: Class,
    pub style: Style,
}

impl Default for SwitchProps {
    fn default() -> Self {
        Self {
            label: "Switch".to_owned(),
            checked: false,
            disabled: false,
            class: Class::new(),
            style: Style::new(),
        }
    }
}

impl SwitchProps {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            ..Self::default()
        }
    }

    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn class(&self) -> Class {
        let mut class = Class::from("ferrum-switch");
        if self.disabled {
            class.add("is-disabled");
        }
        class.merge(&self.class);
        class
    }

    pub fn aria(&self, state: &SwitchState) -> Aria {
        let mut aria = Aria::new()
            .role(Role::Switch)
            .set("aria-checked", if state.checked { "true" } else { "false" });
        if self.disabled {
            aria = aria.set("aria-disabled", "true");
        }
        aria
    }

    pub fn element(&self, state: &SwitchState) -> Element {
        let mut input = Element::new("input")
            .class("ferrum-switch-input")
            .attr("type", "checkbox");
        for (name, value) in self.aria(state).to_attrs() {
            input = input.attr(name.clone(), value.clone());
        }
        if state.checked {
            input = input.attr("checked", "");
        }
        if self.disabled {
            input = input.attr("disabled", "");
        }
        Element::new("label")
            .class(self.class())
            .style(self.style.clone())
            .child(input)
            .child(Element::new("span").class("ferrum-switch-track"))
            .child(Element::new("span").text(self.label.clone()))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SwitchAction {
    None,
    Changed(bool),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SwitchState {
    pub checked: bool,
}

impl SwitchState {
    pub fn new(props: &SwitchProps) -> Self {
        Self {
            checked: props.checked,
        }
    }

    pub fn toggle(&mut self) -> bool {
        self.checked = !self.checked;
        self.checked
    }

    pub fn handle_key(&mut self, props: &SwitchProps, event: &KeyEvent) -> SwitchAction {
        if props.disabled {
            return SwitchAction::None;
        }
        if event.is_space() || event.is_enter() {
            SwitchAction::Changed(self.toggle())
        } else {
            SwitchAction::None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ferrum_ui_core::event::Key;

    #[test]
    fn switch_toggles_with_space() {
        let props = SwitchProps::new("Dark mode");
        let mut state = SwitchState::new(&props);
        assert_eq!(
            state.handle_key(&props, &KeyEvent::new(Key::Space)),
            SwitchAction::Changed(true)
        );
        assert!(props
            .element(&state)
            .render()
            .contains("aria-checked=\"true\""));
    }
}
