use std::fmt;

/// Ordered, de-duplicated class list.
///
/// Later merges win only for duplicate tokens; class order is preserved so
/// generated CSS and visual regression snapshots stay deterministic.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Class {
    items: Vec<String>,
}

impl Class {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add<S: Into<String>>(&mut self, class: S) -> &mut Self {
        let class = class.into();
        if !class.is_empty() && !self.items.iter().any(|item| item == &class) {
            self.items.push(class);
        }
        self
    }

    pub fn extend<I, S>(&mut self, classes: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        for class in classes {
            self.add(class);
        }
        self
    }

    pub fn merge(&mut self, other: &Self) -> &mut Self {
        for class in &other.items {
            self.add(class.clone());
        }
        self
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &String> {
        self.items.iter()
    }

    pub fn as_slice(&self) -> &[String] {
        &self.items
    }
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.items.join(" "))
    }
}

impl From<&str> for Class {
    fn from(value: &str) -> Self {
        let mut class = Self::new();
        class.extend(value.split_whitespace().map(str::to_owned));
        class
    }
}

impl From<String> for Class {
    fn from(value: String) -> Self {
        Self::from(value.as_str())
    }
}

impl FromIterator<String> for Class {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        let mut class = Self::new();
        class.extend(iter);
        class
    }
}

/// Ordered CSS declarations with replace-on-merge semantics.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Style {
    decls: Vec<(String, String)>,
}

impl Style {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set(&mut self, property: impl Into<String>, value: impl Into<String>) -> &mut Self {
        let property = property.into();
        let value = value.into();
        if let Some((_, current)) = self.decls.iter_mut().find(|(key, _)| key == &property) {
            *current = value;
        } else {
            self.decls.push((property, value));
        }
        self
    }

    pub fn remove(&mut self, property: &str) -> &mut Self {
        self.decls.retain(|(key, _)| key != property);
        self
    }

    pub fn merge(&mut self, other: &Self) -> &mut Self {
        for (property, value) in &other.decls {
            self.set(property.clone(), value.clone());
        }
        self
    }

    pub fn get(&self, property: &str) -> Option<&str> {
        self.decls
            .iter()
            .find(|(key, _)| key == property)
            .map(|(_, value)| value.as_str())
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &String)> {
        self.decls.iter().map(|(key, value)| (key, value))
    }

    pub fn is_empty(&self) -> bool {
        self.decls.is_empty()
    }

    pub fn len(&self) -> usize {
        self.decls.len()
    }

    pub fn to_css(&self) -> String {
        self.decls
            .iter()
            .map(|(property, value)| format!("{property}: {value}"))
            .collect::<Vec<_>>()
            .join("; ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn class_merge_dedupes_and_preserves_order() {
        let mut class = Class::from("a b");
        class.add("b").add("c").merge(&Class::from("c d"));
        assert_eq!(class.to_string(), "a b c d");
    }

    #[test]
    fn style_merge_replaces_value() {
        let mut style = Style::new();
        style.set("color", "red").set("display", "flex");
        let mut override_style = Style::new();
        override_style.set("color", "blue");
        style.merge(&override_style);
        assert_eq!(style.get("color"), Some("blue"));
        assert_eq!(style.to_css(), "color: blue; display: flex");
    }
}
