//! Template registry and `ferrum add`-style source distribution.

use std::collections::BTreeMap;
use std::fmt;
use std::path::{Component, Path, PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TemplateFile {
    pub path: String,
    pub contents: String,
}

impl TemplateFile {
    pub fn new(path: impl Into<String>, contents: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            contents: contents.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Manifest {
    pub schema_version: u32,
    pub name: String,
    pub version: String,
    #[serde(default)]
    pub dependencies: Vec<String>,
    #[serde(default)]
    pub deprecated: bool,
    pub files: Vec<TemplateFile>,
}

impl Manifest {
    pub fn new(
        name: impl Into<String>,
        version: impl Into<String>,
        files: Vec<TemplateFile>,
    ) -> Self {
        Self {
            schema_version: 1,
            name: name.into(),
            version: version.into(),
            dependencies: vec!["ferrum-ui".to_owned()],
            deprecated: false,
            files,
        }
    }

    pub fn deprecate(mut self, deprecated: bool) -> Self {
        self.deprecated = deprecated;
        self
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Registry {
    templates: BTreeMap<String, Manifest>,
}

impl Registry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn builtin() -> Self {
        let mut registry = Self::new();
        for manifest in builtin_templates() {
            registry
                .insert(manifest)
                .expect("built-in templates are unique");
        }
        registry
    }

    pub fn insert(&mut self, manifest: Manifest) -> Result<(), RegistryError> {
        if self.templates.contains_key(&manifest.name) {
            return Err(RegistryError::Duplicate(manifest.name));
        }
        self.templates.insert(manifest.name.clone(), manifest);
        Ok(())
    }

    pub fn get(&self, name: &str) -> Option<&Manifest> {
        self.templates.get(name)
    }

    pub fn list(&self) -> Vec<&Manifest> {
        self.templates.values().collect()
    }

    pub fn remove(&mut self, name: &str) -> Option<Manifest> {
        self.templates.remove(name)
    }

    pub fn update(
        &mut self,
        name: &str,
        version: &str,
        files: Vec<TemplateFile>,
    ) -> Result<(), RegistryError> {
        let existing = self
            .templates
            .get(name)
            .ok_or_else(|| RegistryError::NotFound(name.to_owned()))?;
        if !version_gt(version, &existing.version) {
            return Err(RegistryError::OlderVersion {
                current: existing.version.clone(),
                incoming: version.to_owned(),
            });
        }
        let mut manifest = existing.clone();
        manifest.version = version.to_owned();
        manifest.files = files;
        self.templates.insert(name.to_owned(), manifest);
        Ok(())
    }

    pub fn diff_file(&self, name: &str, path: &str, new_contents: &str) -> Option<String> {
        let file = self
            .get(name)?
            .files
            .iter()
            .find(|file| file.path == path)?;
        Some(diff_lines(&file.contents, new_contents))
    }

    pub fn to_json(&self) -> Result<String, RegistryError> {
        Ok(serde_json::to_string_pretty(self)?)
    }

    pub fn from_json(json: &str) -> Result<Self, RegistryError> {
        Ok(serde_json::from_str(json)?)
    }

    pub fn load(path: &Path) -> Result<Self, RegistryError> {
        let json = std::fs::read_to_string(path)?;
        Self::from_json(&json)
    }

    pub fn save(&self, path: &Path) -> Result<(), RegistryError> {
        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                std::fs::create_dir_all(parent)?;
            }
        }
        std::fs::write(path, self.to_json()?)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AddReport {
    pub written: Vec<PathBuf>,
    pub skipped: Vec<PathBuf>,
}

#[derive(Debug)]
#[non_exhaustive]
pub enum RegistryError {
    NotFound(String),
    Duplicate(String),
    OlderVersion { current: String, incoming: String },
    UnsafePath(String),
    Io(std::io::Error),
    Json(serde_json::Error),
}

impl fmt::Display for RegistryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound(name) => write!(f, "component `{name}` is not in the registry"),
            Self::Duplicate(name) => write!(f, "component `{name}` already exists"),
            Self::OlderVersion { current, incoming } => {
                write!(f, "version {incoming} is not newer than {current}")
            }
            Self::UnsafePath(path) => write!(f, "unsafe template path `{path}`"),
            Self::Io(error) => write!(f, "io error: {error}"),
            Self::Json(error) => write!(f, "json error: {error}"),
        }
    }
}

impl std::error::Error for RegistryError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            Self::Json(error) => Some(error),
            _ => None,
        }
    }
}

impl From<std::io::Error> for RegistryError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<serde_json::Error> for RegistryError {
    fn from(error: serde_json::Error) -> Self {
        Self::Json(error)
    }
}

/// Materialise a template into `dest`, refusing traversal and overwriting by default.
pub fn add_component(
    registry: &Registry,
    name: &str,
    dest: &Path,
    force: bool,
) -> Result<AddReport, RegistryError> {
    let manifest = registry
        .get(name)
        .ok_or_else(|| RegistryError::NotFound(name.to_owned()))?;
    let mut report = AddReport::default();
    for file in &manifest.files {
        let relative = Path::new(&file.path);
        if relative.is_absolute()
            || relative.components().any(|component| {
                matches!(
                    component,
                    Component::ParentDir | Component::RootDir | Component::Prefix(_)
                )
            })
        {
            return Err(RegistryError::UnsafePath(file.path.clone()));
        }
        let path = dest.join(relative);
        if path.exists() && !force {
            report.skipped.push(path);
            continue;
        }
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&path, &file.contents)?;
        report.written.push(path);
    }
    Ok(report)
}

pub fn parse_version(version: &str) -> Option<(u64, u64, u64)> {
    let mut parts = version.split('.');
    let major = parts.next()?.parse().ok()?;
    let minor = parts.next()?.parse().ok()?;
    let patch = parts.next()?.parse().ok()?;
    if parts.next().is_some() {
        return None;
    }
    Some((major, minor, patch))
}

pub fn version_gt(incoming: &str, current: &str) -> bool {
    match (parse_version(incoming), parse_version(current)) {
        (Some(incoming), Some(current)) => incoming > current,
        _ => false,
    }
}

/// Line diff used by `ferrum diff`; LCS keeps output deterministic.
pub fn diff_lines(old: &str, new: &str) -> String {
    let old: Vec<&str> = old.lines().collect();
    let new: Vec<&str> = new.lines().collect();
    let mut table = vec![vec![0_usize; new.len() + 1]; old.len() + 1];
    for i in (0..old.len()).rev() {
        for j in (0..new.len()).rev() {
            table[i][j] = if old[i] == new[j] {
                table[i + 1][j + 1] + 1
            } else {
                table[i + 1][j].max(table[i][j + 1])
            };
        }
    }
    let mut out = String::new();
    let (mut i, mut j) = (0, 0);
    while i < old.len() && j < new.len() {
        if old[i] == new[j] {
            out.push_str("  ");
            out.push_str(old[i]);
            out.push('\n');
            i += 1;
            j += 1;
        } else if table[i + 1][j] >= table[i][j + 1] {
            out.push_str("- ");
            out.push_str(old[i]);
            out.push('\n');
            i += 1;
        } else {
            out.push_str("+ ");
            out.push_str(new[j]);
            out.push('\n');
            j += 1;
        }
    }
    while i < old.len() {
        out.push_str("- ");
        out.push_str(old[i]);
        out.push('\n');
        i += 1;
    }
    while j < new.len() {
        out.push_str("+ ");
        out.push_str(new[j]);
        out.push('\n');
        j += 1;
    }
    out
}

fn simple_manifest(name: &str, body: &str, css: &str, test: &str) -> Manifest {
    Manifest::new(
        name,
        "0.1.0",
        vec![
            TemplateFile::new(
                format!("src/ui/{name}.rs"),
                format!("// Generated by `ferrum add {name}`.\n{body}"),
            ),
            TemplateFile::new(format!("src/ui/{name}.css"), css),
            TemplateFile::new(format!("src/ui/{name}.test.rs"), test),
        ],
    )
}

fn builtin_templates() -> Vec<Manifest> {
    vec![
        Manifest::new(
            "button",
            "0.1.0",
            vec![
                TemplateFile::new("src/ui/button.rs", BUTTON_RS),
                TemplateFile::new("src/ui/button.css", BUTTON_CSS),
                TemplateFile::new("src/ui/button.test.rs", BUTTON_TEST),
            ],
        ),
        Manifest::new(
            "dialog",
            "0.1.0",
            vec![
                TemplateFile::new("src/ui/dialog.rs", DIALOG_RS),
                TemplateFile::new("src/ui/dialog.css", DIALOG_CSS),
                TemplateFile::new("src/ui/dialog.test.rs", DIALOG_TEST),
            ],
        ),
        Manifest::new(
            "menu",
            "0.1.0",
            vec![
                TemplateFile::new("src/ui/menu.rs", MENU_RS),
                TemplateFile::new("src/ui/menu.css", MENU_CSS),
                TemplateFile::new("src/ui/menu.test.rs", MENU_TEST),
            ],
        ),
        Manifest::new(
            "tabs",
            "0.1.0",
            vec![
                TemplateFile::new("src/ui/tabs.rs", TABS_RS),
                TemplateFile::new("src/ui/tabs.css", TABS_CSS),
                TemplateFile::new("src/ui/tabs.test.rs", TABS_TEST),
            ],
        ),
        Manifest::new(
            "toast",
            "0.1.0",
            vec![
                TemplateFile::new("src/ui/toast.rs", TOAST_RS),
                TemplateFile::new("src/ui/toast.css", TOAST_CSS),
                TemplateFile::new("src/ui/toast.test.rs", TOAST_TEST),
            ],
        ),
        simple_manifest(
            "accordion",
            r##"use ferrum_ui::components::accordion::{AccordionItem, AccordionProps, AccordionState};

pub fn accordion() -> (AccordionState, AccordionProps) {
    let props = AccordionProps {
        items: vec![AccordionItem::new("one", "One", "Panel one")],
        ..AccordionProps::default()
    };
    (AccordionState::new(&props), props)
}
"##,
            r##".ferrum-accordion { display: grid; gap: 4px; }
.ferrum-accordion-trigger { width: 100%; text-align: left; }
"##,
            r##"#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accordion_starts_closed() {
        let (state, _) = accordion();
        assert!(!state.is_open(0));
    }
}
"##,
        ),
        simple_manifest(
            "alert",
            r##"use ferrum_ui::components::alert::AlertProps;

pub fn alert() -> AlertProps {
    AlertProps::new("Saved")
}
"##,
            r##".ferrum-alert { padding: 12px; border: 1px solid hsl(var(--border)); }
"##,
            r##"#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alert_has_alert_role() {
        assert!(alert().element().render().contains("role=\"alert\""));
    }
}
"##,
        ),
        simple_manifest(
            "avatar",
            r##"use ferrum_ui::components::avatar::AvatarProps;

pub fn avatar() -> AvatarProps {
    AvatarProps::new("AB")
}
"##,
            r##".ferrum-avatar { display: inline-grid; place-items: center; border-radius: 999px; }
"##,
            r##"#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn avatar_has_accessible_name() {
        assert!(avatar().element().render().contains("aria-label=\"AB\""));
    }
}
"##,
        ),
        simple_manifest(
            "badge",
            r##"use ferrum_ui::components::badge::BadgeProps;

pub fn badge() -> BadgeProps {
    BadgeProps::new("New")
}
"##,
            r##".ferrum-badge { padding: 2px 8px; border-radius: 999px; }
"##,
            r##"#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn badge_renders_label() {
        assert!(badge().element().render().contains("New"));
    }
}
"##,
        ),
        simple_manifest(
            "card",
            r##"use ferrum_ui::components::card::CardProps;

pub fn card() -> CardProps {
    CardProps::new("Project")
}
"##,
            r##".ferrum-card { border: 1px solid hsl(var(--border)); border-radius: var(--radius); padding: 16px; }
"##,
            r##"#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn card_renders_title() {
        assert!(card().element().render().contains("Project"));
    }
}
"##,
        ),
        simple_manifest(
            "checkbox",
            r##"use ferrum_ui::components::checkbox::{CheckboxProps, CheckboxState};

pub fn checkbox() -> (CheckboxState, CheckboxProps) {
    let props = CheckboxProps::new("Accept");
    (CheckboxState::new(&props), props)
}
"##,
            r##".ferrum-checkbox { display: grid; gap: 4px; }
"##,
            r##"#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checkbox_starts_unchecked() {
        let (state, _) = checkbox();
        assert!(!state.checked);
    }
}
"##,
        ),
        simple_manifest(
            "input",
            r##"use ferrum_ui::components::input::{InputProps, InputState};

pub fn input() -> (InputState, InputProps) {
    let props = InputProps::new("Email");
    (InputState::new(&props), props)
}
"##,
            r##".ferrum-input { padding: 6px 8px; border: 1px solid hsl(var(--border)); }
"##,
            r##"#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_starts_empty() {
        let (state, _) = input();
        assert!(state.value.is_empty());
    }
}
"##,
        ),
        simple_manifest(
            "progress",
            r##"use ferrum_ui::components::progress::ProgressProps;

pub fn progress() -> ProgressProps {
    ProgressProps::new(40)
}
"##,
            r##".ferrum-progress { height: 8px; background: hsl(var(--muted)); }
"##,
            r##"#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn progress_reports_value() {
        assert!(progress().element().render().contains("aria-valuenow=\"40\""));
    }
}
"##,
        ),
        simple_manifest(
            "separator",
            r##"use ferrum_ui::components::separator::SeparatorProps;

pub fn separator() -> SeparatorProps {
    SeparatorProps::new()
}
"##,
            r##".ferrum-separator { background: hsl(var(--border)); }
"##,
            r##"#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn separator_has_role() {
        assert!(separator().element().render().contains("role=\"separator\""));
    }
}
"##,
        ),
        simple_manifest(
            "skeleton",
            r##"use ferrum_ui::components::skeleton::SkeletonProps;

pub fn skeleton() -> SkeletonProps {
    SkeletonProps::new()
}
"##,
            r##".ferrum-skeleton { background: hsl(var(--muted)); border-radius: var(--radius); }
"##,
            r##"#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn skeleton_is_hidden() {
        assert!(skeleton().element().render().contains("aria-hidden=\"true\""));
    }
}
"##,
        ),
        simple_manifest(
            "breadcrumb",
            r##"use ferrum_ui::components::breadcrumb::BreadcrumbProps;

pub fn breadcrumb() -> BreadcrumbProps {
    BreadcrumbProps::new(vec!["Home".into(), "Docs".into()])
}
"##,
            r##".ferrum-breadcrumb-list { display: flex; gap: 6px; list-style: none; padding: 0; }
"##,
            r##"#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn breadcrumb_has_nav_label() {
        assert!(breadcrumb().element().render().contains("aria-label=\"breadcrumb\""));
    }
}
"##,
        ),
        simple_manifest(
            "empty-state",
            r##"use ferrum_ui::components::empty_state::EmptyStateProps;

pub fn empty_state() -> EmptyStateProps {
    EmptyStateProps::new("No results")
}
"##,
            r##".ferrum-empty-state { text-align: center; padding: 24px; }
"##,
            r##"#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_state_renders_title() {
        assert!(empty_state().element().render().contains("No results"));
    }
}
"##,
        ),
        simple_manifest(
            "kbd",
            r##"use ferrum_ui::components::kbd::KbdProps;

pub fn kbd() -> KbdProps {
    KbdProps::new("⌘K")
}
"##,
            r##".ferrum-kbd { border: 1px solid hsl(var(--border)); border-radius: 4px; padding: 1px 4px; }
"##,
            r##"#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kbd_renders() {
        assert!(kbd().element().render().contains("⌘K"));
    }
}
"##,
        ),
        simple_manifest(
            "label",
            r##"use ferrum_ui::components::label::LabelProps;

pub fn label() -> LabelProps {
    LabelProps::new("Email").with_for("email")
}
"##,
            r##".ferrum-label { font-size: 0.8rem; }
"##,
            r##"#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn label_binds_control() {
        assert!(label().element().render().contains("for=\"email\""));
    }
}
"##,
        ),
        simple_manifest(
            "radio-group",
            r##"use ferrum_ui::components::radio_group::{RadioGroupProps, RadioGroupState, RadioOption};

pub fn radio_group() -> (RadioGroupState, RadioGroupProps) {
    let props = RadioGroupProps {
        name: "size".into(),
        options: vec![RadioOption::new("sm", "Small"), RadioOption::new("md", "Medium")],
        ..RadioGroupProps::default()
    };
    (RadioGroupState::new(&props), props)
}
"##,
            r##".ferrum-radio-group { display: grid; gap: 4px; }
"##,
            r##"#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn radio_group_has_role() {
        let (state, props) = radio_group();
        assert!(state.element(&props).render().contains("role=\"radiogroup\""));
    }
}
"##,
        ),
        simple_manifest(
            "slider",
            r##"use ferrum_ui::components::slider::SliderProps;

pub fn slider() -> SliderProps {
    SliderProps::new(40)
}
"##,
            r##".ferrum-slider { display: grid; gap: 4px; }
"##,
            r##"#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slider_has_range() {
        assert!(slider().element().render().contains("type=\"range\""));
    }
}
"##,
        ),
        simple_manifest(
            "switch",
            r##"use ferrum_ui::components::switch::{SwitchProps, SwitchState};

pub fn switch() -> (SwitchState, SwitchProps) {
    let props = SwitchProps::new("Dark mode");
    (SwitchState::new(&props), props)
}
"##,
            r##".ferrum-switch { display: inline-flex; align-items: center; gap: 6px; }
"##,
            r##"#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn switch_has_role() {
        let (state, props) = switch();
        assert!(props.element(&state).render().contains("role=\"switch\""));
    }
}
"##,
        ),
        simple_manifest(
            "textarea",
            r##"use ferrum_ui::components::textarea::{TextareaProps, TextareaState};

pub fn textarea() -> (TextareaState, TextareaProps) {
    let props = TextareaProps::new("Bio");
    (TextareaState::new(&props), props)
}
"##,
            r##".ferrum-textarea { padding: 6px 8px; border: 1px solid hsl(var(--border)); }
"##,
            r##"#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn textarea_starts_empty() {
        let (state, _) = textarea();
        assert!(state.value.is_empty());
    }
}
"##,
        ),
        simple_manifest(
            "tooltip",
            r##"use ferrum_ui::components::tooltip::TooltipProps;

pub fn tooltip() -> TooltipProps {
    TooltipProps::new("More info")
}
"##,
            r##".ferrum-tooltip { border-bottom: 1px dotted hsl(var(--muted)); }
"##,
            r##"#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tooltip_has_role() {
        assert!(tooltip().element().render().contains("role=\"tooltip\""));
    }
}
"##,
        ),
        simple_manifest(
            "spinner",
            r##"use ferrum_ui::components::spinner::SpinnerProps;

pub fn spinner() -> SpinnerProps {
    SpinnerProps::new()
}
"##,
            r##".ferrum-spinner-icon { display: inline-block; width: 16px; height: 16px; border: 2px solid hsl(var(--border)); border-top-color: hsl(var(--primary)); border-radius: 50%; animation: ferrum-spin 0.8s linear infinite; }
@keyframes ferrum-spin { to { transform: rotate(360deg); } }
"##,
            r##"#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spinner_has_status_role() {
        assert!(spinner().element().render().contains("role=\"status\""));
    }
}
"##,
        ),
    ]
}

const BUTTON_RS: &str = r##"// Generated by `ferrum add button`.
use ferrum_ui::components::button::{ButtonProps, ButtonSize, ButtonVariant};

pub fn button(label: &str) -> ButtonProps {
    ButtonProps::new(label)
        .with_variant(ButtonVariant::Default)
        .with_size(ButtonSize::Default)
}
"##;

const BUTTON_CSS: &str = r##".ferrum-button {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  border-radius: var(--radius);
  background: hsl(var(--bg));
  color: hsl(var(--fg));
  border: 1px solid hsl(var(--border));
}
.ferrum-button:focus-visible { outline: 2px solid hsl(var(--ring)); outline-offset: 2px; }
.ferrum-button.is-disabled { opacity: 0.5; cursor: not-allowed; }
"##;

const BUTTON_TEST: &str = r##"#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_button_is_enabled() {
        let props = button("Save");
        assert!(!props.disabled);
        assert!(props.element().render().contains("Save"));
    }
}
"##;

const DIALOG_RS: &str = r##"// Generated by `ferrum add dialog`.
use ferrum_ui::components::dialog::{DialogController, DialogProps};

pub fn dialog(title: &str) -> (DialogController, DialogProps) {
    (
        DialogController::new(),
        DialogProps {
            title: title.to_owned(),
            ..DialogProps::default()
        },
    )
}
"##;

const DIALOG_CSS: &str = r##".ferrum-dialog-backdrop { position: fixed; inset: 0; background: hsl(var(--fg) / 0.4); }
.ferrum-dialog { background: hsl(var(--bg)); color: hsl(var(--fg)); border: 1px solid hsl(var(--border)); }
"##;

const DIALOG_TEST: &str = r##"#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dialog_defaults_to_modal() {
        let (_, props) = dialog("Settings");
        assert!(props.modal);
        assert!(props.close_on_escape);
    }
}
"##;

const MENU_RS: &str = r##"// Generated by `ferrum add menu`.
use ferrum_ui::components::menu::{MenuItem, MenuProps, MenuState};

pub fn menu() -> (MenuState, MenuProps) {
    let props = MenuProps {
        items: vec![MenuItem::item("new", "New"), MenuItem::item("open", "Open")],
        open: true,
        ..MenuProps::default()
    };
    (MenuState::new(&props), props)
}
"##;

const MENU_CSS: &str = r##".ferrum-menu { background: hsl(var(--bg)); border: 1px solid hsl(var(--border)); }
.ferrum-menu [role="menuitem"]:focus { background: hsl(var(--accent)); color: hsl(var(--accent-fg)); }
"##;

const MENU_TEST: &str = r##"#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn menu_has_focusable_items() {
        let (state, _) = menu();
        assert_eq!(state.roving.items().len(), 2);
    }
}
"##;

const TABS_RS: &str = r##"// Generated by `ferrum add tabs`.
use ferrum_ui::components::tabs::{Tab, TabsProps, TabsState};

pub fn tabs() -> (TabsState, TabsProps) {
    let props = TabsProps {
        tabs: vec![Tab::new("overview", "Overview", "Overview"), Tab::new("activity", "Activity", "Activity")],
        ..TabsProps::default()
    };
    (TabsState::new(&props), props)
}
"##;

const TABS_CSS: &str = r##".ferrum-tablist { display: flex; border-bottom: 1px solid hsl(var(--border)); }
.ferrum-tab[aria-selected="true"] { color: hsl(var(--fg)); border-color: hsl(var(--ring)); }
"##;

const TABS_TEST: &str = r##"#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_tab_is_active() {
        let (state, _) = tabs();
        assert_eq!(state.active, 0);
    }
}
"##;

const TOAST_RS: &str = r##"// Generated by `ferrum add toast`.
use ferrum_ui::components::toast::{Toast, ToastQueue};

pub fn toast_queue() -> ToastQueue {
    let mut queue = ToastQueue::new();
    queue.push(Toast::new("saved", "Saved"));
    queue
}
"##;

const TOAST_CSS: &str = r##".ferrum-toast { background: hsl(var(--bg)); color: hsl(var(--fg)); border: 1px solid hsl(var(--border)); }
.ferrum-toast--danger { border-color: hsl(var(--danger)); }
"##;

const TOAST_TEST: &str = r##"#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn queue_starts_with_one_toast() {
        assert_eq!(toast_queue().len(), 1);
    }
}
"##;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builtin_registry_has_all_components() {
        let registry = Registry::builtin();
        assert_eq!(registry.list().len(), 25);
        for name in [
            "button",
            "dialog",
            "menu",
            "tabs",
            "toast",
            "accordion",
            "alert",
            "avatar",
            "badge",
            "card",
            "checkbox",
            "input",
            "progress",
            "separator",
            "skeleton",
            "breadcrumb",
            "empty-state",
            "kbd",
            "label",
            "radio-group",
            "slider",
            "switch",
            "textarea",
            "tooltip",
            "spinner",
        ] {
            assert!(registry.get(name).is_some(), "missing {name}");
        }
    }

    #[test]
    fn add_component_writes_files_and_skips_existing() {
        let registry = Registry::builtin();
        let dest = std::env::temp_dir().join(format!("ferrum-ui-add-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dest);
        let first = add_component(&registry, "button", &dest, false).unwrap();
        assert_eq!(first.written.len(), 3);
        let second = add_component(&registry, "button", &dest, false).unwrap();
        assert_eq!(second.skipped.len(), 3);
        assert!(dest.join("src/ui/button.rs").exists());
        let _ = std::fs::remove_dir_all(&dest);
    }

    #[test]
    fn registry_round_trips_json() {
        let registry = Registry::builtin();
        let json = registry.to_json().unwrap();
        let decoded = Registry::from_json(&json).unwrap();
        assert_eq!(decoded.list().len(), 25);
    }

    #[test]
    fn diff_marks_added_line() {
        let diff = diff_lines("a\nb", "a\nc");
        assert!(diff.contains("- b"));
        assert!(diff.contains("+ c"));
    }

    #[test]
    fn version_comparison_rejects_older_and_prerelease() {
        assert!(version_gt("0.2.0", "0.1.9"));
        assert!(!version_gt("0.1.0", "0.1.0"));
        assert!(!version_gt("1.0.0-rc1", "0.9.0"));
    }

    #[test]
    fn update_requires_newer_version() {
        let mut registry = Registry::builtin();
        assert!(registry.update("button", "0.1.0", vec![]).is_err());
        assert!(registry.update("button", "0.2.0", vec![]).is_ok());
        assert_eq!(registry.get("button").unwrap().version, "0.2.0");
    }

    #[test]
    fn deprecation_round_trips_and_survives_update() {
        let mut registry = Registry::new();
        registry
            .insert(Manifest::new("legacy", "0.1.0", vec![]).deprecate(true))
            .unwrap();
        let decoded = Registry::from_json(&registry.to_json().unwrap()).unwrap();
        assert!(decoded.get("legacy").unwrap().deprecated);

        registry.update("legacy", "0.2.0", vec![]).unwrap();
        assert!(registry.get("legacy").unwrap().deprecated);
        assert_eq!(registry.get("legacy").unwrap().version, "0.2.0");
    }
}
