use criterion::{criterion_group, criterion_main, Criterion};
use dayendar::types::*;

use std::collections::HashSet;


fn benchmark_yearspec_to_year_month(c: &mut Criterion) {
    let spec: YearSpec = YearSpec::Single(2023);
    c.bench_function("yearspec_to_year_month_single", |b: &mut criterion::Bencher<'_>| b.iter(|| spec.to_year_month()));

    let spec: YearSpec = YearSpec::Range(2020..=2023);
    c.bench_function("yearspec_to_year_month_range", |b: &mut criterion::Bencher<'_>| b.iter(|| spec.to_year_month()));

    let mut set: HashSet<u16> = HashSet::new();
    set.insert(2020);
    set.insert(2021);
    set.insert(2022);
    let spec: YearSpec = YearSpec::List(set);
    c.bench_function("yearspec_to_year_month_list", |b: &mut criterion::Bencher<'_>| b.iter(|| spec.to_year_month()));
}

fn benchmark_monthspec_to_year_month(c: &mut Criterion) {
    let spec: MonthSpec = MonthSpec::Single(Month::January);
    c.bench_function("monthspec_to_year_month_single", |b: &mut criterion::Bencher<'_>| b.iter(|| spec.to_year_month(2023)));

    let spec = MonthSpec::Range(Month::January..=Month::June);
    c.bench_function("monthspec_to_year_month_range", |b: &mut criterion::Bencher<'_>| b.iter(|| spec.to_year_month(2023)));

    let mut set = HashSet::new();
    set.insert(Month::January);
    set.insert(Month::March);
    set.insert(Month::May);
    let spec: MonthSpec = MonthSpec::List(set);
    c.bench_function("monthspec_to_year_month_list", |b: &mut criterion::Bencher<'_>| b.iter(|| spec.to_year_month(2023)));
}

fn benchmark_yearmonthspec_to_year_month(c: &mut Criterion) {
    let mut set: HashSet<(YearSpec, MonthSpec)> = HashSet::new();
    set.insert((YearSpec::Single(2023), MonthSpec::Single(Month::January)));
    let spec: YearMonthSpec = YearMonthSpec(set);
    c.bench_function("yearmonthspec_to_year_month", |b| b.iter(|| spec.to_year_month()));
}

fn benchmark_datespec_to_year_month(c: &mut Criterion) {
    let spec = DateSpec::Single(date!(2023-01-15));
    c.bench_function("datespec_to_year_month_single", |b| b.iter(|| spec.to_year_month()));

    let spec = DateSpec::Range(date!(2023-01-15), date!(2023-06-15));
    c.bench_function("datespec_to_year_month_range", |b| b.iter(|| spec.to_year_month()));

    let mut set = HashSet::new();
    set.insert(date!(2023-01-15));
    set.insert(date!(2023-03-15));
    set.insert(date!(2023-05-15));
    let spec = DateSpec::List(set);
    c.bench_function("datespec_to_year_month_list", |b| b.iter(|| spec.to_year_month()));
}

fn benchmark_datespan_to_year_month(c: &mut Criterion) {
    let spec = DateSpan::Date(DateSpec::Single(date!(2023-01-15)));
    c.bench_function("datespan_to_year_month_date", |b| b.iter(|| spec.to_year_month()));
}

criterion_group!(
    benches, 
    benchmark_yearspec_to_year_month, 
    benchmark_monthspec_to_year_month,
    benchmark_yearmonthspec_to_year_month,
    benchmark_datespec_to_year_month,
    benchmark_datespan_to_year_month
);

criterion_main!(benches);
