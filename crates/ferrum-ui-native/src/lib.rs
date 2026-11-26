//! Native adapter primitives.
//!
//! Web tokens and CSS declarations are converted to a native property list;
//! [`parity`] reports drift between the two representations so native and web
//! cannot silently diverge.

use ferrum_ui_core::style::Style;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct NativeStyle {
    props: Vec<(String, String)>,
}

impl NativeStyle {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set(&mut self, property: impl Into<String>, value: impl Into<String>) -> &mut Self {
        let property = property.into();
        let value = value.into();
        if let Some((_, current)) = self.props.iter_mut().find(|(key, _)| key == &property) {
            *current = value;
        } else {
            self.props.push((property, value));
        }
        self
    }

    pub fn get(&self, property: &str) -> Option<&str> {
        self.props
            .iter()
            .find(|(key, _)| key == property)
            .map(|(_, value)| value.as_str())
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &String)> {
        self.props.iter().map(|(key, value)| (key, value))
    }

    pub fn len(&self) -> usize {
        self.props.len()
    }

    pub fn is_empty(&self) -> bool {
        self.props.is_empty()
    }
}

impl From<&Style> for NativeStyle {
    fn from(style: &Style) -> Self {
        let mut native = Self::new();
        for (property, value) in style.iter() {
            native.set(property.clone(), value.clone());
        }
        native
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SceneNode {
    pub kind: String,
    pub style: NativeStyle,
    pub children: Vec<SceneNode>,
}

impl SceneNode {
    pub fn new(kind: impl Into<String>) -> Self {
        Self {
            kind: kind.into(),
            style: NativeStyle::new(),
            children: Vec::new(),
        }
    }

    pub fn with_style(mut self, style: NativeStyle) -> Self {
        self.style = style;
        self
    }

    pub fn child(mut self, child: Self) -> Self {
        self.children.push(child);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ParityIssue {
    Missing(String),
    Extra(String),
    ValueMismatch {
        property: String,
        web: String,
        native: String,
    },
}

pub fn parity(web: &Style, native: &NativeStyle) -> Vec<ParityIssue> {
    let mut issues = Vec::new();
    for (property, web_value) in web.iter() {
        match native.get(property) {
            None => issues.push(ParityIssue::Missing(property.clone())),
            Some(native_value) if native_value != web_value => {
                issues.push(ParityIssue::ValueMismatch {
                    property: property.clone(),
                    web: web_value.clone(),
                    native: native_value.to_owned(),
                })
            }
            Some(_) => {}
        }
    }
    for (property, _) in native.iter() {
        if web.get(property).is_none() {
            issues.push(ParityIssue::Extra(property.clone()));
        }
    }
    issues
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identical_styles_have_parity() {
        let mut web = Style::new();
        web.set("color", "red").set("display", "flex");
        let native = NativeStyle::from(&web);
        assert!(parity(&web, &native).is_empty());
    }

    #[test]
    fn missing_and_mismatched_properties_are_reported() {
        let mut web = Style::new();
        web.set("color", "red").set("display", "flex");
        let mut native = NativeStyle::new();
        native.set("color", "blue").set("padding", "0");
        let issues = parity(&web, &native);
        assert!(issues.contains(&ParityIssue::Missing("display".into())));
        assert!(issues.contains(&ParityIssue::Extra("padding".into())));
        assert!(issues.iter().any(|issue| matches!(
            issue,
            ParityIssue::ValueMismatch { property, web, native }
                if property == "color" && web == "red" && native == "blue"
        )));
    }
}
