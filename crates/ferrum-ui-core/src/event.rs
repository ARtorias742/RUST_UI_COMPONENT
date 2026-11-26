//! Renderer-neutral keyboard model.

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[non_exhaustive]
pub enum Key {
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    Home,
    End,
    Escape,
    Enter,
    Space,
    Tab,
    Backspace,
    Char(char),
    #[default]
    Unknown,
}

impl Key {
    pub fn from_char(value: char) -> Self {
        match value {
            ' ' => Self::Space,
            other => Self::Char(other),
        }
    }

    pub fn is_printable(self) -> bool {
        matches!(self, Self::Char(_) | Self::Space)
    }

    pub fn is_navigation(self) -> bool {
        matches!(
            self,
            Self::ArrowUp
                | Self::ArrowDown
                | Self::ArrowLeft
                | Self::ArrowRight
                | Self::Home
                | Self::End
        )
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct KeyEvent {
    pub key: Key,
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub meta: bool,
}

impl KeyEvent {
    pub const fn new(key: Key) -> Self {
        Self {
            key,
            shift: false,
            ctrl: false,
            alt: false,
            meta: false,
        }
    }

    pub const fn with_shift(mut self, shift: bool) -> Self {
        self.shift = shift;
        self
    }

    pub const fn with_ctrl(mut self, ctrl: bool) -> Self {
        self.ctrl = ctrl;
        self
    }

    pub const fn with_alt(mut self, alt: bool) -> Self {
        self.alt = alt;
        self
    }

    pub const fn with_meta(mut self, meta: bool) -> Self {
        self.meta = meta;
        self
    }

    pub fn char(self) -> Option<char> {
        match self.key {
            Key::Char(value) => Some(value),
            Key::Space => Some(' '),
            _ => None,
        }
    }

    pub fn is_printable(self) -> bool {
        !self.ctrl && !self.meta && self.key.is_printable()
    }
}

/// Convenience predicates shared by Ferrum renderer adapters.
pub trait FerrumEventExt {
    fn key(&self) -> Key;
    fn is_escape(&self) -> bool;
    fn is_tab(&self) -> bool;
    fn is_enter(&self) -> bool;
    fn is_space(&self) -> bool;
    fn char(&self) -> Option<char>;
}

impl FerrumEventExt for KeyEvent {
    fn key(&self) -> Key {
        self.key
    }

    fn is_escape(&self) -> bool {
        self.key == Key::Escape
    }

    fn is_tab(&self) -> bool {
        self.key == Key::Tab
    }

    fn is_enter(&self) -> bool {
        self.key == Key::Enter
    }

    fn is_space(&self) -> bool {
        self.key == Key::Space
    }

    fn char(&self) -> Option<char> {
        KeyEvent::char(*self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn printable_and_navigation_are_classified() {
        assert!(KeyEvent::new(Key::Char('a')).is_printable());
        assert!(!KeyEvent::new(Key::Char('a')).with_ctrl(true).is_printable());
        assert!(KeyEvent::new(Key::ArrowDown).key().is_navigation());
        assert_eq!(KeyEvent::new(Key::Space).char(), Some(' '));
    }
}
