//! Minimal HTML renderer used by SSR, previews and visual tests.

use crate::style::{Class, Style};

pub fn escape_html(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

pub fn escape_attr(input: &str) -> String {
    escape_html(input).replace('"', "&quot;")
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Element {
    pub tag: String,
    pub class: Class,
    pub style: Style,
    pub attrs: Vec<(String, String)>,
    pub children: Vec<Node>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum Node {
    Element(Element),
    Text(String),
}

impl Element {
    pub fn new(tag: impl Into<String>) -> Self {
        Self {
            tag: tag.into(),
            class: Class::new(),
            style: Style::new(),
            attrs: Vec::new(),
            children: Vec::new(),
        }
    }

    pub fn class(mut self, class: impl Into<Class>) -> Self {
        self.class = class.into();
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn attr(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.attrs.push((name.into(), value.into()));
        self
    }

    pub fn child(mut self, child: impl Into<Node>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn text(self, text: impl Into<String>) -> Self {
        self.child(Node::Text(text.into()))
    }

    pub fn render(&self) -> String {
        let mut out = String::new();
        out.push('<');
        out.push_str(&self.tag);
        if !self.class.is_empty() {
            out.push_str(" class=\"");
            out.push_str(&escape_attr(&self.class.to_string()));
            out.push('"');
        }
        if !self.style.is_empty() {
            out.push_str(" style=\"");
            out.push_str(&escape_attr(&self.style.to_css()));
            out.push('"');
        }
        for (name, value) in &self.attrs {
            out.push(' ');
            out.push_str(name);
            out.push_str("=\"");
            out.push_str(&escape_attr(value));
            out.push('"');
        }
        out.push('>');
        for child in &self.children {
            out.push_str(&child.render());
        }
        out.push_str("</");
        out.push_str(&self.tag);
        out.push('>');
        out
    }
}

impl Node {
    pub fn render(&self) -> String {
        match self {
            Self::Element(element) => element.render(),
            Self::Text(text) => escape_html(text),
        }
    }
}

impl From<Element> for Node {
    fn from(element: Element) -> Self {
        Self::Element(element)
    }
}

impl From<String> for Node {
    fn from(text: String) -> Self {
        Self::Text(text)
    }
}

impl From<&str> for Node {
    fn from(text: &str) -> Self {
        Self::Text(text.to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escapes_text_and_attributes() {
        let element = Element::new("button")
            .class(Class::from("a b"))
            .attr("aria-label", "\"quoted\" & <tag>")
            .text("<script>");
        assert_eq!(
            element.render(),
            "<button class=\"a b\" aria-label=\"&quot;quoted&quot; &amp; &lt;tag&gt;\">&lt;script&gt;</button>"
        );
    }
}
