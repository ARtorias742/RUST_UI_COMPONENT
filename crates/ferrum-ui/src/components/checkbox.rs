use ferrum_ui_core::a11y::{Aria, Role};
use ferrum_ui_core::event::{FerrumEventExt, KeyEvent};
use ferrum_ui_core::ssr::Element;
use ferrum_ui_core::style::{Class, Style};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckboxProps {
    pub label: String,
    pub checked: bool,
    pub disabled: bool,
    pub indeterminate: bool,
    pub class: Class,
    pub style: Style,
}

impl Default for CheckboxProps {
    fn default() -> Self {
        Self {
            label: "Checkbox".to_owned(),
            checked: false,
            disabled: false,
            indeterminate: false,
            class: Class::new(),
            style: Style::new(),
        }
    }
}

impl CheckboxProps {
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

    pub fn indeterminate(mut self, indeterminate: bool) -> Self {
        self.indeterminate = indeterminate;
        self
    }

    pub fn class(&self) -> Class {
        let mut class = Class::from("ferrum-checkbox");
        if self.disabled {
            class.add("is-disabled");
        }
        if self.indeterminate {
            class.add("is-indeterminate");
        }
        class.merge(&self.class);
        class
    }

    pub fn aria(&self, state: &CheckboxState) -> Aria {
        let checked = if state.indeterminate {
            "mixed"
        } else if state.checked {
            "true"
        } else {
            "false"
        };
        let mut aria = Aria::new()
            .role(Role::Checkbox)
            .set("aria-checked", checked);
        if self.disabled {
            aria = aria.set("aria-disabled", "true");
        }
        aria
    }

    pub fn element(&self, state: &CheckboxState) -> Element {
        let mut input = Element::new("input")
            .class("ferrum-checkbox-input")
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
            .child(Element::new("span").text(self.label.clone()))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum CheckboxAction {
    None,
    Changed(bool),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckboxState {
    pub checked: bool,
    pub indeterminate: bool,
}

impl CheckboxState {
    pub fn new(props: &CheckboxProps) -> Self {
        Self {
            checked: props.checked,
            indeterminate: props.indeterminate,
        }
    }

    pub fn toggle(&mut self) -> bool {
        self.checked = !self.checked;
        self.indeterminate = false;
        self.checked
    }

    pub fn handle_key(&mut self, props: &CheckboxProps, event: &KeyEvent) -> CheckboxAction {
        if props.disabled {
            return CheckboxAction::None;
        }
        if event.is_space() || event.is_enter() {
            CheckboxAction::Changed(self.toggle())
        } else {
            CheckboxAction::None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ferrum_ui_core::event::Key;

    #[test]
    fn space_toggles_and_clears_indeterminate() {
        let props = CheckboxProps::new("Accept").indeterminate(true);
        let mut state = CheckboxState::new(&props);
        assert_eq!(
            state.handle_key(&props, &KeyEvent::new(Key::Space)),
            CheckboxAction::Changed(true)
        );
        assert!(!state.indeterminate);
        assert!(props
            .element(&state)
            .render()
            .contains("aria-checked=\"true\""));
    }

    #[test]
    fn disabled_checkbox_ignores_keyboard() {
        let props = CheckboxProps::new("Accept").disabled(true);
        let mut state = CheckboxState::new(&props);
        assert_eq!(
            state.handle_key(&props, &KeyEvent::new(Key::Space)),
            CheckboxAction::None
        );
    }
}
