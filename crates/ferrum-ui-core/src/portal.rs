//! Portal host bookkeeping for overlays.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Portal {
    pub id: String,
    pub name: String,
    pub open: bool,
}

#[derive(Debug, Clone, Default)]
pub struct PortalHost {
    portals: Vec<Portal>,
}

impl PortalHost {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn mount(&mut self, id: impl Into<String>, name: impl Into<String>) -> &mut Self {
        let id = id.into();
        let name = name.into();
        if let Some(portal) = self.portals.iter_mut().find(|item| item.id == id) {
            portal.name = name;
            portal.open = true;
        } else {
            self.portals.push(Portal {
                id,
                name,
                open: true,
            });
        }
        self
    }

    pub fn unmount(&mut self, id: &str) -> bool {
        let before = self.portals.len();
        self.portals.retain(|portal| portal.id != id);
        before != self.portals.len()
    }

    pub fn set_open(&mut self, id: &str, open: bool) -> bool {
        if let Some(portal) = self.portals.iter_mut().find(|item| item.id == id) {
            portal.open = open;
            true
        } else {
            false
        }
    }

    pub fn get(&self, id: &str) -> Option<&Portal> {
        self.portals.iter().find(|portal| portal.id == id)
    }

    pub fn top(&self) -> Option<&Portal> {
        self.portals.iter().rev().find(|portal| portal.open)
    }

    pub fn len(&self) -> usize {
        self.portals.len()
    }

    pub fn is_empty(&self) -> bool {
        self.portals.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Portal> {
        self.portals.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn top_returns_last_open_portal() {
        let mut host = PortalHost::new();
        host.mount("dialog", "dialog");
        host.mount("menu", "menu");
        host.set_open("menu", false);
        assert_eq!(host.top().map(|portal| portal.id.as_str()), Some("dialog"));
        assert_eq!(host.len(), 2);
    }
}
