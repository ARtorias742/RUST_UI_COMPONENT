use ferrum_ui_core::a11y::{Aria, Role};
use ferrum_ui_core::event::{Key, KeyEvent};
use ferrum_ui_core::ssr::Element;
use ferrum_ui_core::style::{Class, Style};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextareaProps {
    pub label: String,
    pub value: String,
    pub placeholder: String,
    pub rows: usize,
    pub disabled: bool,
    pub invalid: bool,
    pub class: Class,
    pub style: Style,
}

impl Default for TextareaProps {
    fn default() -> Self {
        Self {
            label: "Textarea".to_owned(),
            value: String::new(),
            placeholder: String::new(),
            rows: 3,
            disabled: false,
            invalid: false,
            class: Class::new(),
            style: Style::new(),
        }
    }
}

impl TextareaProps {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            ..Self::default()
        }
    }

    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self
    }

    pub fn with_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    pub fn with_rows(mut self, rows: usize) -> Self {
        self.rows = rows.max(1);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn invalid(mut self, invalid: bool) -> Self {
        self.invalid = invalid;
        self
    }

    pub fn class(&self) -> Class {
        let mut class = Class::from("ferrum-textarea");
        if self.invalid {
            class.add("is-invalid");
        }
        class.merge(&self.class);
        class
    }

    pub fn aria(&self) -> Aria {
        let mut aria = Aria::new().role(Role::Textbox);
        if self.invalid {
            aria = aria.set("aria-invalid", "true");
        }
        if self.disabled {
            aria = aria.set("aria-disabled", "true");
        }
        aria
    }

    pub fn element(&self, state: &TextareaState) -> Element {
        let mut textarea = Element::new("textarea")
            .class(self.class())
            .style(self.style.clone())
            .attr("rows", self.rows.to_string())
            .text(state.value.clone());
        if !self.placeholder.is_empty() {
            textarea = textarea.attr("placeholder", self.placeholder.clone());
        }
        for (name, value) in self.aria().to_attrs() {
            textarea = textarea.attr(name.clone(), value.clone());
        }
        if self.disabled {
            textarea = textarea.attr("disabled", "");
        }
        Element::new("label")
            .class("ferrum-input-label")
            .child(Element::new("span").text(self.label.clone()))
            .child(textarea)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TextareaAction {
    None,
    Changed(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextareaState {
    pub value: String,
}

impl TextareaState {
    pub fn new(props: &TextareaProps) -> Self {
        Self {
            value: props.value.clone(),
        }
    }

    pub fn set(&mut self, value: impl Into<String>) {
        self.value = value.into();
    }

    pub fn handle_key(&mut self, props: &TextareaProps, event: &KeyEvent) -> TextareaAction {
        if props.disabled {
            return TextareaAction::None;
        }
        match event.key {
            Key::Backspace => {
                self.value.pop();
                TextareaAction::Changed(self.value.clone())
            }
            Key::Char(value) if event.is_printable() => {
                self.value.push(value);
                TextareaAction::Changed(self.value.clone())
            }
            Key::Space if event.is_printable() => {
                self.value.push(' ');
                TextareaAction::Changed(self.value.clone())
            }
            _ => TextareaAction::None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ferrum_ui_core::event::Key;

    #[test]
    fn textarea_typing_updates_value() {
        let props = TextareaProps::new("Bio");
        let mut state = TextareaState::new(&props);
        assert_eq!(
            state.handle_key(&props, &KeyEvent::new(Key::Char('a'))),
            TextareaAction::Changed("a".into())
        );
        assert_eq!(
            state.handle_key(&props, &KeyEvent::new(Key::Backspace)),
            TextareaAction::Changed(String::new())
        );
    }

    #[test]
    fn invalid_textarea_is_exposed() {
        let props = TextareaProps::new("Bio").invalid(true);
        let state = TextareaState::new(&props);
        assert!(props
            .element(&state)
            .render()
            .contains("aria-invalid=\"true\""));
    }
}
