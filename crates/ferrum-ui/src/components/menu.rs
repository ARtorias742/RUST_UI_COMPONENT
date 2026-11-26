use ferrum_ui_core::a11y::{Aria, Role};
use ferrum_ui_core::event::{Key, KeyEvent};
use ferrum_ui_core::roving::{Orientation, RovingTabindex};
use ferrum_ui_core::ssr::Element;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[non_exhaustive]
pub enum MenuItemKind {
    #[default]
    Item,
    Disabled,
    Separator,
    Label,
}

impl MenuItemKind {
    pub fn is_focusable(self) -> bool {
        self == Self::Item
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MenuItem {
    pub id: String,
    pub label: String,
    pub kind: MenuItemKind,
    pub shortcut: Option<String>,
}

impl MenuItem {
    pub fn item(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            kind: MenuItemKind::Item,
            shortcut: None,
        }
    }

    pub fn disabled(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            kind: MenuItemKind::Disabled,
            ..Self::item(id, label)
        }
    }

    pub fn separator(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: String::new(),
            kind: MenuItemKind::Separator,
            shortcut: None,
        }
    }

    pub fn with_shortcut(mut self, shortcut: impl Into<String>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MenuProps {
    pub items: Vec<MenuItem>,
    pub orientation: Orientation,
    pub wrap: bool,
    pub typeahead: bool,
    pub open: bool,
}

impl Default for MenuProps {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            orientation: Orientation::Horizontal,
            wrap: true,
            typeahead: true,
            open: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum MenuAction {
    None,
    Closed,
    Selected(String),
    Moved(Option<String>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MenuState {
    pub open: bool,
    pub roving: RovingTabindex,
    pub query: String,
    pub active: Option<String>,
}

impl MenuState {
    pub fn new(props: &MenuProps) -> Self {
        let items = props
            .items
            .iter()
            .filter(|item| item.kind.is_focusable())
            .map(|item| item.id.clone())
            .collect();
        Self {
            open: props.open,
            roving: RovingTabindex::new(items, props.orientation).with_wrap(props.wrap),
            query: String::new(),
            active: None,
        }
    }

    pub fn reset_typeahead(&mut self) {
        self.query.clear();
    }

    pub fn handle_outside_click(&mut self) -> bool {
        if self.open {
            self.open = false;
            self.query.clear();
            true
        } else {
            false
        }
    }

    pub fn typeahead(&mut self, items: &[MenuItem], value: char) -> Option<String> {
        if value.is_control() {
            return None;
        }
        self.query.push(value);
        let query = self.query.to_lowercase();
        let focusable: Vec<&MenuItem> = items
            .iter()
            .filter(|item| item.kind.is_focusable())
            .collect();
        let index = focusable
            .iter()
            .position(|item| item.label.to_lowercase().starts_with(&query))?;
        self.roving.set_active(index);
        self.active = self.roving.active_id().map(str::to_owned);
        self.active.clone()
    }

    pub fn handle_key(&mut self, props: &MenuProps, event: &KeyEvent) -> MenuAction {
        if !self.open {
            return MenuAction::None;
        }
        match event.key {
            Key::Escape => {
                self.open = false;
                self.query.clear();
                MenuAction::Closed
            }
            Key::ArrowDown | Key::ArrowRight => {
                let next = self.roving.next_item();
                self.active = next.clone();
                MenuAction::Moved(next)
            }
            Key::ArrowUp | Key::ArrowLeft => {
                let next = self.roving.prev_item();
                self.active = next.clone();
                MenuAction::Moved(next)
            }
            Key::Home => {
                let next = self.roving.home();
                self.active = next.clone();
                MenuAction::Moved(next)
            }
            Key::End => {
                let next = self.roving.end();
                self.active = next.clone();
                MenuAction::Moved(next)
            }
            Key::Enter | Key::Space => {
                let id = self
                    .active
                    .clone()
                    .or_else(|| self.roving.active_id().map(str::to_owned));
                match id {
                    Some(id) => MenuAction::Selected(id),
                    None => MenuAction::None,
                }
            }
            Key::Char(value) if props.typeahead && event.is_printable() => {
                let next = self.typeahead(&props.items, value);
                MenuAction::Moved(next)
            }
            _ => MenuAction::None,
        }
    }

    pub fn aria(&self) -> Aria {
        Aria::new().role(Role::Menu).set(
            "aria-orientation",
            match self.roving.orientation() {
                Orientation::Horizontal => "horizontal",
                Orientation::Vertical => "vertical",
                _ => "horizontal",
            },
        )
    }

    pub fn element(&self, props: &MenuProps) -> Element {
        let mut list = Element::new("div")
            .class("ferrum-menu")
            .attr("role", "menu")
            .attr(
                "aria-orientation",
                match self.roving.orientation() {
                    Orientation::Horizontal => "horizontal",
                    Orientation::Vertical => "vertical",
                    _ => "horizontal",
                },
            );
        let mut focus_index = 0;
        for item in &props.items {
            match item.kind {
                MenuItemKind::Separator => {
                    list = list.child(
                        Element::new("div")
                            .class("ferrum-menu-separator")
                            .attr("role", "separator"),
                    );
                }
                MenuItemKind::Label => {
                    list = list.child(
                        Element::new("div")
                            .class("ferrum-menu-label")
                            .attr("role", "presentation")
                            .text(item.label.clone()),
                    );
                }
                MenuItemKind::Item | MenuItemKind::Disabled => {
                    let tabindex = if item.kind == MenuItemKind::Item {
                        let tabindex = self.roving.tabindex_for(focus_index);
                        focus_index += 1;
                        tabindex
                    } else {
                        "-1"
                    };
                    let mut element = Element::new("button")
                        .class("ferrum-menu-item")
                        .attr("role", "menuitem")
                        .attr("type", "button")
                        .attr("tabindex", tabindex)
                        .text(item.label.clone());
                    if item.kind == MenuItemKind::Disabled {
                        element = element.attr("aria-disabled", "true");
                    }
                    list = list.child(element);
                }
            }
        }
        list
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn props() -> MenuProps {
        MenuProps {
            items: vec![
                MenuItem::item("new", "New"),
                MenuItem::disabled("delete", "Delete"),
                MenuItem::separator("sep"),
                MenuItem::item("open", "Open"),
            ],
            open: true,
            ..MenuProps::default()
        }
    }

    #[test]
    fn roving_skips_disabled_and_separator_items() {
        let state = MenuState::new(&props());
        assert_eq!(state.roving.items(), &["new".to_owned(), "open".to_owned()]);
    }

    #[test]
    fn typeahead_selects_matching_item() {
        let mut state = MenuState::new(&props());
        assert_eq!(
            state.handle_key(&props(), &KeyEvent::new(Key::Char('o'))),
            MenuAction::Moved(Some("open".into()))
        );
    }

    #[test]
    fn escape_closes_menu() {
        let mut state = MenuState::new(&props());
        assert_eq!(
            state.handle_key(&props(), &KeyEvent::new(Key::Escape)),
            MenuAction::Closed
        );
        assert!(!state.open);
    }

    #[test]
    fn outside_click_closes_menu() {
        let mut state = MenuState::new(&props());
        assert!(state.handle_outside_click());
        assert!(!state.open);
    }
}
