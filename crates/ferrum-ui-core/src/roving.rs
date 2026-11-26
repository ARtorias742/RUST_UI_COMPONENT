//! Roving tabindex helper for menus, tabs, toolbars and radio groups.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Orientation {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RovingTabindex {
    items: Vec<String>,
    active: usize,
    orientation: Orientation,
    wrap: bool,
}

impl RovingTabindex {
    pub fn new(items: Vec<String>, orientation: Orientation) -> Self {
        Self {
            items,
            active: 0,
            orientation,
            wrap: true,
        }
    }

    pub fn with_wrap(mut self, wrap: bool) -> Self {
        self.wrap = wrap;
        self
    }

    pub fn items(&self) -> &[String] {
        &self.items
    }

    pub fn orientation(&self) -> Orientation {
        self.orientation
    }

    pub fn active_index(&self) -> Option<usize> {
        (!self.items.is_empty()).then_some(self.active)
    }

    pub fn active_id(&self) -> Option<&str> {
        self.items.get(self.active).map(String::as_str)
    }

    pub fn set_active(&mut self, index: usize) -> bool {
        if index < self.items.len() {
            self.active = index;
            true
        } else {
            false
        }
    }

    pub fn tabindex_for(&self, index: usize) -> &'static str {
        if index == self.active {
            "0"
        } else {
            "-1"
        }
    }

    pub fn move_by(&mut self, delta: isize) -> Option<String> {
        if self.items.is_empty() {
            return None;
        }
        let len = self.items.len() as isize;
        let next = if self.wrap {
            (self.active as isize + delta).rem_euclid(len) as usize
        } else {
            (self.active as isize + delta).clamp(0, len - 1) as usize
        };
        self.active = next;
        Some(self.items[next].clone())
    }

    pub fn next_item(&mut self) -> Option<String> {
        self.move_by(1)
    }

    pub fn prev_item(&mut self) -> Option<String> {
        self.move_by(-1)
    }

    pub fn home(&mut self) -> Option<String> {
        if self.items.is_empty() {
            return None;
        }
        self.active = 0;
        Some(self.items[0].clone())
    }

    pub fn end(&mut self) -> Option<String> {
        self.active = self.items.len().checked_sub(1)?;
        Some(self.items[self.active].clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roving_wraps_and_reports_tabindex() {
        let mut roving = RovingTabindex::new(vec!["a".into(), "b".into()], Orientation::Horizontal);
        assert_eq!(roving.tabindex_for(0), "0");
        assert_eq!(roving.next_item().as_deref(), Some("b"));
        assert_eq!(roving.next_item().as_deref(), Some("a"));
        assert_eq!(roving.end().as_deref(), Some("b"));
        assert_eq!(roving.home().as_deref(), Some("a"));
    }

    #[test]
    fn non_wrapping_clamps_at_edges() {
        let mut roving = RovingTabindex::new(vec!["a".into(), "b".into()], Orientation::Vertical)
            .with_wrap(false);
        assert_eq!(roving.prev_item().as_deref(), Some("a"));
        assert_eq!(roving.next_item().as_deref(), Some("b"));
        assert_eq!(roving.next_item().as_deref(), Some("b"));
    }
}
