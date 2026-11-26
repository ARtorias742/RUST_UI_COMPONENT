use ferrum_ui_core::a11y::{Aria, Role};
use ferrum_ui_core::event::{FerrumEventExt, KeyEvent};
use ferrum_ui_core::focus::{FocusManager, FocusScope};
use ferrum_ui_core::ssr::Element;
use ferrum_ui_core::style::{Class, Style};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DialogProps {
    pub title: String,
    pub description: String,
    pub open: bool,
    pub modal: bool,
    pub close_on_escape: bool,
    pub close_on_outside: bool,
    pub class: Class,
    pub style: Style,
}

impl Default for DialogProps {
    fn default() -> Self {
        Self {
            title: "Dialog".to_owned(),
            description: String::new(),
            open: false,
            modal: true,
            close_on_escape: true,
            close_on_outside: true,
            class: Class::new(),
            style: Style::new(),
        }
    }
}

impl DialogProps {
    pub fn element(&self, controller: &DialogController) -> Option<Element> {
        if !controller.state.open {
            return None;
        }
        let mut panel = Element::new("div")
            .class("ferrum-dialog")
            .attr("role", "dialog")
            .attr("aria-modal", if self.modal { "true" } else { "false" })
            .attr("aria-label", self.title.clone())
            .attr("data-state", "open");
        if !self.description.is_empty() {
            panel = panel.attr("aria-describedby", "dialog-description");
        }
        panel = panel.child(
            Element::new("h2")
                .attr("id", "dialog-title")
                .text(self.title.clone()),
        );
        if !self.description.is_empty() {
            panel = panel.child(
                Element::new("p")
                    .attr("id", "dialog-description")
                    .text(self.description.clone()),
            );
        }
        Some(
            Element::new("div")
                .class("ferrum-dialog-backdrop")
                .child(panel),
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum DialogAction {
    None,
    Closed,
    PreventDefault,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DialogState {
    pub open: bool,
    pub restore_focus: Option<String>,
    pub scope_id: String,
    pub focusables: Vec<String>,
}

impl Default for DialogState {
    fn default() -> Self {
        Self {
            open: false,
            restore_focus: None,
            scope_id: "dialog".to_owned(),
            focusables: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct DialogController {
    pub state: DialogState,
    pub focus: FocusManager,
}

impl DialogController {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn open(&mut self, trigger: Option<String>, focusables: Vec<String>) {
        self.state.open = true;
        self.state.restore_focus = trigger.clone();
        self.state.focusables = focusables.clone();
        self.focus.push(FocusScope {
            id: self.state.scope_id.clone(),
            restore: trigger,
            focusables,
        });
    }

    pub fn close(&mut self) -> bool {
        if !self.state.open {
            return false;
        }
        self.state.open = false;
        if let Some(scope) = self.focus.pop() {
            self.state.restore_focus = scope.restore;
        }
        true
    }

    pub fn handle_key(&mut self, props: &DialogProps, event: &KeyEvent) -> DialogAction {
        if !self.state.open {
            return DialogAction::None;
        }
        if event.is_escape() && props.close_on_escape {
            self.close();
            return DialogAction::Closed;
        }
        if event.is_tab() {
            self.focus.next(!event.shift);
            return DialogAction::PreventDefault;
        }
        DialogAction::None
    }

    pub fn handle_outside_click(&mut self, props: &DialogProps) -> bool {
        self.state.open && props.close_on_outside && self.close()
    }

    pub fn aria(&self, props: &DialogProps) -> Aria {
        Aria::new()
            .role(Role::Dialog)
            .label(props.title.clone())
            .modal(props.modal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ferrum_ui_core::event::Key;

    #[test]
    fn escape_closes_and_restores_trigger() {
        let mut controller = DialogController::new();
        controller.open(Some("trigger".into()), vec!["close".into(), "input".into()]);
        assert_eq!(
            controller.handle_key(&DialogProps::default(), &KeyEvent::new(Key::Escape)),
            DialogAction::Closed
        );
        assert!(!controller.state.open);
        assert_eq!(controller.state.restore_focus.as_deref(), Some("trigger"));
    }

    #[test]
    fn tab_is_trapped_in_dialog() {
        let mut controller = DialogController::new();
        controller.open(None, vec!["a".into(), "b".into()]);
        assert_eq!(
            controller.handle_key(&DialogProps::default(), &KeyEvent::new(Key::Tab)),
            DialogAction::PreventDefault
        );
        assert_eq!(controller.focus.active(), Some("b"));
    }

    #[test]
    fn outside_click_closes_when_enabled() {
        let mut controller = DialogController::new();
        controller.open(None, vec!["a".into()]);
        assert!(controller.handle_outside_click(&DialogProps::default()));
        assert!(!controller.state.open);
    }
}
