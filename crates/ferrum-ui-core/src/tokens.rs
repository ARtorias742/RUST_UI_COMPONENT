//! Semantic design tokens.
//!
//! Tokens are emitted as CSS custom properties for web/SSR and converted to
//! native style properties by the native adapter. Dark and RTL are token sets
//! (direction is carried by [`Theme`]), not duplicated component code.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Scheme {
    Light,
    Dark,
}

impl Scheme {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Light => "light",
            Self::Dark => "dark",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Direction {
    Ltr,
    Rtl,
}

impl Direction {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ltr => "ltr",
            Self::Rtl => "rtl",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token {
    pub light: &'static str,
    pub dark: &'static str,
}

impl Token {
    pub const fn new(light: &'static str, dark: &'static str) -> Self {
        Self { light, dark }
    }

    pub fn value(self, scheme: Scheme) -> &'static str {
        match scheme {
            Scheme::Light => self.light,
            Scheme::Dark => self.dark,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Tokens {
    pub bg: Token,
    pub fg: Token,
    pub muted: Token,
    pub muted_fg: Token,
    pub accent: Token,
    pub accent_fg: Token,
    pub border: Token,
    pub danger: Token,
    pub ring: Token,
    pub radius: Token,
    pub density: Token,
}

impl Tokens {
    pub const LIGHT: Self = Self {
        bg: Token::new("0 0% 100%", "240 10% 3.9%"),
        fg: Token::new("240 10% 3.9%", "0 0% 98%"),
        muted: Token::new("240 4.8% 95.9%", "240 3.7% 15.9%"),
        muted_fg: Token::new("240 3.8% 46.1%", "240 5% 64.9%"),
        accent: Token::new("240 4.8% 95.9%", "240 3.7% 15.9%"),
        accent_fg: Token::new("240 5.9% 10%", "0 0% 98%"),
        border: Token::new("240 5.9% 90%", "240 3.7% 15.9%"),
        danger: Token::new("0 72% 51%", "0 62.8% 45%"),
        ring: Token::new("240 5% 64.9%", "240 4.9% 83.9%"),
        radius: Token::new("0.5rem", "0.5rem"),
        density: Token::new("1", "1"),
    };

    pub const DARK: Self = Self {
        bg: Token::new("0 0% 100%", "240 10% 3.9%"),
        fg: Token::new("240 10% 3.9%", "0 0% 98%"),
        muted: Token::new("240 4.8% 95.9%", "240 3.7% 15.9%"),
        muted_fg: Token::new("240 3.8% 46.1%", "240 5% 64.9%"),
        accent: Token::new("240 4.8% 95.9%", "240 3.7% 15.9%"),
        accent_fg: Token::new("240 5.9% 10%", "0 0% 98%"),
        border: Token::new("240 5.9% 90%", "240 3.7% 15.9%"),
        danger: Token::new("0 72% 51%", "0 62.8% 45%"),
        ring: Token::new("240 5% 64.9%", "240 4.9% 83.9%"),
        radius: Token::new("0.5rem", "0.5rem"),
        density: Token::new("1", "1"),
    };

    pub const fn for_scheme(scheme: Scheme) -> Self {
        match scheme {
            Scheme::Light => Self::LIGHT,
            Scheme::Dark => Self::DARK,
        }
    }

    pub fn pairs(&self) -> [(&'static str, Token); 11] {
        [
            ("bg", self.bg),
            ("fg", self.fg),
            ("muted", self.muted),
            ("muted-fg", self.muted_fg),
            ("accent", self.accent),
            ("accent-fg", self.accent_fg),
            ("border", self.border),
            ("danger", self.danger),
            ("ring", self.ring),
            ("radius", self.radius),
            ("density", self.density),
        ]
    }

    pub fn css_vars(&self, scheme: Scheme) -> String {
        let mut out = String::from(":root{");
        for (name, token) in self.pairs() {
            out.push_str("--");
            out.push_str(name);
            out.push_str(": ");
            out.push_str(token.value(scheme));
            out.push(';');
        }
        out.push('}');
        out
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Theme {
    pub tokens: Tokens,
    pub scheme: Scheme,
    pub direction: Direction,
}

impl Theme {
    pub const LIGHT: Self = Self {
        tokens: Tokens::LIGHT,
        scheme: Scheme::Light,
        direction: Direction::Ltr,
    };

    pub const DARK: Self = Self {
        tokens: Tokens::DARK,
        scheme: Scheme::Dark,
        direction: Direction::Ltr,
    };

    pub const RTL: Self = Self {
        tokens: Tokens::LIGHT,
        scheme: Scheme::Light,
        direction: Direction::Rtl,
    };

    pub fn with_direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }

    pub fn css_vars(&self) -> String {
        self.tokens.css_vars(self.scheme)
    }

    pub fn data_attrs(&self) -> [(&'static str, &'static str); 2] {
        [
            ("data-theme", self.scheme.as_str()),
            ("dir", self.direction.as_str()),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn light_and_dark_emit_all_semantic_tokens() {
        let light = Tokens::LIGHT.css_vars(Scheme::Light);
        let dark = Tokens::DARK.css_vars(Scheme::Dark);
        assert!(light.contains("--bg: 0 0% 100%;"));
        assert!(dark.contains("--bg: 240 10% 3.9%;"));
        assert_eq!(light.matches("--").count(), 11);
    }

    #[test]
    fn theme_direction_is_independent_of_scheme() {
        let theme = Theme::DARK.with_direction(Direction::Rtl);
        assert_eq!(theme.data_attrs(), [("data-theme", "dark"), ("dir", "rtl")]);
    }
}
