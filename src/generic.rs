/// The `types` module provides the general data types used in the system
pub mod types {

    /// Represents the months of the year and comes directly from `time::Month`.
    pub use time::Month as TimeMonth;
    pub use time::{Date as Date, Duration as Duration, Weekday as Weekday};
    /// Re-export: The macro `date`. Comes directly from `time::macros::date`.
    pub use time::macros::date;
    use core::ops::RangeInclusive;
    use std::collections::HashSet;

    /// Represents the year
    pub type Year = u16;
    /// Represents the day of the month
    pub type Day = u8;

    /// Is the value of the minimum date allowed. It comes from the data type `time::Date`.
    pub const MIN_DATE: Date = date!(0001 - 01 - 01);
    /// Is the value of the maximun date allowed. It comes from the data type `time::Date`.
    pub const MAX_DATE: Date = date!(9999 - 12 - 31);

    /// Enumerator `Month` representing the months of the year
    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[repr(u8)]
    pub enum Month {
        January = 1,
        February,
        March,
        April,
        May,
        June,
        July,
        August,
        September,
        October,
        November,
        December,
    }

    impl Month {
        /// `to_index` function takes the name of a given month and returns the number of the month
        pub fn to_index(&self) -> u8 {
            *self as u8
        }
        /// `from_index` returns the name of the month from its numerical representation
        pub fn from_index(index: u8) -> Option<Self> {
            match index {
                1 => Some(Month::January),
                2 => Some(Month::February),
                3 => Some(Month::March),
                4 => Some(Month::April),
                5 => Some(Month::May),
                6 => Some(Month::June),
                7 => Some(Month::July),
                8 => Some(Month::August),
                9 => Some(Month::September),
                10 => Some(Month::October),
                11 => Some(Month::November),
                12 => Some(Month::December),
                _ => None,
            }
        }
        /// Converts `Month` type to `TimeMonth` type
        pub fn to_time_month(&self) -> TimeMonth {
            TimeMonth::try_from(self.to_index()).expect("Invalid month index")
        }

        /// Converts a string representation of a month into a `Month`.
        /// 
        /// # Arguments
        /// 
        /// * `s` - A string slice that holds the representation of a month.
        /// 
        /// # Returns
        /// 
        /// * `Option<Month>` - Returns `Some(Month)` if the string is a valid representation, otherwise returns `None`.
        pub fn from_str(s: &str) -> Option<Self> {
            match s.to_lowercase().as_str() {
                "jan" | "january" | "1" | "01" => Some(Month::January),
                "feb" | "february" | "2" | "02" => Some(Month::February),
                "mar" | "march" | "3" | "03" => Some(Month::March),
                "apr" | "april" | "4" | "04" => Some(Month::April),
                "may" | "5" | "05" => Some(Month::May),
                "jun" | "june" | "6" | "06" => Some(Month::June),
                "jul" | "july" | "7" | "07" => Some(Month::July),
                "aug" | "august" | "8" | "08" => Some(Month::August),
                "sep" | "september" | "9" | "09" => Some(Month::September),
                "oct" | "october" | "10" => Some(Month::October),
                "nov" | "november" | "11" => Some(Month::November),
                "dec" | "december" | "12" => Some(Month::December),
                _ => None,
            }
        }

    }

    /// Enumerator representing the binary value of the day
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum BiDay {
        /// The value `Zero` means that the item does not included in the calendar
        Zero,
        /// The value `One` means that the item is included in the calendar.
        One,
    }

    impl BiDay {
        /// Convert types from BiDay to u8
        pub fn to_u8(&self) -> u8 {
            match self {
                BiDay::Zero => 0,
                BiDay::One => 1,
            }
        }
        /// Convert types from u8 to BiDay
        pub fn from_u8(value: u8) -> Option<BiDay> {
            match value {
                0 => Some(BiDay::Zero),
                1 => Some(BiDay::One),
                _ => None,
            }
        }
    }

    #[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
    pub(crate) struct YearMonth {
        year: Year,
        month: Month,
    }

    impl YearMonth {
        pub fn new(year: Year, month: Month) -> Result<Self, String> {
            if month as u8 >= 1 && month as u8 <= 12 {
                Ok(YearMonth { year, month })
            } else {
                Err("Month should be between 1 and 12".to_string())
            }
        }

        /// Creates a `YearMonth` from a given `Date`
        pub fn from_date(date: Date) -> Self {
            let year = date.year() as Year;
            let month = Month::from_index(date.month() as u8).expect("Invalid month index from date");
            YearMonth { year, month }
        }

        pub fn next_month(&self) -> Option<Self> {
            let next_month_index = self.month.to_index() + 1;
            let next_year = self.year + 1;
            
            match Month::from_index(next_month_index) {
                Some(next_month) => Some(YearMonth { year: self.year, month: next_month }),
                None if next_month_index == 13 => Month::from_index(1).map(|january| YearMonth { year: next_year, month: january }),
                _ => None,
            }
        }
        
        pub fn previous_month(&self) -> Option<Self> {
            let prev_month_index = self.month.to_index().saturating_sub(1);
            
            match Month::from_index(prev_month_index) {
                Some(prev_month) => Some(YearMonth { year: self.year, month: prev_month }),
                None if prev_month_index == 0 => Month::from_index(12).map(|december| YearMonth { year: self.year - 1, month: december }),
                _ => None,
            }
        }
    }

    // Enum to describe the detected type of Spec from a string.
    #[derive(Debug, PartialEq, Eq)]
    pub enum SpecType {
        Single,
        Range,
        List,
        Invalid,
    }

    impl SpecType {
        /// Determines the type of specification from the given string.
        /// 
        /// # Arguments
        /// 
        /// * `input` - A string slice that might represent a specification.
        pub fn from_str(input: &str) -> SpecType {
            // Single year
            if input.parse::<Year>().is_ok() {
                return SpecType::Single;
            }

            // Year range
            if input.contains('-') {
                return SpecType::Range;
            }

            // Year list
            if input.contains(',') {
                return SpecType::List;
            }

            SpecType::Invalid
        }
    }

    #[derive(PartialEq, Eq)]
    pub enum YearSpec {
        Single(Year),
        Range(RangeInclusive<Year>),
        List(HashSet<Year>),
    }

    impl YearSpec {
        /// Parses a string into a `YearSpec`.
        /// 
        /// # Arguments
        /// 
        /// * `input` - A string slice that holds the representation of the `YearSpec`.
        /// 
        /// # Examples
        /// 
        /// ```
        /// let y = YearSpec::parse("2023").unwrap();
        /// assert_eq!(y, YearSpec::Single(2023));
        /// 
        /// let y = YearSpec::parse("2023-2025").unwrap();
        /// assert_eq!(y, YearSpec::Range(2023..=2025));
        /// 
        /// let y = YearSpec::parse("2023,2025,2027").unwrap();
        /// let mut set = HashSet::new();
        /// set.insert(2023);
        /// set.insert(2025);
        /// set.insert(2027);
        /// assert_eq!(y, YearSpec::List(set));
        /// ```
        pub fn parse(input: &str) -> Option<Self> {
            // Single year
            if let Ok(year) = input.parse::<Year>() {
                return Some(YearSpec::Single(year));
            }
    
            // Year range
            if let Some(idx) = input.find('-') {
                let start = input[..idx].trim().parse::<Year>().ok()?;
                let end = input[idx+1..].trim().parse::<Year>().ok()?;
                return Some(YearSpec::Range(start..=end));
            }
    
            // Year list
            let years: Result<HashSet<Year>, _> = input.split(',').map(str::trim).map(str::parse).collect();
            years.ok().map(YearSpec::List)
        }
    
        /// Determines if the given string can be parsed into a `YearSpec`.
        /// 
        /// # Arguments
        /// 
        /// * `input` - A string slice that might represent a `YearSpec`.
        pub fn is_valid(input: &str) -> bool {
            Self::parse(input).is_some()
        }

        fn to_year_month(&self) -> HashSet<(Year, Month)> {
            match self {
                YearSpec::Single(year) => (1..=12).map(|month_num| {
                    let month = Month::from_index(month_num).unwrap();
                    (*year, month)
                }).collect(),
                YearSpec::Range(range) => {
                    let mut result = HashSet::new();
                    for year in range.clone() {
                        for month_num in 1..=12 {
                            let month = Month::from_index(month_num).unwrap();
                            result.insert((year, month));
                        }
                    }
                    result
                },
                YearSpec::List(years) => {
                    let mut result = HashSet::new();
                    for year in years {
                        for month_num in 1..=12 {
                            let month = Month::from_index(month_num).unwrap();
                            result.insert((*year, month));
                        }
                    }
                    result
                }
            }
        }
        
    }

    impl Hash for YearSpec {
        fn hash<H: Hasher>(&self, state: &mut H) {
            match self {
                YearSpec::Single(year) => {
                    "Single".hash(state);
                    year.hash(state);
                },
                YearSpec::Range(range) => {
                    let start = range.start();
                    let end = range.end();
                    "Range".hash(state);
                    start.hash(state);
                    end.hash(state);
                },
                YearSpec::List(years) => {
                    "List".hash(state);
                    // Dado que Year es simplemente un alias para u16, podemos iterar y hacer hash para cada año en la lista
                    for year in years {
                        year.hash(state);
                    }
                }
            }
        }
    }
    
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub(crate) enum MonthSpec {
        Single(Month),
        Range(RangeInclusive<Month>),
        List(HashSet<Month>),
    }

    use std::hash::{Hash, Hasher};

    impl Hash for MonthSpec {
        fn hash<H: Hasher>(&self, state: &mut H) {
            match self {
                MonthSpec::Single(month) => {
                    1.hash(state);  // Represent Single variant with a unique value.
                    month.hash(state);
                },
                MonthSpec::Range(range) => {
                    let start: &Month = range.start();
                    let end: &Month = range.end();
                    2.hash(state);  // Represent Range variant with a unique value.
                    start.hash(state);
                    end.hash(state);
                },
                MonthSpec::List(months) => {
                    3.hash(state);  // Represent List variant with a unique value.
                    // Sort months and then hash them for consistency.
                    let mut sorted_months: Vec<_> = months.iter().collect();
                    sorted_months.sort();
                    for month in sorted_months {
                        month.hash(state);
                    }
                },
            }
        }
    }


    pub struct YearMonthSpec(HashSet<(YearSpec, MonthSpec)>);

    pub enum DateSpec {
        Single(Date),
        Range(Date, Date),
        List(HashSet<Date>),
    }

    pub enum DateSpan {
        Year(YearSpec),
        YearMonth(YearMonthSpec),
        Date(DateSpec),
    }

    

    impl MonthSpec {
        /// Verifies if a string is a valid representation of MonthSpec.
        /// 
        /// # Arguments
        /// 
        /// * `input` - A string slice that might represent a `MonthSpec`.
        pub fn is_valid(input: &str) -> bool {
            match SpecType::from_str(input) {
                SpecType::Invalid => false,
                _ => true,
            }
        }

        /// Parses a string to create a MonthSpec.
        /// 
        /// # Arguments
        /// 
        /// * `input` - A string slice that should represent a `MonthSpec`.
        pub fn parse(input: &str) -> Result<MonthSpec, &'static str> {
            match SpecType::from_str(input) {
                SpecType::Single => {
                    let month = Month::from_str(input).ok_or("Invalid month format")?;
                    Ok(MonthSpec::Single(month))
                },
                SpecType::Range => {
                    let parts: Vec<&str> = input.split('-').collect();
                    if parts.len() != 2 {
                        return Err("Invalid range format");
                    }
                    let start = Month::from_str(parts[0]).ok_or("Invalid start month format")?;
                    let end = Month::from_str(parts[1]).ok_or("Invalid end month format")?;
                    Ok(MonthSpec::Range(start..=end))
                },
                SpecType::List => {
                    let months = input.split(',')
                        .map(|s| s.trim())
                        .filter_map(Month::from_str)
                        .collect::<HashSet<Month>>();
                    if months.is_empty() {
                        return Err("Invalid list format");
                    }
                    Ok(MonthSpec::List(months))
                },
                SpecType::Invalid => Err("Invalid MonthSpec format"),
            }
        }

        
        fn to_year_month(&self, year: Year) -> HashSet<(Year, Month)> {
            match self {
                MonthSpec::Single(month) => vec![(year, *month)].into_iter().collect(),
                MonthSpec::Range(range) => {
                    let start = range.start().to_index();
                    let end = range.end().to_index();
                    (start..=end)
                        .filter_map(Month::from_index)
                        .map(|month| (year, month))
                        .collect()
                },
                MonthSpec::List(months) => months.iter().map(|&month| (year, month)).collect(),
            }
        }
    }
    

    impl YearMonthSpec {
        /// Checks if the given string represents a valid `YearMonthSpec`.
        /// 
        /// # Arguments
        /// 
        /// * `input` - A string slice that might represent a `YearMonthSpec`.
        /// 
        /// # Returns
        /// 
        /// * `bool` - Returns `true` if the string is a valid representation, otherwise returns `false`.
        pub fn is_valid(input: &str) -> bool {
            let parts: Vec<&str> = input.split('-').collect();
            if parts.len() != 2 {
                return false;
            }

            let year = parts[0].parse::<Year>();
            let month = Month::from_str(parts[1]);

            year.is_ok() && month.is_some()
        }

        /// Tries to parse the given string into a `YearMonthSpec`.
        /// 
        /// # Arguments
        /// 
        /// * `input` - A string slice that holds the representation of a `YearMonthSpec`.
        /// 
        /// # Returns
        /// 
        /// * `Result<YearMonthSpec, &'static str>` - Returns `Ok(YearMonthSpec)` if the string is a valid representation, 
        ///   otherwise returns an `Err` with a description of the error.
        pub fn parse(input: &str) -> Result<Self, &'static str> {
            if !Self::is_valid(input) {
                return Err("Invalid YearMonthSpec format");
            }

            let parts: Vec<&str> = input.split('-').collect();
            let year = parts[0].parse::<Year>().expect("Already validated");
            let month = Month::from_str(parts[1]).expect("Already validated");

            let mut set = HashSet::new();
            set.insert((YearSpec::Single(year), MonthSpec::Single(month)));

            Ok(YearMonthSpec(set))
        }

        fn to_year_month(&self) -> HashSet<(Year, Month)> {
            let mut result = HashSet::new();
            for (year_spec, month_spec) in &self.0 {
                let years = year_spec.to_year_month().into_iter().map(|(year, _)| year).collect::<HashSet<_>>();
                for &year in &years {
                    result.extend(month_spec.to_year_month(year));
                }
            }
            result
        }
    }

    impl DateSpec {
        
        fn to_year_month(&self) -> HashSet<(Year, Month)> {
            match self {
                DateSpec::Single(date) => {
                    let month = Month::from_index(date.month() as u8).unwrap();
                    vec![(date.year() as Year, month)].into_iter().collect()
                },
                DateSpec::Range(start, end) => {
                    let mut result = HashSet::new();
                    let end_ym = YearMonth::from_date(*end);
                    
                    // Convertimos la fecha inicial en YearMonth
                    let mut current_ym = YearMonth::from_date(*start);
                
                    while current_ym <= end_ym {
                        result.insert((current_ym.year, current_ym.month));
                
                        // Avanzamos al próximo mes
                        if let Some(next_ym) = current_ym.next_month() {
                            current_ym = next_ym;
                        } else {
                            break;
                        }
                    }
                    
                    result
                },                                
                DateSpec::List(dates) => {
                    dates.iter().map(|date| {
                        let month = Month::from_index(date.month() as u8).unwrap();
                        (date.year() as Year, month)
                    }).collect()
                }
            }
        }
    }

    impl DateSpan {
        fn to_year_month(&self) -> HashSet<(Year, Month)> {
            match self {
                DateSpan::Year(year_spec) => year_spec.to_year_month(),
                DateSpan::YearMonth(year_month_spec) => year_month_spec.to_year_month(),
                DateSpan::Date(date_spec) => date_spec.to_year_month(),
            }
        }
    }


    #[cfg(test)]
    mod tests_types {
        use super::*;

        #[test]
        fn month_to_index_test() {
            assert_eq!(Month::January.to_index(), 1);
            assert_eq!(Month::December.to_index(), 12);
        }

        #[test]
        fn month_from_index_test() {
            assert_eq!(Month::from_index(1), Some(Month::January));
            assert_eq!(Month::from_index(12), Some(Month::December));
            assert_eq!(Month::from_index(13), None);
        }

        #[test]
        fn month_from_index_outbound_no_panic_test() {
            assert_eq!(Month::from_index(13), None);
        }

        #[test]
        fn biday_to_u8_test() {
            assert_eq!(BiDay::Zero.to_u8(), 0);
            assert_eq!(BiDay::One.to_u8(), 1);
        }

        #[test]
        fn biday_from_u8_test() {
            assert_eq!(BiDay::from_u8(0), Some(BiDay::Zero));
            assert_eq!(BiDay::from_u8(1), Some(BiDay::One));
            assert_eq!(BiDay::from_u8(2), None);
        }

        #[test]
        fn test_from_index_valid_values() {
            assert_eq!(Month::from_index(1), Some(Month::January));
            assert_eq!(Month::from_index(2), Some(Month::February));
            assert_eq!(Month::from_index(3), Some(Month::March));
            assert_eq!(Month::from_index(4), Some(Month::April));
            assert_eq!(Month::from_index(5), Some(Month::May));
            assert_eq!(Month::from_index(6), Some(Month::June));
            assert_eq!(Month::from_index(7), Some(Month::July));
            assert_eq!(Month::from_index(8), Some(Month::August));
            assert_eq!(Month::from_index(9), Some(Month::September));
            assert_eq!(Month::from_index(10), Some(Month::October));
            assert_eq!(Month::from_index(11), Some(Month::November));
            assert_eq!(Month::from_index(12), Some(Month::December));
        }

        #[test]
        fn test_from_index_invalid_values() {
            assert_eq!(Month::from_index(0), None);
            assert_eq!(Month::from_index(13), None);
            assert_eq!(Month::from_index(100), None);
        }

        #[test]
        fn test_month_to_index() {
            assert_eq!(Month::January.to_index(), 1);
            assert_eq!(Month::February.to_index(), 2);
            // ... similar assertions for all months ...
            assert_eq!(Month::December.to_index(), 12);
        }

        #[test]
        fn test_month_from_index() {
            assert_eq!(Month::from_index(1), Some(Month::January));
            assert_eq!(Month::from_index(2), Some(Month::February));
            // ... similar assertions for all months ...
            assert_eq!(Month::from_index(12), Some(Month::December));
            assert_eq!(Month::from_index(0), None); // less than January
            assert_eq!(Month::from_index(13), None); // more than December
        }

        #[test]
        fn test_month_to_time_month() {
            assert_eq!(Month::January.to_time_month(), TimeMonth::January);
            assert_eq!(Month::February.to_time_month(), TimeMonth::February);
            // ... similar assertions for all months ...
            assert_eq!(Month::December.to_time_month(), TimeMonth::December);
        }

        #[test]
        fn test_biday_to_u8() {
            assert_eq!(BiDay::Zero.to_u8(), 0);
            assert_eq!(BiDay::One.to_u8(), 1);
        }

        #[test]
        fn test_biday_from_u8() {
            assert_eq!(BiDay::from_u8(0), Some(BiDay::Zero));
            assert_eq!(BiDay::from_u8(1), Some(BiDay::One));
            assert_eq!(BiDay::from_u8(2), None); // invalid value
        }

        #[test]
        fn test_year_month_new() {
            // Test with valid year and month
            assert_eq!(YearMonth::new(2023, Month::January), Ok(YearMonth { year: 2023, month: Month::January }));
            assert_eq!(YearMonth::new(2024, Month::December), Ok(YearMonth { year: 2024, month: Month::December }));
        }
        
        #[test]
        fn test_year_month_new_invalid_month() {
            // Test with invalid month
            let invalid_month = Month::from_index(13);
            assert!(invalid_month.is_none());

            if let Some(month) = invalid_month {
                assert_eq!(YearMonth::new(2023, month), Err("Month should be between 1 and 12".to_string()));
            }
        }


        #[test]
        fn test_year_month_next_month() {
            // Test with January, expecting February of the same year
            let jan = YearMonth { year: 2023, month: Month::January };
            assert_eq!(jan.next_month(), Some(YearMonth { year: 2023, month: Month::February }));

            // Test with December, expecting January of the next year
            let dec = YearMonth { year: 2023, month: Month::December };
            assert_eq!(dec.next_month(), Some(YearMonth { year: 2024, month: Month::January }));
        }

        #[test]
        fn test_year_month_previous_month() {
            // Test with February, expecting January of the same year
            let feb = YearMonth { year: 2023, month: Month::February };
            assert_eq!(feb.previous_month(), Some(YearMonth { year: 2023, month: Month::January }));

            // Test with January, expecting December of the previous year
            let jan = YearMonth { year: 2023, month: Month::January };
            assert_eq!(jan.previous_month(), Some(YearMonth { year: 2022, month: Month::December }));
        }

        #[test]
        fn test_year_spec_to_year_month() {
            // Test Single variant
            let single = YearSpec::Single(2023);
            let mut expected = HashSet::new();
            for month_num in 1..=12 {
                expected.insert((2023, Month::from_index(month_num).unwrap()));
            }
            assert_eq!(single.to_year_month(), expected);

            // Test Range variant
            let range = YearSpec::Range(2022..=2023);
            expected.clear();
            for year in 2022..=2023 {
                for month_num in 1..=12 {
                    expected.insert((year, Month::from_index(month_num).unwrap()));
                }
            }
            assert_eq!(range.to_year_month(), expected);

            // Test List variant
            let list = YearSpec::List(vec![2022, 2024].into_iter().collect());
            expected.clear();
            for &year in &[2022, 2024] {
                for month_num in 1..=12 {
                    expected.insert((year, Month::from_index(month_num).unwrap()));
                }
            }
            assert_eq!(list.to_year_month(), expected);
        }

        #[test]
        fn test_month_spec_to_year_month() {
            // Test Single variant
            let single = MonthSpec::Single(Month::January);
            let expected = vec![(2023, Month::January)].into_iter().collect();
            assert_eq!(single.to_year_month(2023), expected);

            // Test Range variant
            let range = MonthSpec::Range(Month::January..=Month::March);
            let expected = vec![
                (2023, Month::January),
                (2023, Month::February),
                (2023, Month::March)
            ].into_iter().collect();
            assert_eq!(range.to_year_month(2023), expected);

            // Test List variant
            let list = MonthSpec::List(vec![Month::January, Month::March].into_iter().collect());
            let expected = vec![
                (2023, Month::January),
                (2023, Month::March)
            ].into_iter().collect();
            assert_eq!(list.to_year_month(2023), expected);
        }

        #[test]
        fn test_date_spec_to_year_month_single() {
            // Test Single variant
            let single_date = date!(2023 - 01 - 15);
            let single = DateSpec::Single(single_date);
            let expected = vec![(2023, Month::January)].into_iter().collect();
            assert_eq!(single.to_year_month(), expected);

        }

        #[test]
        fn test_date_spec_to_year_month_range() {

            // Test Range variant
            let start_date = date!(2023 - 01 - 15);
            let end_date = date!(2023 - 03 - 15);
            let range = DateSpec::Range(start_date, end_date);
            let expected = vec![
                (2023, Month::January),
                (2023, Month::February),
                (2023, Month::March)
            ].into_iter().collect();
            assert_eq!(range.to_year_month(), expected);
        }

        #[test]
        fn test_date_spec_to_year_month_list() {

            // Test List variant
            let list_dates = vec![date!(2023 - 01 - 15), date!(2023 - 03 - 15)];
            let list = DateSpec::List(list_dates.into_iter().collect());
            let expected = vec![
                (2023, Month::January),
                (2023, Month::March)
            ].into_iter().collect();
            assert_eq!(list.to_year_month(), expected);
        }

        #[test]
        fn test_datespan_year_to_year_month() {
            // Test Single variant of YearSpec
            let single = YearSpec::Single(2023);
            let date_span = DateSpan::Year(single);
            let mut expected = HashSet::new();
            for month_num in 1..=12 {
                expected.insert((2023, Month::from_index(month_num).unwrap()));
            }
            assert_eq!(date_span.to_year_month(), expected);
        }

        #[test]
        fn test_datespan_year_month_to_year_month() {
            // Using a combination of YearSpec and MonthSpec
            let year_spec = YearSpec::Single(2023);
            let month_spec = MonthSpec::List(vec![Month::January, Month::March].into_iter().collect());
            let year_month_spec = YearMonthSpec(vec![(year_spec, month_spec)].into_iter().collect());
            let date_span = DateSpan::YearMonth(year_month_spec);
            
            let expected = vec![
                (2023, Month::January),
                (2023, Month::March)
            ].into_iter().collect();
            
            assert_eq!(date_span.to_year_month(), expected);
        }

        #[test]
        fn test_datespan_date_to_year_month() {
            // Test Single variant of DateSpec
            let single_date = date!(2023 - 01 - 15); // Asumiendo que tienes disponible el macro date!
            let single = DateSpec::Single(single_date);
            let date_span = DateSpan::Date(single);
            let expected = vec![(2023, Month::January)].into_iter().collect();
            assert_eq!(date_span.to_year_month(), expected);

            // Test Range variant of DateSpec
            let start_date = date!(2023 - 01 - 15);
            let end_date = date!(2023 - 03 - 15);
            let range = DateSpec::Range(start_date, end_date);
            let date_span = DateSpan::Date(range);
            let expected = vec![
                (2023, Month::January),
                (2023, Month::February),
                (2023, Month::March)
            ].into_iter().collect();
            assert_eq!(date_span.to_year_month(), expected);

            // Test List variant of DateSpec
            let list_dates = vec![date!(2023 - 01 - 15), date!(2023 - 03 - 15)];
            let list = DateSpec::List(list_dates.into_iter().collect());
            let date_span = DateSpan::Date(list);
            let expected = vec![
                (2023, Month::January),
                (2023, Month::March)
            ].into_iter().collect();
            assert_eq!(date_span.to_year_month(), expected);
        }











    }

}

