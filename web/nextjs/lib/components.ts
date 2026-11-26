export type ComponentCategory = "Action" | "Form" | "Data" | "Feedback" | "Layout";

export type PropDef = {
  name: string;
  type: string;
  default: string;
  description: string;
};

export type ComponentExample = {
  title: string;
  description: string;
  code: string;
};

export type ComponentEntry = {
  name: string;
  slug: string;
  category: ComponentCategory;
  description: string;
  code: string;
  props: PropDef[];
  a11y: string[];
  keyboard: string[];
  examples: ComponentExample[];
};

export const components: ComponentEntry[] = [
  {
    name: "Accordion",
    slug: "accordion",
    category: "Layout",
    description: "Single or multiple open panels with roving focus.",
    code: 'AccordionProps { items, multiple: false }',
    props: [
      { name: "items", type: "Vec<AccordionItem>", default: "—", description: "Panel definitions rendered in order." },
      { name: "multiple", type: "bool", default: "false", description: "Allow more than one panel to be open at a time." },
      { name: "default_open", type: "Option<usize>", default: "None", description: "Index of the panel open on first render." },
      { name: "class", type: "Option<&str>", default: "None", description: "Extra class names applied to the root element." },
      { name: "style", type: "Option<&str>", default: "None", description: "Inline style overrides." },
    ],
    a11y: [
      "Each trigger is a button with aria-expanded and aria-controls.",
      "Panels use role=region and aria-labelledby pointing at their trigger.",
    ],
    keyboard: [
      "ArrowDown / ArrowUp move focus between triggers.",
      "Home / End jump to the first / last trigger.",
      "Enter / Space toggle the focused panel.",
    ],
    examples: [
      {
        title: "Single open",
        description: "Only one panel can be expanded at a time.",
        code: 'AccordionProps { items, multiple: false }',
      },
      {
        title: "Multiple open",
        description: "Independent panels can be expanded together.",
        code: 'AccordionProps { items, multiple: true, default_open: Some(0) }',
      },
    ],
  },
  {
    name: "Alert",
    slug: "alert",
    category: "Feedback",
    description: "Semantic status message with four variants.",
    code: 'AlertProps::new("Saved").with_variant(Success)',
    props: [
      { name: "variant", type: "AlertVariant", default: "Info", description: "Visual and semantic tone: Info, Success, Warning, Destructive." },
      { name: "title", type: "String", default: "—", description: "Short, prominent message." },
      { name: "description", type: "Option<String>", default: "None", description: "Optional supporting copy." },
      { name: "class", type: "Option<&str>", default: "None", description: "Extra class names." },
      { name: "style", type: "Option<&str>", default: "None", description: "Inline style overrides." },
    ],
    a11y: [
      "role=alert announces the message to assistive technology.",
      "The title is always rendered as text, never as an icon-only hint.",
    ],
    keyboard: ["No keyboard interaction; alerts are presentational status messages."],
    examples: [
      {
        title: "Success",
        description: "Confirm a completed action.",
        code: 'AlertProps::new("Saved").with_variant(Success)',
      },
      {
        title: "Destructive",
        description: "Call out a failed or destructive action.",
        code: 'AlertProps::new("Upload failed").with_variant(Destructive)',
      },
    ],
  },
  {
    name: "Avatar",
    slug: "avatar",
    category: "Data",
    description: "Accessible initials avatar with size tokens.",
    code: 'AvatarProps::new("AB").with_size(Lg)',
    props: [
      { name: "initials", type: "String", default: "—", description: "Two-letter fallback rendered inside the circle." },
      { name: "label", type: "Option<String>", default: "None", description: "Accessible name; falls back to initials." },
      { name: "size", type: "AvatarSize", default: "Md", description: "Sm, Md or Lg size token." },
      { name: "class", type: "Option<&str>", default: "None", description: "Extra class names." },
      { name: "style", type: "Option<&str>", default: "None", description: "Inline style overrides." },
    ],
    a11y: ["role=img with an accessible name from label, or the initials when no label is set."],
    keyboard: ["No keyboard interaction."],
    examples: [
      {
        title: "Initials",
        description: "A compact identity chip.",
        code: 'AvatarProps::new("AB").with_size(Lg)',
      },
      {
        title: "Labelled",
        description: "Provide a full accessible name.",
        code: 'AvatarProps::new("AL").with_label("Ada Lovelace")',
      },
    ],
  },
  {
    name: "Badge",
    slug: "badge",
    category: "Data",
    description: "Compact status label with six variants.",
    code: 'BadgeProps::new("New").with_variant(Success)',
    props: [
      { name: "label", type: "String", default: "—", description: "Visible badge text." },
      { name: "variant", type: "BadgeVariant", default: "Default", description: "Default, Success, Warning, Error, Info or Outline." },
      { name: "class", type: "Option<&str>", default: "None", description: "Extra class names." },
      { name: "style", type: "Option<&str>", default: "None", description: "Inline style overrides." },
    ],
    a11y: ["Presentational text; pair with a live region when the status changes dynamically."],
    keyboard: ["No keyboard interaction."],
    examples: [
      {
        title: "Status",
        description: "Communicate state at a glance.",
        code: 'BadgeProps::new("New").with_variant(Success)',
      },
      {
        title: "Outline",
        description: "A quieter, bordered treatment.",
        code: 'BadgeProps::new("Draft").with_variant(Outline)',
      },
    ],
  },
  {
    name: "Button",
    slug: "button",
    category: "Action",
    description: "Six variants, four sizes, loading and disabled states.",
    code: 'ButtonProps::new("Save").with_variant(Primary)',
    props: [
      { name: "variant", type: "ButtonVariant", default: "Primary", description: "Primary, Secondary, Outline, Ghost, Destructive or Link." },
      { name: "size", type: "ButtonSize", default: "Md", description: "Sm, Md, Lg or Icon." },
      { name: "disabled", type: "bool", default: "false", description: "Prevent interaction and expose aria-disabled." },
      { name: "loading", type: "bool", default: "false", description: "Show a spinner and set aria-busy." },
      { name: "label", type: "String", default: "—", description: "Visible button text." },
      { name: "class", type: "Option<&str>", default: "None", description: "Extra class names." },
      { name: "style", type: "Option<&str>", default: "None", description: "Inline style overrides." },
    ],
    a11y: [
      "Native button semantics plus aria-disabled and aria-busy.",
      "Focus ring is only shown for :focus-visible.",
    ],
    keyboard: ["Enter and Space activate the button.", "Tab moves focus to the next control."],
    examples: [
      {
        title: "Variants",
        description: "The full variant set.",
        code: 'ButtonProps::new("Save").with_variant(Primary)',
      },
      {
        title: "Loading",
        description: "Announce progress without losing the label.",
        code: 'ButtonProps::new("Saving").loading(true)',
      },
      {
        title: "Disabled",
        description: "Unavailable actions stay focusable but inert.",
        code: 'ButtonProps::new("Save").disabled(true)',
      },
    ],
  },
  {
    name: "Card",
    slug: "card",
    category: "Layout",
    description: "Titled content container with optional footer.",
    code: 'CardProps::new("Project").with_footer("Done")',
    props: [
      { name: "title", type: "String", default: "—", description: "Heading rendered at the top of the card." },
      { name: "description", type: "Option<String>", default: "None", description: "Supporting copy under the title." },
      { name: "footer", type: "Option<String>", default: "None", description: "Optional footer content." },
      { name: "class", type: "Option<&str>", default: "None", description: "Extra class names." },
      { name: "style", type: "Option<&str>", default: "None", description: "Inline style overrides." },
    ],
    a11y: ["Semantic section with a heading and optional footer."],
    keyboard: ["No keyboard interaction."],
    examples: [
      {
        title: "With footer",
        description: "Group related content and actions.",
        code: 'CardProps::new("Project").with_footer("Done")',
      },
      {
        title: "Minimal",
        description: "A title is all you need.",
        code: 'CardProps::new("Project")',
      },
    ],
  },
  {
    name: "Checkbox",
    slug: "checkbox",
    category: "Form",
    description: "Checked, disabled and indeterminate states.",
    code: 'CheckboxProps::new("Accept").indeterminate(true)',
    props: [
      { name: "label", type: "String", default: "—", description: "Text bound to the checkbox." },
      { name: "checked", type: "bool", default: "false", description: "Controlled checked state." },
      { name: "disabled", type: "bool", default: "false", description: "Ignore input and expose aria-disabled." },
      { name: "indeterminate", type: "bool", default: "false", description: "Show a mixed state." },
      { name: "class", type: "Option<&str>", default: "None", description: "Extra class names." },
      { name: "style", type: "Option<&str>", default: "None", description: "Inline style overrides." },
    ],
    a11y: ["Native checkbox with aria-checked and aria-disabled; the label wraps the input."],
    keyboard: ["Space or Enter toggles the checkbox.", "Disabled checkboxes ignore input."],
    examples: [
      {
        title: "Checked",
        description: "A controlled checked state.",
        code: 'CheckboxProps::new("Accept").checked(true)',
      },
      {
        title: "Indeterminate",
        description: "A partially selected group.",
        code: 'CheckboxProps::new("Select all").indeterminate(true)',
      },
    ],
  },
  {
    name: "Dialog",
    slug: "dialog",
    category: "Layout",
    description: "Portal-friendly modal with focus trap and restore.",
    code: 'DialogController::new().open(trigger, focusables)',
    props: [
      { name: "title", type: "String", default: "—", description: "Accessible dialog title." },
      { name: "description", type: "Option<String>", default: "None", description: "Supporting copy." },
      { name: "open", type: "bool", default: "false", description: "Controlled visibility." },
      { name: "modal", type: "bool", default: "true", description: "Trap focus and block outside interaction." },
      { name: "close_on_escape", type: "bool", default: "true", description: "Close when Escape is pressed." },
      { name: "class", type: "Option<&str>", default: "None", description: "Extra class names." },
      { name: "style", type: "Option<&str>", default: "None", description: "Inline style overrides." },
    ],
    a11y: [
      "role=dialog with aria-modal and aria-labelledby.",
      "Focus is restored to the trigger when the dialog closes.",
    ],
    keyboard: ["Escape closes the dialog.", "Tab / Shift+Tab are trapped inside the dialog."],
    examples: [
      {
        title: "Basic",
        description: "A modal with a title and description.",
        code: 'DialogProps::new("New task").with_description("Create a task.")',
      },
      {
        title: "Non-modal",
        description: "Allow interaction outside the dialog.",
        code: 'DialogProps::new("Details").modal(false)',
      },
    ],
  },
  {
    name: "Input",
    slug: "input",
    category: "Form",
    description: "Text, email, password and search inputs with invalid state.",
    code: 'InputProps::new("Email").with_kind(Email)',
    props: [
      { name: "label", type: "String", default: "—", description: "Visible label bound to the input." },
      { name: "value", type: "String", default: '""', description: "Controlled value." },
      { name: "placeholder", type: "Option<String>", default: "None", description: "Hint shown when empty." },
      { name: "kind", type: "InputKind", default: "Text", description: "Text, Email, Password or Search." },
      { name: "disabled", type: "bool", default: "false", description: "Disable the input." },
      { name: "invalid", type: "bool", default: "false", description: "Expose aria-invalid and error styling." },
      { name: "class", type: "Option<&str>", default: "None", description: "Extra class names." },
      { name: "style", type: "Option<&str>", default: "None", description: "Inline style overrides." },
    ],
    a11y: ["Label bound to the input; aria-invalid and aria-disabled exposed when set."],
    keyboard: ["Typing updates state; Backspace deletes.", "Disabled inputs ignore input."],
    examples: [
      {
        title: "Email",
        description: "A labelled email field.",
        code: 'InputProps::new("Email").with_kind(Email)',
      },
      {
        title: "Invalid",
        description: "Show validation state.",
        code: 'InputProps::new("Email").invalid(true)',
      },
    ],
  },
  {
    name: "Menu",
    slug: "menu",
    category: "Action",
    description: "Roving tabindex, typeahead, Escape and outside click.",
    code: 'MenuState::new(&props).handle_key(&props, &event)',
    props: [
      { name: "items", type: "Vec<MenuItem>", default: "—", description: "Menu entries, separators and labels." },
      { name: "orientation", type: "Orientation", default: "Horizontal", description: "Arrow-key axis." },
      { name: "wrap", type: "bool", default: "true", description: "Wrap focus at the ends." },
      { name: "typeahead", type: "bool", default: "true", description: "Jump to items by printable characters." },
      { name: "open", type: "bool", default: "false", description: "Controlled visibility." },
    ],
    a11y: ["role=menu with role=menuitem children and aria-orientation."],
    keyboard: [
      "Arrows move focus, Home / End jump.",
      "Enter / Space select, Escape closes.",
      "Printable keys perform typeahead.",
    ],
    examples: [
      {
        title: "Basic",
        description: "A vertical action menu.",
        code: 'MenuProps { items, orientation: Vertical, wrap: true }',
      },
      {
        title: "No wrap",
        description: "Stop focus at the first and last item.",
        code: 'MenuProps { items, wrap: false }',
      },
    ],
  },
  {
    name: "Progress",
    slug: "progress",
    category: "Data",
    description: "Determinate progress bar with clamped ARIA values.",
    code: 'ProgressProps::new(64).with_label("Upload")',
    props: [
      { name: "value", type: "u8", default: "0", description: "Current value, clamped to 0…max." },
      { name: "max", type: "u8", default: "100", description: "Upper bound." },
      { name: "label", type: "Option<String>", default: "None", description: "Accessible name." },
      { name: "class", type: "Option<&str>", default: "None", description: "Extra class names." },
      { name: "style", type: "Option<&str>", default: "None", description: "Inline style overrides." },
    ],
    a11y: ["role=progressbar with aria-valuemin, aria-valuemax, aria-valuenow and a label."],
    keyboard: ["No keyboard interaction."],
    examples: [
      {
        title: "Determinate",
        description: "Show measured completion.",
        code: 'ProgressProps::new(64).with_label("Upload")',
      },
      {
        title: "Custom max",
        description: "Use a non-100 scale.",
        code: 'ProgressProps::new(3).with_max(4)',
      },
    ],
  },
  {
    name: "Separator",
    slug: "separator",
    category: "Layout",
    description: "Horizontal or vertical rule, decorative or semantic.",
    code: 'SeparatorProps::new().vertical()',
    props: [
      { name: "orientation", type: "Orientation", default: "Horizontal", description: "Horizontal or vertical rule." },
      { name: "decorative", type: "bool", default: "false", description: "Hide from assistive technology when true." },
      { name: "class", type: "Option<&str>", default: "None", description: "Extra class names." },
      { name: "style", type: "Option<&str>", default: "None", description: "Inline style overrides." },
    ],
    a11y: ["role=separator with aria-orientation, or aria-hidden when decorative."],
    keyboard: ["No keyboard interaction."],
    examples: [
      {
        title: "Horizontal",
        description: "Divide stacked content.",
        code: 'SeparatorProps::new()',
      },
      {
        title: "Vertical",
        description: "Divide inline content.",
        code: 'SeparatorProps::new().vertical()',
      },
    ],
  },
  {
    name: "Skeleton",
    slug: "skeleton",
    category: "Feedback",
    description: "Hidden-from-AT loading placeholder with size tokens.",
    code: 'SkeletonProps::new().with_size("180px", "12px")',
    props: [
      { name: "width", type: "Option<&str>", default: "None", description: "CSS width, e.g. 180px or 60%." },
      { name: "height", type: "Option<&str>", default: "None", description: "CSS height, e.g. 12px." },
      { name: "class", type: "Option<&str>", default: "None", description: "Extra class names." },
      { name: "style", type: "Option<&str>", default: "None", description: "Inline style overrides." },
    ],
    a11y: ["aria-hidden=true so loading placeholders are not announced."],
    keyboard: ["No keyboard interaction."],
    examples: [
      {
        title: "Lines",
        description: "Placeholder text lines.",
        code: 'SkeletonProps::new().with_size("180px", "12px")',
      },
      {
        title: "Circle",
        description: "Placeholder avatar.",
        code: 'SkeletonProps::new().with_size("40px", "40px")',
      },
    ],
  },
  {
    name: "Tabs",
    slug: "tabs",
    category: "Layout",
    description: "Automatic or manual activation with lazy panels.",
    code: 'TabsProps { activation: Manual, lazy: true }',
    props: [
      { name: "tabs", type: "Vec<Tab>", default: "—", description: "Tab definitions with panel content." },
      { name: "orientation", type: "Orientation", default: "Horizontal", description: "Arrow-key axis." },
      { name: "activation", type: "Activation", default: "Automatic", description: "Automatic or manual selection on focus." },
      { name: "lazy", type: "bool", default: "false", description: "Mount panels only after first activation." },
      { name: "wrap", type: "bool", default: "true", description: "Wrap focus at the ends." },
      { name: "active", type: "Option<usize>", default: "None", description: "Controlled active index." },
    ],
    a11y: ["role=tablist/tab/tabpanel with aria-selected, aria-controls and labelledby."],
    keyboard: [
      "Arrows move focus, Home / End jump.",
      "Automatic activation selects on focus; manual only moves focus.",
    ],
    examples: [
      {
        title: "Automatic",
        description: "Selection follows focus.",
        code: 'TabsProps { activation: Automatic, lazy: false }',
      },
      {
        title: "Manual + lazy",
        description: "Select with Enter and defer panel mounting.",
        code: 'TabsProps { activation: Manual, lazy: true }',
      },
    ],
  },
  {
    name: "Toast",
    slug: "toast",
    category: "Feedback",
    description: "Deduplicated queue with polite live region and pause.",
    code: 'ToastQueue::new().push(Toast::new("saved", "Saved"))',
    props: [
      { name: "id", type: "&str", default: "—", description: "Stable id used for deduplication." },
      { name: "title", type: "String", default: "—", description: "Short headline." },
      { name: "description", type: "Option<String>", default: "None", description: "Supporting copy." },
      { name: "variant", type: "ToastVariant", default: "Default", description: "Default, Success, Error or Warning." },
      { name: "duration_ms", type: "Option<u64>", default: "None", description: "Auto-dismiss delay; None keeps it until dismissed." },
    ],
    a11y: ["role=status with aria-live=polite; pause switches the live region to off."],
    keyboard: ["No keyboard interaction; dismiss controls should be reachable in the host app."],
    examples: [
      {
        title: "Success",
        description: "Confirm a saved change.",
        code: 'Toast::new("saved", "Saved").with_variant(Success)',
      },
      {
        title: "Timed",
        description: "Auto-dismiss after 4 seconds.",
        code: 'Toast::new("copied", "Copied").with_duration_ms(4000)',
      },
    ],
  },
  {
    name: "Breadcrumb",
    slug: "breadcrumb",
    category: "Layout",
    description: "Navigation trail with separators and current-page semantics.",
    code: 'BreadcrumbProps::new(vec!["Home".into(), "Docs".into()])',
    props: [
      { name: "items", type: "Vec<String>", default: "—", description: "Ordered trail labels." },
      { name: "separator", type: "String", default: '"/"', description: "Separator rendered between items." },
      { name: "class", type: "Option<&str>", default: "None", description: "Extra class names." },
      { name: "style", type: "Option<&str>", default: "None", description: "Inline style overrides." },
    ],
    a11y: ["nav with aria-label=breadcrumb; the current page gets aria-current=page."],
    keyboard: ["Links inside the trail are reachable with Tab."],
    examples: [
      {
        title: "Default",
        description: "A slash-separated trail.",
        code: 'BreadcrumbProps::new(vec!["Home".into(), "Docs".into()])',
      },
      {
        title: "Custom separator",
        description: "Use a chevron instead.",
        code: 'BreadcrumbProps::new(items).with_separator("›")',
      },
    ],
  },
  {
    name: "EmptyState",
    slug: "empty-state",
    category: "Layout",
    description: "Centered placeholder for empty collections and search results.",
    code: 'EmptyStateProps::new("No results").with_description("Try another filter")',
    props: [
      { name: "title", type: "String", default: "—", description: "Primary message." },
      { name: "description", type: "Option<String>", default: "None", description: "Suggested next step." },
      { name: "class", type: "Option<&str>", default: "None", description: "Extra class names." },
      { name: "style", type: "Option<&str>", default: "None", description: "Inline style overrides." },
    ],
    a11y: ["Semantic section with a heading and optional description."],
    keyboard: ["No keyboard interaction."],
    examples: [
      {
        title: "Search",
        description: "Explain why a list is empty.",
        code: 'EmptyStateProps::new("No results").with_description("Try another filter")',
      },
      {
        title: "First run",
        description: "Guide the user to create something.",
        code: 'EmptyStateProps::new("No projects yet")',
      },
    ],
  },
  {
    name: "Kbd",
    slug: "kbd",
    category: "Action",
    description: "Keyboard shortcut badge.",
    code: 'KbdProps::new("⌘K")',
    props: [
      { name: "label", type: "String", default: "—", description: "Shortcut text." },
      { name: "class", type: "Option<&str>", default: "None", description: "Extra class names." },
      { name: "style", type: "Option<&str>", default: "None", description: "Inline style overrides." },
    ],
    a11y: ["Native kbd element; presentational shortcut text."],
    keyboard: ["No keyboard interaction."],
    examples: [
      {
        title: "Modifier",
        description: "Show a command shortcut.",
        code: 'KbdProps::new("⌘K")',
      },
      {
        title: "Single key",
        description: "Show a single key press.",
        code: 'KbdProps::new("Esc")',
      },
    ],
  },
  {
    name: "Label",
    slug: "label",
    category: "Form",
    description: "Form label bound to a control id.",
    code: 'LabelProps::new("Email").with_for("email")',
    props: [
      { name: "text", type: "String", default: "—", description: "Visible label text." },
      { name: "for_id", type: "Option<&str>", default: "None", description: "Id of the bound control." },
      { name: "class", type: "Option<&str>", default: "None", description: "Extra class names." },
      { name: "style", type: "Option<&str>", default: "None", description: "Inline style overrides." },
    ],
    a11y: ["Native label with for/id binding."],
    keyboard: ["Clicking the label focuses the bound control."],
    examples: [
      {
        title: "Bound",
        description: "Associate with an input id.",
        code: 'LabelProps::new("Email").with_for("email")',
      },
      {
        title: "Standalone",
        description: "A label without a control.",
        code: 'LabelProps::new("Section")',
      },
    ],
  },
  {
    name: "RadioGroup",
    slug: "radio-group",
    category: "Form",
    description: "Radio group with arrow-key selection.",
    code: 'RadioGroupProps { name, options, value }',
    props: [
      { name: "name", type: "&str", default: "—", description: "Shared input name." },
      { name: "options", type: "Vec<String>", default: "—", description: "Selectable labels." },
      { name: "value", type: "Option<usize>", default: "None", description: "Controlled selected index." },
      { name: "disabled", type: "bool", default: "false", description: "Disable the whole group." },
      { name: "class", type: "Option<&str>", default: "None", description: "Extra class names." },
      { name: "style", type: "Option<&str>", default: "None", description: "Inline style overrides." },
    ],
    a11y: ["role=radiogroup with native radio inputs and disabled state."],
    keyboard: ["Arrows move and select.", "Home / End jump to the first / last option."],
    examples: [
      {
        title: "Sizes",
        description: "Choose one of three sizes.",
        code: 'RadioGroupProps { name: "size", options, value: Some(0) }',
      },
      {
        title: "Disabled",
        description: "Lock the group.",
        code: 'RadioGroupProps { name: "size", options, disabled: true }',
      },
    ],
  },
  {
    name: "Slider",
    slug: "slider",
    category: "Form",
    description: "Range slider with clamped value and step.",
    code: 'SliderProps::new(64).with_range(0, 100)',
    props: [
      { name: "value", type: "i64", default: "0", description: "Controlled value." },
      { name: "min", type: "i64", default: "0", description: "Lower bound." },
      { name: "max", type: "i64", default: "100", description: "Upper bound." },
      { name: "step", type: "i64", default: "1", description: "Increment between values." },
      { name: "label", type: "Option<String>", default: "None", description: "Accessible name." },
      { name: "disabled", type: "bool", default: "false", description: "Disable interaction." },
    ],
    a11y: ["Native range input with aria-label and disabled state."],
    keyboard: ["Arrow keys change the value.", "Home / End jump to the bounds."],
    examples: [
      {
        title: "Default",
        description: "A 0–100 range.",
        code: 'SliderProps::new(64).with_range(0, 100)',
      },
      {
        title: "Stepped",
        description: "Increment by 10.",
        code: 'SliderProps::new(50).with_step(10)',
      },
    ],
  },
  {
    name: "Switch",
    slug: "switch",
    category: "Form",
    description: "On/off switch with role=switch semantics.",
    code: 'SwitchProps::new("Dark mode").checked(true)',
    props: [
      { name: "label", type: "String", default: "—", description: "Visible label." },
      { name: "checked", type: "bool", default: "false", description: "Controlled on/off state." },
      { name: "disabled", type: "bool", default: "false", description: "Disable interaction." },
      { name: "class", type: "Option<&str>", default: "None", description: "Extra class names." },
      { name: "style", type: "Option<&str>", default: "None", description: "Inline style overrides." },
    ],
    a11y: ["Native checkbox with role=switch and aria-checked."],
    keyboard: ["Space or Enter toggles the switch.", "Disabled switches ignore input."],
    examples: [
      {
        title: "On",
        description: "An enabled switch.",
        code: 'SwitchProps::new("Dark mode").checked(true)',
      },
      {
        title: "Off",
        description: "A disabled switch.",
        code: 'SwitchProps::new("Airplane mode").disabled(true)',
      },
    ],
  },
  {
    name: "Textarea",
    slug: "textarea",
    category: "Form",
    description: "Multiline input with label, placeholder and invalid state.",
    code: 'TextareaProps::new("Bio").with_rows(4)',
    props: [
      { name: "label", type: "String", default: "—", description: "Visible label." },
      { name: "value", type: "String", default: '""', description: "Controlled value." },
      { name: "placeholder", type: "Option<String>", default: "None", description: "Hint shown when empty." },
      { name: "rows", type: "usize", default: "3", description: "Visible text rows." },
      { name: "disabled", type: "bool", default: "false", description: "Disable the textarea." },
      { name: "invalid", type: "bool", default: "false", description: "Expose aria-invalid and error styling." },
    ],
    a11y: ["Label bound to the textarea; aria-invalid and aria-disabled exposed when set."],
    keyboard: ["Typing updates value; Backspace deletes.", "Disabled textareas ignore input."],
    examples: [
      {
        title: "Bio",
        description: "A four-row textarea.",
        code: 'TextareaProps::new("Bio").with_rows(4)',
      },
      {
        title: "Invalid",
        description: "Show a validation error.",
        code: 'TextareaProps::new("Bio").invalid(true)',
      },
    ],
  },
  {
    name: "Tooltip",
    slug: "tooltip",
    category: "Action",
    description: "Short contextual hint with tooltip semantics.",
    code: 'TooltipProps::new("More info").with_label("Info")',
    props: [
      { name: "text", type: "String", default: "—", description: "Tooltip content." },
      { name: "label", type: "Option<String>", default: "None", description: "Accessible name for the trigger." },
      { name: "class", type: "Option<&str>", default: "None", description: "Extra class names." },
      { name: "style", type: "Option<&str>", default: "None", description: "Inline style overrides." },
    ],
    a11y: ["role=tooltip with an optional aria-label on the trigger."],
    keyboard: ["The host app should show / hide on focus and hover."],
    examples: [
      {
        title: "Icon trigger",
        description: "Explain an icon-only control.",
        code: 'TooltipProps::new("More info").with_label("Info")',
      },
      {
        title: "Text trigger",
        description: "Add a hint to a text control.",
        code: 'TooltipProps::new("Deletes permanently")',
      },
    ],
  },
  {
    name: "Spinner",
    slug: "spinner",
    category: "Feedback",
    description: "Indeterminate loading indicator with status semantics.",
    code: 'SpinnerProps::new().with_size(Lg)',
    props: [
      { name: "label", type: "Option<String>", default: "None", description: "Accessible status text." },
      { name: "size", type: "SpinnerSize", default: "Md", description: "Sm, Md or Lg." },
      { name: "class", type: "Option<&str>", default: "None", description: "Extra class names." },
      { name: "style", type: "Option<&str>", default: "None", description: "Inline style overrides." },
    ],
    a11y: ["role=status with aria-label; the icon is aria-hidden."],
    keyboard: ["No keyboard interaction."],
    examples: [
      {
        title: "Inline",
        description: "Loading next to text.",
        code: 'SpinnerProps::new().with_label("Loading")',
      },
      {
        title: "Large",
        description: "A prominent loading state.",
        code: 'SpinnerProps::new().with_size(Lg)',
      },
    ],
  },
];

export const categories: ComponentCategory[] = [
  "Action",
  "Form",
  "Data",
  "Feedback",
  "Layout",
];

export function getComponent(slug: string) {
  return components.find((entry) => entry.slug === slug);
}

export function getAdjacent(slug: string) {
  const index = components.findIndex((entry) => entry.slug === slug);
  return {
    previous: index > 0 ? components[index - 1] : null,
    next: index >= 0 && index < components.length - 1 ? components[index + 1] : null,
  };
}