//! Focus-scope stack used by dialogs, menus and popovers.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FocusScope {
    pub id: String,
    pub restore: Option<String>,
    pub focusables: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct FocusManager {
    stack: Vec<FocusScope>,
    active: Option<String>,
}

impl FocusManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, scope: FocusScope) {
        if let Some(index) = self.stack.iter().position(|item| item.id == scope.id) {
            self.stack.remove(index);
        }
        self.active = scope.focusables.first().cloned();
        self.stack.push(scope);
    }

    pub fn pop(&mut self) -> Option<FocusScope> {
        let scope = self.stack.pop();
        self.active = self
            .stack
            .last()
            .and_then(|item| item.focusables.first())
            .cloned();
        scope
    }

    pub fn active(&self) -> Option<&str> {
        self.active.as_deref()
    }

    pub fn set_active(&mut self, id: impl Into<String>) -> bool {
        let id = id.into();
        if self.contains(&id) {
            self.active = Some(id);
            true
        } else {
            false
        }
    }

    pub fn current(&self) -> Option<&FocusScope> {
        self.stack.last()
    }

    pub fn contains(&self, id: &str) -> bool {
        self.stack
            .last()
            .is_some_and(|scope| scope.focusables.iter().any(|item| item == id))
    }

    pub fn next(&mut self, forward: bool) -> Option<String> {
        let next = {
            let scope = self.stack.last()?;
            if scope.focusables.is_empty() {
                return None;
            }
            let len = scope.focusables.len();
            let current = self
                .active
                .as_deref()
                .and_then(|id| scope.focusables.iter().position(|item| item == id));
            let index = match current {
                Some(index) if forward => (index + 1) % len,
                Some(index) => (index + len - 1) % len,
                None if forward => 0,
                None => len - 1,
            };
            scope.focusables[index].clone()
        };
        self.active = Some(next.clone());
        Some(next)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scope(id: &str, focusables: &[&str]) -> FocusScope {
        FocusScope {
            id: id.into(),
            restore: None,
            focusables: focusables.iter().map(|item| (*item).into()).collect(),
        }
    }

    #[test]
    fn tab_wraps_within_top_scope_and_restores_on_pop() {
        let mut manager = FocusManager::new();
        manager.push(scope("dialog", &["a", "b"]));
        assert_eq!(manager.active(), Some("a"));
        assert_eq!(manager.next(true).as_deref(), Some("b"));
        assert_eq!(manager.next(true).as_deref(), Some("a"));
        assert_eq!(manager.next(false).as_deref(), Some("b"));
        assert_eq!(manager.pop().map(|item| item.id).as_deref(), Some("dialog"));
        assert_eq!(manager.active(), None);
    }

    #[test]
    fn nested_scopes_restore_parent_focus() {
        let mut manager = FocusManager::new();
        manager.push(scope("dialog", &["a", "b"]));
        manager.push(scope("menu", &["x", "y"]));
        manager.pop();
        assert_eq!(manager.active(), Some("a"));
    }
}
