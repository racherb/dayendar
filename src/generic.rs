/// The `types` module provides the general data types used in the system
pub mod types {

    /// Represents the months of the year and comes directly from `time::Month`.
    pub use time::Month as TimeMonth;
    pub use time::{Date as Date, Duration as Duration, Weekday as Weekday};
    /// Re-export: The macro `date`. Comes directly from `time::macros::date`.
    pub use time::macros::date;
    use core::ops::RangeInclusive;
    use std::collections::HashSet;
    use std::str::FromStr;
 

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
        //#[flux::sig(fn(x: i32{v: 0 <= v}) -> i32{res: res >= x})] 
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

        pub fn next(&self) -> Option<Self> {
            match self {
                Month::December => None,
                _ => Some(unsafe { std::mem::transmute((*self as u8) + 1) }),
            }
        }
    
        pub fn prev(&self) -> Option<Self> {
            match self {
                Month::January => None,
                _ => Some(unsafe { std::mem::transmute((*self as u8) - 1) }),
            }
        }
    
        pub fn forward(&self, count: usize) -> Option<Self> {
            let mut current = *self;
            for _ in 0..count {
                match current.next() {
                    Some(next_month) => current = next_month,
                    None => return None,
                }
            }
            Some(current)
        }
    
        pub fn backward(&self, count: usize) -> Option<Self> {
            let mut current = *self;
            for _ in 0..count {
                match current.prev() {
                    Some(prev_month) => current = prev_month,
                    None => return None,
                }
            }
            Some(current)
        }
    
        pub fn range_to(&self, end: &Self) -> Vec<Self> {
            let mut current = *self;
            let mut months = vec![];
            while current != *end {
                months.push(current);
                match current.next() {
                    Some(next_month) => current = next_month,
                    None => break,
                }
            }
            months.push(*end);
            months
        }

    }

    impl FromStr for Month {
        type Err = ();

        /// Converts a string representation of a month into a `Month`.
        /// 
        /// # Arguments
        /// 
        /// * `s` - A string slice that holds the representation of a month.
        /// 
        /// # Returns
        /// 
        /// * `Option<Month>` - Returns `Some(Month)` if the string is a valid representation, otherwise returns `None`.
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s.to_lowercase().as_str() {
                "jan" | "january" | "1" | "01" => Ok(Month::January),
                "feb" | "february" | "2" | "02" => Ok(Month::February),
                "mar" | "march" | "3" | "03" => Ok(Month::March),
                "apr" | "april" | "4" | "04" => Ok(Month::April),
                "may" | "5" | "05" => Ok(Month::May),
                "jun" | "june" | "6" | "06" => Ok(Month::June),
                "jul" | "july" | "7" | "07" => Ok(Month::July),
                "aug" | "august" | "8" | "08" => Ok(Month::August),
                "sep" | "september" | "9" | "09" => Ok(Month::September),
                "oct" | "october" | "10" => Ok(Month::October),
                "nov" | "november" | "11" => Ok(Month::November),
                "dec" | "december" | "12" => Ok(Month::December),
                _ => Err(()),
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

    #[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
    pub(crate) struct YearMonth {
        year: Year,
        month: Month,
    }

    impl YearMonth {
        #[allow(dead_code)]
        pub fn new(year: Year, month: Month) -> Result<Self, String> {
            if month as u8 >= 1 && month as u8 <= 12 {
                Ok(YearMonth { year, month })
            } else {
                Err("Month should be between 1 and 12".to_string())
            }
        }

        /// Creates a `YearMonth` from a given `Date`
        pub fn from_date(date: Date) -> Self {
            let year: u16 = date.year() as Year;
            let month: Month = Month::from_index(date.month() as u8).expect("Invalid month index from date");
            YearMonth { year, month }
        }

        pub fn next_month(&self) -> Option<Self> {
            let next_month_index: u8 = self.month.to_index() + 1;
            let next_year: u16 = self.year + 1;
            
            match Month::from_index(next_month_index) {
                Some(next_month) => Some(YearMonth { year: self.year, month: next_month }),
                None if next_month_index == 13 => Month::from_index(1).map(|january: Month| YearMonth { year: next_year, month: january }),
                _ => None,
            }
        }
        
        pub fn previous_month(&self) -> Option<Self> {
            let prev_month_index = self.month.to_index().saturating_sub(1);
            
            match Month::from_index(prev_month_index) {
                Some(prev_month) => Some(YearMonth { year: self.year, month: prev_month }),
                None if prev_month_index == 0 => Month::from_index(12).map(|december: Month| YearMonth { year: self.year - 1, month: december }),
                _ => None,
            }
        }

        pub fn next(&self) -> Self {
            match self.month {
                Month::December => YearMonth {
                    year: self.year + 1,
                    month: Month::January,
                },
                _ => YearMonth {
                    year: self.year,
                    month: self.month.next().unwrap(),  // Es seguro porque el mes siguiente a December es manejado arriba
                },
            }
        }
    
        pub fn prev(&self) -> Self {
            match self.month {
                Month::January => YearMonth {
                    year: self.year - 1,
                    month: Month::December,
                },
                _ => YearMonth {
                    year: self.year,
                    month: self.month.prev().unwrap(),  // Es seguro porque el mes anterior a January es manejado arriba
                },
            }
        }
    
        pub fn forward(&self, months: usize) -> Self {
            let mut result: YearMonth = self.clone();
            for _ in 0..months {
                result = result.next();
            }
            result
        }
    
        pub fn backward(&self, months: usize) -> Self {
            let mut result: YearMonth = self.clone();
            for _ in 0..months {
                result = result.prev();
            }
            result
        }
    
        pub fn range_to(&self, end: &Self) -> Vec<Self> {
            let mut current = self.clone();
            let mut months = vec![];
            while current != *end {
                months.push(current.clone());
                current = current.next();
            }
            months.push(end.clone());
            months
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

    impl FromStr for SpecType {
        type Err = ();
    
        fn from_str(input: &str) -> Result<Self, Self::Err> {
            // Single month
            if Month::from_str(input).is_ok() {
                return Ok(SpecType::Single);
            }
            // Single year
            if input.parse::<Year>().is_ok() {
                return Ok(SpecType::Single);
            }
            // Year range
            if input.contains('-') {
                return Ok(SpecType::Range);
            }
            // Year list
            if input.contains(',') {
                return Ok(SpecType::List);
            }
            Ok(SpecType::Invalid)
        }
    }

    #[derive(Debug, Clone)]
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
        /// ```rust
        /// use dayendar::types::YearSpec;
        /// use std::collections::HashSet;
        /// 
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
                let start: u16 = input[..idx].trim().parse::<Year>().ok()?;
                let end: u16 = input[idx+1..].trim().parse::<Year>().ok()?;
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

        pub fn to_year_month(&self) -> HashSet<(Year, Month)> {
            match self {
                YearSpec::Single(year) => (1..=12).map(|month_num: u8| {
                    let month: Month = Month::from_index(month_num).unwrap();
                    (*year, month)
                }).collect(),
                YearSpec::Range(range) => {
                    let mut result: HashSet<(u16, Month)> = HashSet::new();
                    for year in range.clone() {
                        for month_num in 1..=12 {
                            let month: Month = Month::from_index(month_num).unwrap();
                            result.insert((year, month));
                        }
                    }
                    result
                },
                YearSpec::List(years) => {
                    let mut result: HashSet<(u16, Month)> = HashSet::new();
                    for year in years {
                        for month_num in 1..=12 {
                            let month: Month = Month::from_index(month_num).unwrap();
                            result.insert((*year, month));
                        }
                    }
                    result
                }
            }
        }

        pub fn next(&self) -> Self {
            match self {
                YearSpec::Single(year) => YearSpec::Single(year + 1),
                YearSpec::Range(range) => YearSpec::Range((range.start() + 1)..=(range.end() + 1)),
                YearSpec::List(set) => {
                    let new_set: HashSet<_> = set.iter().map(|&year| year + 1).collect();
                    YearSpec::List(new_set)
                }
            }
        }
    
        pub fn prev(&self) -> Self {
            match self {
                YearSpec::Single(year) => YearSpec::Single(year - 1),
                YearSpec::Range(range) => YearSpec::Range((range.start() - 1)..=(range.end() - 1)),
                YearSpec::List(set) => {
                    let new_set: HashSet<_> = set.iter().map(|&year| year - 1).collect();
                    YearSpec::List(new_set)
                }
            }
        }
    
        pub fn forward(&self, years: usize) -> Self {
            let mut result = self.clone();
            for _ in 0..years {
                result = result.next();
            }
            result
        }
    
        pub fn backward(&self, years: usize) -> Self {
            let mut result = self.clone();
            for _ in 0..years {
                result = result.prev();
            }
            result
        }

        pub fn range_to(&self, end: &Self) -> Self {
            match self {
                YearSpec::Single(year) => YearSpec::Range(*year..=match end {
                    YearSpec::Single(end_year) => *end_year,
                    YearSpec::Range(range) => *range.end(),
                    YearSpec::List(set) => *set.iter().max().unwrap(),
                }),
                YearSpec::Range(range) => YearSpec::Range(*range.start()..=match end {
                    YearSpec::Single(end_year) => *end_year,
                    YearSpec::Range(end_range) => *end_range.end(),
                    YearSpec::List(set) => *set.iter().max().unwrap(),
                }),
                YearSpec::List(set) => {
                    let min_year = *set.iter().min().unwrap();
                    let end_year = match end {
                        YearSpec::Single(year) => *year,
                        YearSpec::Range(range) => *range.end(),
                        YearSpec::List(end_set) => *end_set.iter().max().unwrap(),
                    };
                    let new_set: HashSet<_> = (min_year..=end_year).collect();
                    YearSpec::List(new_set)
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
                    let start: &u16 = range.start();
                    let end: &u16 = range.end();
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

    impl PartialEq for YearSpec {
        fn eq(&self, other: &Self) -> bool {
            match (self, other) {
                (YearSpec::Single(y1), YearSpec::Single(y2)) => y1 == y2,
                (YearSpec::Single(y), YearSpec::Range(range)) 
                | (YearSpec::Range(range), YearSpec::Single(y)) => range.start() == y && range.end() == y,
                (YearSpec::Single(y), YearSpec::List(set)) 
                | (YearSpec::List(set), YearSpec::Single(y)) => set.contains(y) && set.len() == 1,
                (YearSpec::Range(r1), YearSpec::Range(r2)) => r1.start() == r2.start() && r1.end() == r2.end(),
                (YearSpec::Range(range), YearSpec::List(set)) 
                | (YearSpec::List(set), YearSpec::Range(range)) => {
                    if set.len() != (range.end() - range.start() + 1) as usize {
                        return false;
                    }
                    for year in *range.start()..=*range.end() {
                        if !set.contains(&year) {
                            return false;
                        }
                    }
                    true
                },
                (YearSpec::List(set1), YearSpec::List(set2)) => set1 == set2,
                //_ => false,
            }
        }
    }
    
    impl Eq for YearSpec {}
    
    #[derive(Debug, Clone)]
    pub enum MonthSpec {
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

    impl PartialEq for MonthSpec {
        fn eq(&self, other: &Self) -> bool {
            match (self, other) {
                (MonthSpec::Single(m1), MonthSpec::Single(m2)) => m1 == m2,
    
                (MonthSpec::Single(m), MonthSpec::Range(range))
                | (MonthSpec::Range(range), MonthSpec::Single(m)) => {
                    let start = range.start();
                    let end = range.end();
                    let months_range = start.range_to(end);
                    months_range.len() == 1 && &months_range[0] == m
                },
    
                (MonthSpec::Single(m), MonthSpec::List(set)) 
                | (MonthSpec::List(set), MonthSpec::Single(m)) => set.contains(m) && set.len() == 1,
    
                (MonthSpec::Range(range1), MonthSpec::Range(range2)) => {
                    let range1_vec = range1.start().range_to(range1.end());
                    let range2_vec = range2.start().range_to(range2.end());
                    range1_vec == range2_vec
                },
    
                (MonthSpec::Range(range), MonthSpec::List(set))
                | (MonthSpec::List(set), MonthSpec::Range(range)) => {
                    let months_range: Vec<Month> = range.start().range_to(range.end());
                    if set.len() != months_range.len() {
                        return false;
                    }
                    for month in months_range {
                        if !set.contains(&month) {
                            return false;
                        }
                    }
                    true
                },
    
                (MonthSpec::List(set1), MonthSpec::List(set2)) => set1 == set2,
            }
        }
    }
    
    
    impl Eq for MonthSpec {}


    #[derive(Debug, Clone)]
    pub struct YearMonthSpec(pub HashSet<(YearSpec, MonthSpec)>);

    impl PartialEq for YearMonthSpec {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0
        }
    }
    
    impl Eq for YearMonthSpec {}

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum DateSpec {
        Single(Date),
        Range(Date, Date),
        List(HashSet<Date>),
    }

    impl MonthSpec {
        /// Verifies if a string is a valid representation of MonthSpec.
        /// 
        /// # Arguments
        /// 
        /// * `input` - A string slice that might represent a `MonthSpec`.
        pub fn is_valid(input: &str) -> bool {
            !matches!(SpecType::from_str(input), Ok(SpecType::Invalid))
        }

        /// Parses a string to create a MonthSpec.
        /// 
        /// # Arguments
        /// 
        /// * `input` - A string slice that should represent a `MonthSpec`.
        pub fn parse(input: &str) -> Result<MonthSpec, &'static str> {
            match SpecType::from_str(input) {
                Ok(SpecType::Single) => {
                    let month: Month = Month::from_str(input).map_err(|_| "Invalid month format")?;
                    Ok(MonthSpec::Single(month))
                },
                Ok(SpecType::Range) => {
                    let parts: Vec<&str> = input.split('-').collect();
                    if parts.len() != 2 {
                        return Err("Invalid range format");
                    }
                    let start: Month = Month::from_str(parts[0]).map_err(|_| "Invalid month format")?;
                    let end = Month::from_str(parts[1]).map_err(|_| "Invalid end month format")?;
                    Ok(MonthSpec::Range(start..=end))
                },
                Ok(SpecType::List) => {
                    let months: HashSet<Month> = input.split(',')
                        .map(|s: &str| s.trim())
                        .filter_map(|input| Month::from_str(input).ok())
                        .collect::<HashSet<Month>>();
                    if months.is_empty() {
                        return Err("Invalid list format");
                    }
                    Ok(MonthSpec::List(months))
                },
                Ok(SpecType::Invalid) => Err("Invalid MonthSpec format"),
                _ => Err("Invalid MonthSpec format"),
            }
        }

        
        pub fn to_year_month(&self, year: Year) -> HashSet<(Year, Month)> {
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

        pub fn next(&self) -> Self {
            match self {
                MonthSpec::Single(month) => MonthSpec::Single(month.next().unwrap_or(Month::January)),
                MonthSpec::Range(range) => {
                    let new_start = range.start().next().unwrap_or(Month::January);
                    let new_end = range.end().next().unwrap_or(Month::January);
                    MonthSpec::Range(new_start..=new_end)
                }
                MonthSpec::List(set) => {
                    let new_set: HashSet<_> = set.iter().filter_map(|&month| month.next()).collect();
                    MonthSpec::List(new_set)
                }
            }
        }
    
        pub fn prev(&self) -> Self {
            match self {
                MonthSpec::Single(month) => MonthSpec::Single(month.prev().unwrap_or(Month::December)),
                MonthSpec::Range(range) => {
                    let new_start = range.start().prev().unwrap_or(Month::December);
                    let new_end = range.end().prev().unwrap_or(Month::December);
                    MonthSpec::Range(new_start..=new_end)
                }
                MonthSpec::List(set) => {
                    let new_set: HashSet<_> = set.iter().filter_map(|&month| month.prev()).collect();
                    MonthSpec::List(new_set)
                }
            }
        }
    
        pub fn forward(&self, months: usize) -> Self {
            let mut result = self.clone();
            for _ in 0..months {
                result = result.next();
            }
            result
        }
    
        pub fn backward(&self, months: usize) -> Self {
            let mut result = self.clone();
            for _ in 0..months {
                result = result.prev();
            }
            result
        }

        pub fn range_to(&self, end: &Self) -> Self {
            match self {
                MonthSpec::Single(month) => {
                    let end_month: &Month = match end {
                        MonthSpec::Single(end_month) => end_month,
                        MonthSpec::Range(range) => range.end(),
                        MonthSpec::List(set) => set.iter().max().unwrap(),
                    };
                    MonthSpec::Range(*month..=*end_month)
                }
                MonthSpec::Range(range) => {
                    let end_month: &Month = match end {
                        MonthSpec::Single(end_month) => end_month,
                        MonthSpec::Range(end_range) => end_range.end(),
                        MonthSpec::List(set) => set.iter().max().unwrap(),
                    };
                    MonthSpec::Range(*range.start()..=*end_month)
                }
                MonthSpec::List(set) => {
                    let start_month = set.iter().min().unwrap();
                    let end_month: &Month = match end {
                        MonthSpec::Single(month) => month,
                        MonthSpec::Range(range) => range.end(),
                        MonthSpec::List(end_set) => end_set.iter().max().unwrap(),
                    };
                    let months_range: Vec<Month> = start_month.range_to(end_month);
                    MonthSpec::List(months_range.into_iter().collect())
                }
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

            year.is_ok() && month.is_ok()
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

        pub fn to_year_month(&self) -> HashSet<(Year, Month)> {
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
        
        pub fn to_year_month(&self) -> HashSet<(Year, Month)> {
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

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum DateSpan {
        Year(YearSpec),
        YearMonth(YearMonthSpec),
        Date(DateSpec),
    }

    impl DateSpan {
        pub fn to_year_month(&self) -> HashSet<(Year, Month)> {
            match self {
                DateSpan::Year(year_spec) => year_spec.to_year_month(),
                DateSpan::YearMonth(year_month_spec) => year_month_spec.to_year_month(),
                DateSpan::Date(date_spec) => date_spec.to_year_month(),
            }
        }

        /*pub fn intersect(&self, other: &DateSpan) -> Option<DateSpan> {
            
        }
        */

        
    }



    
#[cfg(test)]
mod tests_generic_types {
    use super::*;
    use std::collections::HashSet;


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
    fn month_next_prev() {
        assert_eq!(Month::January.next(), Some(Month::February));
        assert_eq!(Month::December.next(), None);
        assert_eq!(Month::December.prev(), Some(Month::November));
        assert_eq!(Month::January.prev(), None);
    }

    #[test]
    fn month_forward_backward() {
        assert_eq!(Month::January.forward(2), Some(Month::March));
        assert_eq!(Month::December.forward(1), None);
        assert_eq!(Month::December.backward(2), Some(Month::October));
        assert_eq!(Month::January.backward(1), None);
    }

    #[test]
    fn month_range_to() {
        let expected = vec![Month::February, Month::March, Month::April];
        assert_eq!(Month::February.range_to(&Month::April), expected);
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
    fn test_yearmonth_next_prev() {
        assert_eq!(
            YearMonth { year: 2023, month: Month::December }.next(),
            YearMonth { year: 2024, month: Month::January }
        );
        assert_eq!(
            YearMonth { year: 2023, month: Month::January }.prev(),
            YearMonth { year: 2022, month: Month::December }
        );
        assert_eq!(
            YearMonth { year: 2023, month: Month::June }.next(),
            YearMonth { year: 2023, month: Month::July }
        );
        assert_eq!(
            YearMonth { year: 2023, month: Month::June }.prev(),
            YearMonth { year: 2023, month: Month::May }
        );
    }

    #[test]
    fn test_yearmonth_forward_backward() {
        assert_eq!(
            YearMonth { year: 2023, month: Month::June }.forward(6),
            YearMonth { year: 2023, month: Month::December }
        );
        assert_eq!(
            YearMonth { year: 2023, month: Month::June }.forward(7),
            YearMonth { year: 2024, month: Month::January }
        );
        assert_eq!(
            YearMonth { year: 2023, month: Month::June }.backward(5),
            YearMonth { year: 2023, month: Month::January }
        );
        assert_eq!(
            YearMonth { year: 2023, month: Month::June }.backward(6),
            YearMonth { year: 2022, month: Month::December }
        );
    }

    #[test]
    fn test_yearmonth_range_to() {
        let range = YearMonth { year: 2023, month: Month::January }.range_to(&YearMonth { year: 2023, month: Month::March });
        let expected = vec![
            YearMonth { year: 2023, month: Month::January },
            YearMonth { year: 2023, month: Month::February },
            YearMonth { year: 2023, month: Month::March }
        ];
        assert_eq!(range, expected);
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
        let range: MonthSpec = MonthSpec::Range(Month::January..=Month::March);
        let expected: HashSet<(u16, Month)> = vec![
            (2023, Month::January),
            (2023, Month::February),
            (2023, Month::March)
        ].into_iter().collect();
        assert_eq!(range.to_year_month(2023), expected);

        // Test List variant
        let list: MonthSpec = MonthSpec::List(vec![Month::January, Month::March].into_iter().collect());
        let expected: HashSet<(u16, Month)> = vec![
            (2023, Month::January),
            (2023, Month::March)
        ].into_iter().collect();
        assert_eq!(list.to_year_month(2023), expected);
    }

    #[test]
    fn test_date_spec_to_year_month_single() {
        // Test Single variant
        let single_date: Date = date!(2023 - 01 - 15);
        let single: DateSpec = DateSpec::Single(single_date);
        let expected: HashSet<(u16, Month)> = vec![(2023, Month::January)].into_iter().collect();
        assert_eq!(single.to_year_month(), expected);

    }

    #[test]
    fn test_date_spec_to_year_month_range() {

        // Test Range variant
        let start_date: Date = date!(2023 - 01 - 15);
        let end_date: Date = date!(2023 - 03 - 15);
        let range: DateSpec = DateSpec::Range(start_date, end_date);
        let expected: HashSet<(u16, Month)> = vec![
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
        let single: YearSpec = YearSpec::Single(2023);
        let date_span: DateSpan = DateSpan::Year(single);
        let mut expected: HashSet<(u16, Month)> = HashSet::new();
        for month_num in 1..=12 {
            expected.insert((2023, Month::from_index(month_num).unwrap()));
        }
        assert_eq!(date_span.to_year_month(), expected);
    }

    #[test]
    fn test_datespan_year_month_to_year_month() {
        // Using a combination of YearSpec and MonthSpec
        let year_spec: YearSpec = YearSpec::Single(2023);
        let month_spec: MonthSpec = MonthSpec::List(vec![Month::January, Month::March].into_iter().collect());
        let year_month_spec: YearMonthSpec = YearMonthSpec(vec![(year_spec, month_spec)].into_iter().collect());
        let date_span: DateSpan = DateSpan::YearMonth(year_month_spec);
        
        let expected: HashSet<(u16, Month)> = vec![
            (2023, Month::January),
            (2023, Month::March)
        ].into_iter().collect();
        
        assert_eq!(date_span.to_year_month(), expected);
    }

    #[test]
    fn test_datespan_date_to_year_month() {
        // Test Single variant of DateSpec
        let single_date: Date = date!(2023 - 01 - 15);
        let single: DateSpec = DateSpec::Single(single_date);
        let date_span: DateSpan = DateSpan::Date(single);
        let expected: HashSet<(u16, Month)> = vec![(2023, Month::January)].into_iter().collect();
        assert_eq!(date_span.to_year_month(), expected);

        // Test Range variant of DateSpec
        let start_date: Date = date!(2023 - 01 - 15);
        let end_date: Date = date!(2023 - 03 - 15);
        let range: DateSpec = DateSpec::Range(start_date, end_date);
        let date_span: DateSpan = DateSpan::Date(range);
        let expected: HashSet<(u16, Month)> = vec![
            (2023, Month::January),
            (2023, Month::February),
            (2023, Month::March)
        ].into_iter().collect();
        assert_eq!(date_span.to_year_month(), expected);

        // Test List variant of DateSpec
        let list_dates: Vec<Date> = vec![date!(2023 - 01 - 15), date!(2023 - 03 - 15)];
        let list: DateSpec = DateSpec::List(list_dates.into_iter().collect());
        let date_span: DateSpan = DateSpan::Date(list);
        let expected: HashSet<(u16, Month)> = vec![
            (2023, Month::January),
            (2023, Month::March)
        ].into_iter().collect();
        assert_eq!(date_span.to_year_month(), expected);
    }

    #[test]
    fn test_month_to_time_month_2() {
        assert_eq!(Month::January.to_time_month(), TimeMonth::January);
        assert_eq!(Month::December.to_time_month(), TimeMonth::December);
    }

    #[test]
    fn test_month_from_str() {
        assert_eq!(Month::from_str("jan"), Ok(Month::January));
        assert_eq!(Month::from_str("december"), Ok(Month::December));
        assert_eq!(Month::from_str("13"), Err(()));
        assert_eq!(Month::from_str("invalid"), Err(()));
    }

    #[test]
    fn test_yearmonth_new() {
        assert_eq!(YearMonth::new(2023, Month::January), Ok(YearMonth { year: 2023, month: Month::January }));
        assert_eq!(YearMonth::new(2023, Month::from_index(1).unwrap()), Ok(YearMonth { year: 2023, month: Month::January }));
    }

    #[test]
    fn test_yearmonth_from_date() {
        let date = date!(2023 - 01 - 15);
        assert_eq!(YearMonth::from_date(date), YearMonth { year: 2023, month: Month::January });
    }

    #[test]
    fn test_yearmonth_next_month() {
        let january = YearMonth { year: 2023, month: Month::January };
        let february = YearMonth { year: 2023, month: Month::February };
        let december = YearMonth { year: 2023, month: Month::December };
        let next_january = YearMonth { year: 2024, month: Month::January };

        assert_eq!(january.next_month(), Some(february));
        assert_eq!(december.next_month(), Some(next_january));
    }

    #[test]
    fn test_yearmonth_previous_month() {
        let january = YearMonth { year: 2023, month: Month::January };
        let previous_december = YearMonth { year: 2022, month: Month::December };
        let february = YearMonth { year: 2023, month: Month::February };
        let january_again = YearMonth { year: 2023, month: Month::January };

        assert_eq!(january.previous_month(), Some(previous_december));
        assert_eq!(february.previous_month(), Some(january_again));
    }

    #[test]
    fn test_yearspec_parse() {
        assert_eq!(YearSpec::parse("2023"), Some(YearSpec::Single(2023)));
        
        assert_eq!(YearSpec::parse("2023-2025"), Some(YearSpec::Range(2023..=2025)));

        let mut set = HashSet::new();
        set.insert(2023);
        set.insert(2025);
        set.insert(2027);
        assert_eq!(YearSpec::parse("2023,2025,2027"), Some(YearSpec::List(set)));
    }

    #[test]
    fn test_yearspec_is_valid() {
        assert!(YearSpec::is_valid("2023"));
        assert!(YearSpec::is_valid("2023-2025"));
        assert!(YearSpec::is_valid("2023,2025,2027"));
        assert!(!YearSpec::is_valid("invalid_input"));
    }

    #[test]
    fn test_yearspec_to_year_month() {
        let year_spec_single = YearSpec::Single(2023);
        let mut expected_single = HashSet::new();
        for month_num in 1..=12 {
            let month = Month::from_index(month_num).unwrap();
            expected_single.insert((2023, month));
        }
        assert_eq!(year_spec_single.to_year_month(), expected_single);

        let year_spec_range = YearSpec::Range(2023..=2024);
        let mut expected_range = HashSet::new();
        for year in 2023..=2024 {
            for month_num in 1..=12 {
                let month = Month::from_index(month_num).unwrap();
                expected_range.insert((year, month));
            }
        }
        assert_eq!(year_spec_range.to_year_month(), expected_range);

        let mut set = HashSet::new();
        set.insert(2023);
        set.insert(2025);
        let year_spec_list = YearSpec::List(set);
        let mut expected_list = HashSet::new();
        for &year in &[2023, 2025] {
            for month_num in 1..=12 {
                let month = Month::from_index(month_num).unwrap();
                expected_list.insert((year, month));
            }
        }
        assert_eq!(year_spec_list.to_year_month(), expected_list);
    }

    #[test]
    fn test_monthspec_is_valid() {
        assert!(MonthSpec::is_valid("January"));
        assert!(MonthSpec::is_valid("Jan-Feb"));
        assert!(MonthSpec::is_valid("Jan,Feb,Mar"));
        assert!(!MonthSpec::is_valid("invalid_input"));
    }

    #[test]
    fn test_monthspec_parse() {
        assert_eq!(MonthSpec::parse("January"), Ok(MonthSpec::Single(Month::January)));
        
        assert_eq!(MonthSpec::parse("Jan-Feb"), Ok(MonthSpec::Range(Month::January..=Month::February)));

        let mut set = HashSet::new();
        set.insert(Month::January);
        set.insert(Month::February);
        set.insert(Month::March);
        assert_eq!(MonthSpec::parse("Jan,Feb,Mar"), Ok(MonthSpec::List(set)));
    }

    #[test]
    fn test_monthspec_to_year_month() {
        let year = 2023;
        let month_spec_single = MonthSpec::Single(Month::January);
        assert_eq!(month_spec_single.to_year_month(year), vec![(year, Month::January)].into_iter().collect());

        let month_spec_range = MonthSpec::Range(Month::January..=Month::March);
        let mut expected_range = HashSet::new();
        for month in &[Month::January, Month::February, Month::March] {
            expected_range.insert((year, *month));
        }
        assert_eq!(month_spec_range.to_year_month(year), expected_range);

        let mut set = HashSet::new();
        set.insert(Month::January);
        set.insert(Month::March);
        let month_spec_list = MonthSpec::List(set);
        let mut expected_list = HashSet::new();
        for &month in &[Month::January, Month::March] {
            expected_list.insert((year, month));
        }
        assert_eq!(month_spec_list.to_year_month(year), expected_list);
    }

    #[test]
    fn test_yearmonthspec_is_valid() {
        assert!(YearMonthSpec::is_valid("2023-January"));
        assert!(!YearMonthSpec::is_valid("invalid_input"));
        assert!(!YearMonthSpec::is_valid("Jan-2023"));
    }

    #[test]
    fn test_yearmonthspec_parse() {
        let year = 2023;
        let month = Month::January;
        let mut set = HashSet::new();
        set.insert((YearSpec::Single(year), MonthSpec::Single(month)));
        assert_eq!(YearMonthSpec::parse("2023-January"), Ok(YearMonthSpec(set)));
    }

    #[test]
    fn test_yearmonthspec_to_year_month() {
        let year = 2023;
        let month = Month::January;
        let mut set = HashSet::new();
        set.insert((YearSpec::Single(year), MonthSpec::Single(month)));
        let year_month_spec = YearMonthSpec(set);
        assert_eq!(year_month_spec.to_year_month(), vec![(year, month)].into_iter().collect());
    }

    #[test]
    fn test_biday_conversion() {
        assert_eq!(BiDay::from_u8(0), Some(BiDay::Zero));
        assert_eq!(BiDay::from_u8(1), Some(BiDay::One));
        assert_eq!(BiDay::from_u8(2), None); // Invalid value
        assert_eq!(BiDay::Zero.to_u8(), 0);
        assert_eq!(BiDay::One.to_u8(), 1);
    }

    #[test]
    fn test_datespec_to_year_month() {
        let single_date = DateSpec::Single(date!(2023 - 05 - 15));
        let range_date = DateSpec::Range(date!(2022 - 05 - 15), date!(2024 - 07 - 15));
        let mut dates = HashSet::new();
        dates.insert(date!(2021 - 01 - 01));
        dates.insert(date!(2021 - 12 - 31));
        let list_date = DateSpec::List(dates);

        assert_eq!(single_date.to_year_month(), vec![(2023, Month::May)].into_iter().collect());
        assert_eq!(list_date.to_year_month(), vec![(2021, Month::January), (2021, Month::December)].into_iter().collect());

        let range_output = range_date.to_year_month();
        assert!(range_output.contains(&(2022, Month::May)));
        assert!(range_output.contains(&(2024, Month::July)));
    }

    #[test]
    fn test_datespan_to_year_month() {
        let single_year = DateSpan::Year(YearSpec::Single(2023));
        let mut year_month_set = HashSet::new();
        year_month_set.insert((YearSpec::Single(2023), MonthSpec::Single(Month::May)));
        let year_month = DateSpan::YearMonth(YearMonthSpec(year_month_set));
        let single_date = DateSpan::Date(DateSpec::Single(date!(2023 - 05 - 15)));

        assert!(single_year.to_year_month().contains(&(2023, Month::May)));
        assert!(year_month.to_year_month().contains(&(2023, Month::May)));
        assert!(single_date.to_year_month().contains(&(2023, Month::May)));
    }

    #[test]
    fn test_yearspec_is_valid_2() {
        assert!(YearSpec::is_valid("2023"));
        assert!(YearSpec::is_valid("2020-2025"));
        assert!(YearSpec::is_valid("2020,2022,2024"));

        // Casos no válidos
        assert!(!YearSpec::is_valid("20a3"));
        assert!(!YearSpec::is_valid("2020-20a5"));
        assert!(!YearSpec::is_valid("2020,20a2,2024"));
    }

    #[test]
    fn test_yearspec_parse_2() {
        assert_eq!(YearSpec::parse("2023").unwrap(), YearSpec::Single(2023));

        let range = 2020..=2025;
        assert_eq!(YearSpec::parse("2020-2025").unwrap(), YearSpec::Range(range));

        let mut set = HashSet::new();
        set.insert(2020);
        set.insert(2022);
        set.insert(2024);
        assert_eq!(YearSpec::parse("2020,2022,2024").unwrap(), YearSpec::List(set));
    }

    #[test]
    fn test_yearmonth_navigation() {
        let ym = YearMonth::new(2022, Month::December).unwrap();
        assert_eq!(ym.next_month().unwrap(), YearMonth::new(2023, Month::January).unwrap());


        let ym = YearMonth::new(2023, Month::January).unwrap();
        assert_eq!(ym.previous_month().unwrap(), YearMonth::new(2022, Month::December).unwrap());
    }

    #[test]
    fn test_eq_single_single() {
        assert_eq!(YearSpec::Single(2023), YearSpec::Single(2023));
        assert_ne!(YearSpec::Single(2023), YearSpec::Single(2022));
    }

    #[test]
    fn eq_single_range() {
        assert_eq!(YearSpec::Single(2023), YearSpec::Range(2023..=2023));
        assert_ne!(YearSpec::Single(2023), YearSpec::Range(2022..=2023));
        assert_ne!(YearSpec::Single(2023), YearSpec::Range(2023..=2024));
    }

    #[test]
    fn eq_single_list() {
        let mut set: HashSet<u16> = HashSet::new();
        set.insert(2023);
        assert_eq!(YearSpec::Single(2023), YearSpec::List(set.clone()));
        
        set.insert(2024);
        assert_ne!(YearSpec::Single(2023), YearSpec::List(set));
    }

    #[test]
    fn eq_range_range() {
        assert_eq!(YearSpec::Range(2023..=2023), YearSpec::Range(2023..=2023));
        assert_ne!(YearSpec::Range(2022..=2023), YearSpec::Range(2023..=2024));
    }

    #[test]
    fn eq_range_list() {
        let mut set = HashSet::new();
        set.insert(2023);
        assert_eq!(YearSpec::Range(2023..=2023), YearSpec::List(set.clone()));

        set.insert(2024);
        assert_eq!(YearSpec::Range(2023..=2024), YearSpec::List(set.clone()));
        
        set.insert(2025);
        assert_ne!(YearSpec::Range(2023..=2024), YearSpec::List(set));
    }

    #[test]
    fn eq_list_list() {
        let mut set1 = HashSet::new();
        set1.insert(2023);
        
        let mut set2 = HashSet::new();
        set2.insert(2023);

        assert_eq!(YearSpec::List(set1.clone()), YearSpec::List(set2.clone()));

        set2.insert(2024);
        assert_ne!(YearSpec::List(set1.clone()), YearSpec::List(set2.clone()));
    }

    /* 
    #[test]
    fn test_invalid_month_from_str() {
        assert!(Month::from_str("Januar").is_err());
        assert!(Month::from_str("13").is_err());
        assert!(Month::from_str("0").is_err());
    }

    #[test]
    fn test_no_overlap() {
        let ds1 = DateSpan::Year(YearSpec::Single(2020));
        let ds2 = DateSpan::Year(YearSpec::Single(2021));
        assert_eq!(ds1.intersect(&ds2), None);
    }
    
    #[test]
    fn test_partial_overlap() {
        let ym_set1: HashSet<(YearSpec, MonthSpec)> = vec![
            (YearSpec::Single(2020), MonthSpec::Single(Month::January)),
            (YearSpec::Single(2020), MonthSpec::Single(Month::February))
        ].into_iter().collect();
        let ym_set2: HashSet<(YearSpec, MonthSpec)> = vec![
            (YearSpec::Single(2020), MonthSpec::Single(Month::February)),
            (YearSpec::Single(2020), MonthSpec::Single(Month::March))
        ].into_iter().collect();
        
        let ds1: DateSpan = DateSpan::YearMonth(YearMonthSpec(ym_set1));
        let ds2: DateSpan = DateSpan::YearMonth(YearMonthSpec(ym_set2));

        let expected_ym_set: HashSet<(YearSpec, MonthSpec)> = vec![
            (YearSpec::Single(2020), MonthSpec::Single(Month::February))
        ].into_iter().collect();
        
        assert_eq!(ds1.intersect(&ds2), Some(DateSpan::YearMonth(YearMonthSpec(expected_ym_set))));
    }
    
    #[test]
    fn test_exact_overlap() {
        let ds1: DateSpan = DateSpan::Year(YearSpec::Single(2020));
        let ds2: DateSpan = DateSpan::Year(YearSpec::Single(2020));
        assert_eq!(ds1.intersect(&ds2), Some(ds1.clone()));
    }

    #[test]
    fn test_full_containment() {
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

        assert_eq!(ds1.intersect(&ds2), Some(ds2.clone()));
    }
    */

    #[test]
    fn test_monthspec_single_single() {
        assert_eq!(MonthSpec::Single(Month::January), MonthSpec::Single(Month::January));
        assert_ne!(MonthSpec::Single(Month::January), MonthSpec::Single(Month::February));
    }

    #[test]
    fn test_monthspec_single_range() {
        assert_eq!(MonthSpec::Single(Month::January), MonthSpec::Range(Month::January..=Month::January));
        assert_ne!(MonthSpec::Single(Month::January), MonthSpec::Range(Month::January..=Month::February));
    }

    #[test]
    fn test_monthspec_single_list() {
        let mut set = HashSet::new();
        set.insert(Month::January);
        assert_eq!(MonthSpec::Single(Month::January), MonthSpec::List(set.clone()));

        set.insert(Month::February);
        assert_ne!(MonthSpec::Single(Month::January), MonthSpec::List(set));
    }

    #[test]
    fn test_monthspec_range_range() {
        assert_eq!(MonthSpec::Range(Month::January..=Month::February), MonthSpec::Range(Month::January..=Month::February));
        assert_ne!(MonthSpec::Range(Month::January..=Month::February), MonthSpec::Range(Month::February..=Month::March));
    }

    #[test]
    fn test_monthspec_range_list() {
        let mut set = HashSet::new();
        set.insert(Month::January);
        set.insert(Month::February);
        assert_eq!(MonthSpec::Range(Month::January..=Month::February), MonthSpec::List(set));
    }

    #[test]
    fn test_yearmonthspec_equality() {
        let mut set1 = HashSet::new();
        set1.insert((YearSpec::Single(2023), MonthSpec::Single(Month::January)));

        let mut set2 = HashSet::new();
        set2.insert((YearSpec::Single(2023), MonthSpec::Single(Month::January)));

        assert_eq!(YearMonthSpec(set1.clone()), YearMonthSpec(set2.clone()));

        set2.insert((YearSpec::Single(2024), MonthSpec::Single(Month::February)));
        assert_ne!(YearMonthSpec(set1.clone()), YearMonthSpec(set2.clone()));
    }


    #[test]
    fn test_yearspec_next_prev() {
        assert_eq!(
            YearSpec::Single(2023).next(),
            YearSpec::Single(2024)
        );
        assert_eq!(
            YearSpec::Single(2023).prev(),
            YearSpec::Single(2022)
        );
        assert_eq!(
            YearSpec::Range(2020..=2023).next(),
            YearSpec::Range(2021..=2024)
        );
        assert_eq!(
            YearSpec::Range(2020..=2023).prev(),
            YearSpec::Range(2019..=2022)
        );
        let mut set = HashSet::new();
        set.insert(2021);
        set.insert(2022);
        let mut expected_set = HashSet::new();
        expected_set.insert(2022);
        expected_set.insert(2023);
        assert_eq!(
            YearSpec::List(set).next(),
            YearSpec::List(expected_set)
        );
    }


    #[test]
    fn test_monthspec_next_prev() {
        assert_eq!(
            MonthSpec::Single(Month::June).next(),
            MonthSpec::Single(Month::July)
        );
        assert_eq!(
            MonthSpec::Single(Month::June).prev(),
            MonthSpec::Single(Month::May)
        );
        assert_eq!(
            MonthSpec::Range(Month::April..=Month::June).next(),
            MonthSpec::Range(Month::May..=Month::July)
        );
        assert_eq!(
            MonthSpec::Range(Month::April..=Month::June).prev(),
            MonthSpec::Range(Month::March..=Month::May)
        );
        let mut set = HashSet::new();
        set.insert(Month::April);
        set.insert(Month::May);
        let mut expected_set = HashSet::new();
        expected_set.insert(Month::May);
        expected_set.insert(Month::June);
        assert_eq!(
            MonthSpec::List(set).next(),
            MonthSpec::List(expected_set)
        );
    }

    #[test]
    fn test_yearspec_forward_backward() {
        assert_eq!(
            YearSpec::Single(2023).forward(2),
            YearSpec::Single(2025)
        );
        assert_eq!(
            YearSpec::Single(2023).backward(1),
            YearSpec::Single(2022)
        );
        assert_eq!(
            YearSpec::Range(2020..=2023).forward(1),
            YearSpec::Range(2021..=2024)
        );
        assert_eq!(
            YearSpec::Range(2020..=2023).backward(2),
            YearSpec::Range(2018..=2021)
        );
    }

    #[test]
    fn test_yearspec_range_to() {
        assert_eq!(
            YearSpec::Single(2020).range_to(&YearSpec::Single(2023)),
            YearSpec::Range(2020..=2023)
        );
        assert_eq!(
            YearSpec::Single(2020).range_to(&YearSpec::Range(2021..=2023)),
            YearSpec::Range(2020..=2023)
        );
        let mut end_set = HashSet::new();
        end_set.insert(2021);
        end_set.insert(2022);
        let mut expected_set = HashSet::new();
        for year in 2020..=2022 {
            expected_set.insert(year);
        }
        assert_eq!(
            YearSpec::Single(2020).range_to(&YearSpec::List(end_set)),
            YearSpec::List(expected_set)
        );
    }

    // Pruebas adicionales para MonthSpec
    #[test]
    fn test_monthspec_forward_backward() {
        assert_eq!(
            MonthSpec::Single(Month::June).forward(2),
            MonthSpec::Single(Month::August)
        );
        assert_eq!(
            MonthSpec::Single(Month::June).backward(1),
            MonthSpec::Single(Month::May)
        );
        assert_eq!(
            MonthSpec::Range(Month::April..=Month::June).forward(1),
            MonthSpec::Range(Month::May..=Month::July)
        );
        assert_eq!(
            MonthSpec::Range(Month::April..=Month::June).backward(2),
            MonthSpec::Range(Month::February..=Month::April)
        );
    }

    #[test]
    fn test_monthspec_range_to() {
        assert_eq!(
            MonthSpec::Single(Month::January).range_to(&MonthSpec::Single(Month::March)),
            MonthSpec::Range(Month::January..=Month::March)
        );
        assert_eq!(
            MonthSpec::Single(Month::January).range_to(&MonthSpec::Range(Month::February..=Month::April)),
            MonthSpec::Range(Month::January..=Month::April)
        );
        let mut end_set = HashSet::new();
        end_set.insert(Month::February);
        end_set.insert(Month::March);
        let mut expected_set = HashSet::new();
        expected_set.insert(Month::January);
        expected_set.insert(Month::February);
        expected_set.insert(Month::March);
        assert_eq!(
            MonthSpec::Single(Month::January).range_to(&MonthSpec::List(end_set)),
            MonthSpec::List(expected_set)
        );
    }














}

    



}

