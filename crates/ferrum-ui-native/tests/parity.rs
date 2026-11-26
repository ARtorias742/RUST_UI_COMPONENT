use ferrum_ui::components::accordion::{AccordionProps, AccordionState};
use ferrum_ui::components::alert::{AlertProps, AlertVariant};
use ferrum_ui::components::avatar::{AvatarProps, AvatarSize};
use ferrum_ui::components::badge::{BadgeProps, BadgeVariant};
use ferrum_ui::components::button::{ButtonProps, ButtonSize, ButtonVariant};
use ferrum_ui::components::card::CardProps;
use ferrum_ui::components::checkbox::{CheckboxProps, CheckboxState};
use ferrum_ui::components::dialog::DialogProps;
use ferrum_ui::components::input::{InputKind, InputProps, InputState};
use ferrum_ui::components::progress::ProgressProps;
use ferrum_ui::components::separator::{SeparatorOrientation, SeparatorProps};
use ferrum_ui::components::skeleton::SkeletonProps;
use ferrum_ui::components::tabs::TabsProps;
use ferrum_ui_core::style::Style;
use ferrum_ui_native::{parity, NativeStyle};

fn assert_parity(style: &Style) {
    let native = NativeStyle::from(style);
    assert!(
        parity(style, &native).is_empty(),
        "web/native style drift: {:?}",
        parity(style, &native)
    );
}

#[test]
fn every_component_base_style_has_web_native_parity() {
    assert_parity(&ButtonProps::new("Save").element().style);
    assert_parity(&AlertProps::new("Saved").element().style);
    assert_parity(&AvatarProps::new("AB").element().style);
    assert_parity(&BadgeProps::new("New").element().style);
    assert_parity(&CardProps::new("Project").element().style);
    assert_parity(&ProgressProps::new(40).element().style);
    assert_parity(&SeparatorProps::new().element().style);
    assert_parity(&SkeletonProps::new().element().style);
    assert_parity(&DialogProps::default().style);
    assert_parity(&TabsProps::default().style);

    let accordion_props = AccordionProps::default();
    assert_parity(
        &AccordionState::new(&accordion_props)
            .element(&accordion_props)
            .style,
    );

    let checkbox_props = CheckboxProps::new("Accept");
    let checkbox_state = CheckboxState::new(&checkbox_props);
    assert_parity(&checkbox_props.element(&checkbox_state).style);

    let input_props = InputProps::new("Email");
    let input_state = InputState::new(&input_props);
    assert_parity(&input_props.element(&input_state).style);
}

#[test]
fn every_variant_has_web_native_parity() {
    for variant in [
        ButtonVariant::Default,
        ButtonVariant::Secondary,
        ButtonVariant::Outline,
        ButtonVariant::Ghost,
        ButtonVariant::Destructive,
        ButtonVariant::Link,
    ] {
        for size in [
            ButtonSize::Default,
            ButtonSize::Sm,
            ButtonSize::Lg,
            ButtonSize::Icon,
        ] {
            assert_parity(
                &ButtonProps::new("Save")
                    .with_variant(variant)
                    .with_size(size)
                    .element()
                    .style,
            );
        }
    }
    for variant in [
        AlertVariant::Default,
        AlertVariant::Success,
        AlertVariant::Warning,
        AlertVariant::Danger,
    ] {
        assert_parity(
            &AlertProps::new("Saved")
                .with_variant(variant)
                .element()
                .style,
        );
    }
    for size in [AvatarSize::Sm, AvatarSize::Md, AvatarSize::Lg] {
        assert_parity(&AvatarProps::new("AB").with_size(size).element().style);
    }
    for variant in [
        BadgeVariant::Default,
        BadgeVariant::Secondary,
        BadgeVariant::Outline,
        BadgeVariant::Success,
        BadgeVariant::Warning,
        BadgeVariant::Danger,
    ] {
        assert_parity(&BadgeProps::new("New").with_variant(variant).element().style);
    }
    for kind in [
        InputKind::Text,
        InputKind::Email,
        InputKind::Password,
        InputKind::Search,
    ] {
        let props = InputProps::new("Field").with_kind(kind);
        let state = InputState::new(&props);
        assert_parity(&props.element(&state).style);
    }
    for orientation in [
        SeparatorOrientation::Horizontal,
        SeparatorOrientation::Vertical,
    ] {
        let props = SeparatorProps::new();
        let props = if orientation == SeparatorOrientation::Vertical {
            props.vertical()
        } else {
            props
        };
        assert_parity(&props.element().style);
    }
}

#[test]
fn custom_styles_round_trip_without_drift() {
    let mut style = Style::new();
    style
        .set("color", "red")
        .set("display", "flex")
        .set("width", "50%");
    let props = ButtonProps::new("Save").with_style(style.clone());
    assert_parity(&props.element().style);
    assert_parity(&style);
}
