use dayendar::types::{
    DateSpan, YearSpec, MonthSpec, YearMonthSpec, DateSpec, 
    Month, date
};

fn main() {
    use std::collections::HashSet;

    // 1. Using YearSpec
    // 1.1. Single Year
    let year_single: YearSpec = YearSpec::Single(2023);

    // 1.2. Range Year
    let _year_range: YearSpec = YearSpec::Range(2023..=2025);

    // 1.3. List Year
    let mut years_set: HashSet<u16> = HashSet::new();
    years_set.insert(2021);
    years_set.insert(2023);
    years_set.insert(2025);
    let _year_list: YearSpec = YearSpec::List(years_set);

    // 2. Using YearMonthSpec
    // 2.1. Single Month
    let year_month_single: YearMonthSpec = YearMonthSpec::parse("2023-July").expect("Failed to parse YearMonthSpec");

    // 3. Using DateSpec
    // 3.1. Single Date
    let date_single: DateSpec = DateSpec::Single(date!(2023 - 07 - 01));

    // 3.2. Range Range
    let _date_range: DateSpec = DateSpec::Range(date!(2023 - 07 - 01), date!(2023 - 07 - 15));

    // 3.3. Date list
    let mut dates_set: HashSet<time::Date> = HashSet::new();
    dates_set.insert(date!(2023 - 07 - 01));
    dates_set.insert(date!(2023 - 07 - 10));
    dates_set.insert(date!(2023 - 07 - 15));
    let _date_list: DateSpec = DateSpec::List(dates_set);

    // To DateSpan Conversions
    let date_span_from_year: DateSpan = DateSpan::Year(year_single);
    let date_span_from_year_month: DateSpan = DateSpan::YearMonth(year_month_single);
    let date_span_from_date: DateSpan = DateSpan::Date(date_single);

    // Print ...
    println!("{:?}", date_span_from_year);
    println!("{:?}", date_span_from_year_month);
    println!("{:?}", date_span_from_date);


    let ym_set1: HashSet<(YearSpec, MonthSpec)> = vec![
                (YearSpec::Single(2020), MonthSpec::Single(Month::January)),
                (YearSpec::Single(2020), MonthSpec::Single(Month::February)),
                (YearSpec::Single(2020), MonthSpec::Single(Month::March))
            ].into_iter().collect();
    
    let ym_set2: HashSet<(YearSpec, MonthSpec)> = vec![
                (YearSpec::Single(2020), MonthSpec::Single(Month::February))
            ].into_iter().collect();
            
    let ds1: DateSpan = DateSpan::YearMonth(YearMonthSpec(ym_set1));
    let ds2: DateSpan = DateSpan::YearMonth(YearMonthSpec(ym_set2));
    
    println!("\n{:?}", ds1.to_year_month());
    println!("{:?}", date_span_from_year.to_year_month());

}
