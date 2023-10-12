use criterion::{black_box, criterion_group, criterion_main, Criterion};
use dayendar::calendar::{DaysCalendar, BiDay, or_daycalendar, and_daycalendar, match_daycalendar};

fn or_benchmark(c: &mut Criterion) {
    let v1 = black_box(vec![BiDay::One; 1000]);
    let v2 = black_box(vec![BiDay::Zero; 1000]);

    c.bench_function("or days calendar", |b| b.iter(|| or_daycalendar(&v1, &v2)));
}

fn and_benchmark(c: &mut Criterion) {
    let v1 = black_box(vec![BiDay::One; 1000]);
    let v2 = black_box(vec![BiDay::One; 1000]);

    c.bench_function("and days calendar", |b| b.iter(|| and_daycalendar(&v1, &v2))); 
}

fn match_benchmark(c: &mut Criterion) {
    let v1 = black_box(vec![BiDay::One; 1000]);
    let v2 = black_box(vec![BiDay::Zero; 1000]);

    c.bench_function("match days calendar", |b| b.iter(|| match_daycalendar(&v1, &v2)));
}

fn calendar_operations_benchmark(c: &mut Criterion) {
    let calendar1 = black_box(DaysCalendar::ones(DaysCalendar::empty()));
    let calendar2 = black_box(DaysCalendar::zeros(DaysCalendar::empty()));

    c.bench_function("calendar or", |b| b.iter(|| calendar1.or(&calendar2)));
    c.bench_function("calendar and", |b| b.iter(|| calendar1.and(&calendar2)));
    c.bench_function("calendar match", |b| b.iter(|| calendar1.r#match(&calendar2)));
}

criterion_group!(
    benches, 
    or_benchmark,
    and_benchmark,
    match_benchmark,
    calendar_operations_benchmark
);
criterion_main!(benches);