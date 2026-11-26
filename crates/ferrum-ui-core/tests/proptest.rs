use ferrum_ui_core::style::{Class, Style};
use proptest::prelude::*;

proptest! {
    #[test]
    fn class_merge_is_idempotent(classes in prop::collection::vec("[a-z]{1,8}", 0..20)) {
        let class = Class::from(classes.join(" "));
        let mut merged = class.clone();
        merged.merge(&class);
        prop_assert_eq!(merged, class);
    }

    #[test]
    fn style_last_write_wins(
        property in "[a-z-]{1,12}",
        first in "[a-z]{1,8}",
        second in "[a-z]{1,8}"
    ) {
        let mut style = Style::new();
        style.set(property.clone(), first);
        style.set(property.clone(), second.clone());
        prop_assert_eq!(style.get(&property), Some(second.as_str()));
    }
}
