use criterion::{criterion_group, criterion_main, Criterion};
use ferrum_ui_core::roving::{Orientation, RovingTabindex};
use ferrum_ui_core::style::{Class, Style};

fn bench_class_merge(criterion: &mut Criterion) {
    criterion.bench_function("class_merge", |bencher| {
        bencher.iter(|| {
            let mut class = Class::from("ferrum-button ferrum-button--default");
            class.merge(&Class::from("w-full focus-visible:ring"));
            class.to_string()
        });
    });
}

fn bench_style_merge(criterion: &mut Criterion) {
    criterion.bench_function("style_merge", |bencher| {
        bencher.iter(|| {
            let mut style = Style::new();
            style.set("color", "red").set("display", "flex");
            let mut override_style = Style::new();
            override_style.set("color", "blue");
            style.merge(&override_style);
            style.to_css()
        });
    });
}

fn bench_roving(criterion: &mut Criterion) {
    criterion.bench_function("roving_next", |bencher| {
        let mut roving = RovingTabindex::new(
            (0..20).map(|index| format!("item-{index}")).collect(),
            Orientation::Vertical,
        );
        bencher.iter(|| roving.next_item());
    });
}

criterion_group!(benches, bench_class_merge, bench_style_merge, bench_roving);
criterion_main!(benches);
