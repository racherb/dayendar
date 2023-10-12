pub mod calendar {
    use std::collections::HashMap;
    use time::Month as TimeMonth;
    use time::{Date, Duration, Weekday};
    use time::macros::date;

    pub const MIN_DATE: Date = date!(0001 - 01 - 01);
    pub const MAX_DATE: Date = date!(9999 - 12 - 31);

    pub type Year = u16;
    pub type Day = u8;
    
    //pub type Month = u8;
    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[repr(u8)]
    pub enum Month {
        January,
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
        pub fn to_index(&self) -> u8 {
            *self as u8 + 1
        }

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

        pub fn to_time_month(&self) -> TimeMonth {
            TimeMonth::try_from(self.to_index()).expect("Invalid month index")
        }
    }
    
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum BiDay {
        Zero,
        One,
    }

    #[derive(Debug, Clone, Eq)]
    pub struct DaysCalendar<T> {
        pub days_calendar: Vec<(Year, Month, Vec<T>)>,
    }

    impl BiDay {
        pub fn to_u8(&self) -> u8 {
            match self {
                BiDay::Zero => 0,
                BiDay::One => 1,
            }
        }
    
        pub fn from_u8(value: u8) -> Option<BiDay> {
            match value {
                0 => Some(BiDay::Zero),
                1 => Some(BiDay::One),
                _ => None,
            }
        }
    }

    // Determines whether a year is a leap year
    pub fn is_leap(year: Year) -> bool {
        (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
    }

    // Gets the number of days in a given month
    pub fn days_in_month(year: Year, month: Month) -> Option<u8> {
        if month < Month::January || month > Month::December {
            return None;
        }
    
        // Validate year range
        if year < 0001 || year > 9999 {
            return None;
        }
    
        let v_days_in_month: [u8; 12] = [
            31,
            if is_leap(year) { 29 } else { 28 },
            31,
            30,
            31,
            30,
            31,
            31,
            30,
            31,
            30,
            31,
        ];
        let n_days: u8 = v_days_in_month[month.to_index() as usize -1];
        Some(n_days)
    }

    // Creates a vector of given size filled with a BiDay value
    pub fn generate_vec_days(size: usize, value: BiDay) -> Vec<BiDay> {
        let mut vec = Vec::with_capacity(size);
        vec.resize(size, value);
        vec
    }

    impl DaysCalendar<BiDay> {
        // Create a DaysCalendar of a single year month
        pub fn singleton(year: Year, month: Month) -> Option<DaysCalendar<BiDay>> {
            if let Some(n_days) = days_in_month(year, month) {
                let mut days_calendar: Vec<(Year, Month, Vec<BiDay>)> = Vec::new();
                let vec: Vec<BiDay> = generate_vec_days(n_days as usize, BiDay::One);
                days_calendar.push((year, month, vec));
                Some(DaysCalendar { days_calendar })
            } else {
                None
            }
        }
        // Creates a new DaysCalendar from raw data
        pub fn new(data: Vec<(Year, Month, Vec<BiDay>)>) -> Self {
            Self { days_calendar: data }
        }
        // Create an empty DaysCalendar
        pub fn empty() -> DaysCalendar<BiDay> {
            DaysCalendar {
                days_calendar: Vec::new(),
            }
        }

        /*pub fn zeros() -> DaysCalendar<BiDay> {

        } */

        pub fn append(&self, other: &Self) -> DaysCalendar<BiDay> {
            let mut combined_days = Vec::with_capacity(self.days_calendar.len() + other.days_calendar.len());
            combined_days.extend(self.days_calendar.iter().cloned());
            combined_days.extend(other.days_calendar.iter().cloned());
            DaysCalendar { days_calendar: combined_days }
        }

        pub fn or(&self, other: &Self) -> DaysCalendar<BiDay> {
            let combined_days = self.append(&other);
            resume(&combined_days, |a, b| or_biday_operation(a, b))
        }

        pub fn and(&self, other: &Self) -> DaysCalendar<BiDay> {
            let combined_days = self.append(&other);
            resume(&combined_days, |a, b| and_biday_operation(a, b))
        }

        pub fn r#match(&self, other: &Self) -> DaysCalendar<BiDay> {
            let combined_days = self.append(&other);
            resume(&combined_days, |a, b| match_biday_operation(a, b))
        }

        pub fn nomatch(&self, other: &Self) -> DaysCalendar<BiDay> {
            let combined_days = self.append(&other);
            resume(&combined_days, |a, b| nomatch_biday_operation(a, b))
        }
        
        // Gets the days of a given month as BiDay
        fn get_days(&self, year: Year, month: Month) -> Option<&Vec<BiDay>> {
            self.days_calendar.iter().find_map(|(y, m, days)| {
                if *y == year && *m == month {
                    Some(days)
                } else {
                    None
                }
            })
        }
        
        // Finds the next day on the calendar after a given date
        pub fn next_day(&self, year: Year, month: Month, day: Day) -> Option<Date> {
            let reference_date = Date::from_calendar_date(year.into(), month.to_time_month(), day).ok()?;
    
            let dates = to_date(self.clone());
    
            let mut next_date = None;
            for date in dates.into_iter() {
                if date > reference_date {
                    next_date = Some(date);
                    break;
                }
            }
    
            next_date
        }

        // Finds the previous day in the calendar before a given date 
        pub fn previous_day(&self, year: Year, month: Month, day: Day) -> Option<Date> {
            let reference_date = Date::from_calendar_date(year.into(), month.to_time_month(), day).ok()?;
    
            let dates = to_date(self.clone());
    
            let mut previous_date = None;
            for date in dates.into_iter().rev() {
                if date < reference_date {
                    previous_date = Some(date);
                    break;
                }
            }
    
            previous_date
        }

        // Searches for the n-th day after/before a given date
        pub fn seek_nth_day(&self, year: Year, month: Month, day: Day, mut n: isize) -> Option<Date> {
            let reference_date = Date::from_calendar_date(year.into(), month.to_time_month(), day).ok()?;
    
            let dates = to_date(self.clone());
    
            let mut target_date = None;
            if n > 0 {
                for date in dates.into_iter() {
                    if date > reference_date {
                        n -= 1;
                        if n == 0 {
                            target_date = Some(date);
                            break;
                        }
                    }
                }
            } else if n < 0 {
                for date in dates.into_iter().rev() {
                    if date < reference_date {
                        n += 1;
                        if n == 0 {
                            target_date = Some(date);
                            break;
                        }
                    }
                }
            } else {
                target_date = Some(reference_date);
            }
    
            target_date
        }

        // Filter a calendar by keeping only the specified days of the week
        pub fn and_weekdays(&self, weekdays: HashSet<Weekday>) -> DaysCalendar<BiDay> {
            let mut filtered_days_calendar = vec![];
    
            for (year, month, days) in &self.days_calendar {
                let mut filtered_days = vec![];
                for (day_index, bit) in days.iter().enumerate() {
                    let day_num = day_index as u8 + 1;
                    let date = Date::from_calendar_date((*year).into(), month.to_time_month(), day_num).unwrap();
                    let weekday = date.weekday();
    
                    if *bit == BiDay::One && weekdays.contains(&weekday) {
                        filtered_days.push(BiDay::One);
                    } else {
                        filtered_days.push(BiDay::Zero);
                    }
                }
    
                filtered_days_calendar.push((*year, *month, filtered_days));
            }
    
            DaysCalendar {
                days_calendar: filtered_days_calendar,
            }
        }

        // Add specified days of the week to a calendar
        pub fn or_weekdays(&self, weekdays: HashSet<Weekday>) -> DaysCalendar<BiDay> {
            let mut filtered_days_calendar = vec![];
    
            for (year, month, days) in &self.days_calendar {
                let mut filtered_days = vec![];
                for (day_index, bit) in days.iter().enumerate() {
                    let day_num = day_index as u8 + 1;
                    let date = Date::from_calendar_date((*year).into(), month.to_time_month(), day_num).unwrap();
                    let weekday = date.weekday();
    
                    if *bit == BiDay::One || (*bit == BiDay::Zero && weekdays.contains(&weekday)) {
                        filtered_days.push(BiDay::One);
                    } else {
                        filtered_days.push(BiDay::Zero);
                    }
                }
    
                filtered_days_calendar.push((*year, *month, filtered_days));
            }
    
            DaysCalendar {
                days_calendar: filtered_days_calendar,
            }
        }

        // Filters a calendar by keeping only the specified ISO weeks
        pub fn and_iso_weeks(&self, weeks: Vec<u32>) -> DaysCalendar<BiDay> {
            let mut new_calendar = self.clone();
    
            for (year, month, days) in new_calendar.days_calendar.iter_mut() {
                let first_day_of_month = Date::from_calendar_date(*year as i32, month.to_time_month(), 1).unwrap();
                let days_in_month = days.len();
    
                for day_index in 0..days_in_month {
                    let date = first_day_of_month + Duration::days(day_index as i64);
                    let week_number = date.iso_week() as u32;
    
                    if days[day_index] == BiDay::One && !weeks.contains(&week_number) {
                        days[day_index] = BiDay::Zero;
                    }
                }
            }
    
            new_calendar
        }

        // Adds specified ISO weeks to a calendar
        pub fn or_iso_weeks(&self, weeks: Vec<u32>) -> DaysCalendar<BiDay> {
            let mut new_calendar = self.clone();
    
            for (year, month, days) in new_calendar.days_calendar.iter_mut() {
                let first_day_of_month = Date::from_calendar_date(*year as i32, month.to_time_month(), 1).unwrap();
                let days_in_month = days.len();
    
                for day_index in 0..days_in_month {
                    let date = first_day_of_month + Duration::days(day_index as i64);
                    let week_number = date.iso_week() as u32;
    
                    if days[day_index] == BiDay::One || weeks.contains(&week_number) {
                        days[day_index] = BiDay::One;
                    }
                }
            }
    
            new_calendar
        }

        pub fn not_iso_weeks(&self, weeks: Vec<u32>) -> DaysCalendar<BiDay> {
            let mut new_calendar = self.clone();
    
            for (year, month, days) in new_calendar.days_calendar.iter_mut() {
                let first_day_of_month = Date::from_calendar_date(*year as i32, month.to_time_month(), 1).unwrap();
                let days_in_month = days.len();
    
                for day_index in 0..days_in_month {
                    let date = first_day_of_month + Duration::days(day_index as i64);
                    let week_number = date.iso_week() as u32;
    
                    if days[day_index] == BiDay::One && !weeks.contains(&week_number) {
                        days[day_index] = BiDay::One;
                    } else {
                        days[day_index] = BiDay::Zero;
                    }
                }
            }
    
            new_calendar
        }

        pub fn not_weekdays(&self, weekdays: HashSet<Weekday>) -> DaysCalendar<BiDay> {
            let mut filtered_days_calendar = vec![];
        
            for (year, month, days) in &self.days_calendar {
                let mut filtered_days = vec![];
                for (day_index, bit) in days.iter().enumerate() {
                    let day_num = day_index as u8 + 1;
                    let date = Date::from_calendar_date((*year).into(), month.to_time_month(), day_num).unwrap();
                    let weekday = date.weekday();
        
                    if *bit == BiDay::One && !weekdays.contains(&weekday) {
                        filtered_days.push(BiDay::One);
                    } else {
                        filtered_days.push(BiDay::Zero);
                    }
                }
        
                filtered_days_calendar.push((*year, *month, filtered_days));
            }
        
            DaysCalendar {
                days_calendar: filtered_days_calendar,
            }
        }

    }

    // Replicates a pattern of days in a calendar
    pub fn replicate<T>(pattern: &[BiDay], calendar: DaysCalendar<BiDay>) -> DaysCalendar<BiDay>
    where T: Clone
    {
        let new_calendar = calendar.days_calendar
            .iter()
            .map(|(year, month, days)| {
                let replicated_pattern = replicate_pattern(pattern, days.len());
                let new_days = days
                    .iter()
                    .enumerate()
                    .map(|(i, _)| replicated_pattern[i].clone())
                    .collect::<Vec<BiDay>>();

                (*year, *month, new_days)
            })
            .collect::<Vec<(Year, Month, Vec<BiDay>)>>();

        DaysCalendar {
            days_calendar: new_calendar,
        }
    }

    impl DaysCalendar<BiDay> {
        pub fn zeros(self) -> DaysCalendar<BiDay> {
            let pattern: [BiDay; 1] = [BiDay::Zero];
            replicate::<BiDay>(&pattern, self)
        }

        pub fn ones(self) -> DaysCalendar<BiDay> {
            let pattern: [BiDay; 1] = [BiDay::One];
            replicate::<BiDay>(&pattern, self)
        }

        pub fn upward_step(self) -> DaysCalendar<BiDay> {
            let pattern: [BiDay; 2] = [BiDay::Zero, BiDay::One];
            replicate::<BiDay>(&pattern, self)
        }

        pub fn downward_step(self) -> DaysCalendar<BiDay> {
            let pattern: [BiDay; 2] = [BiDay::One, BiDay::Zero];
            replicate::<BiDay>(&pattern, self)
        }

        pub fn invert_biday(&self) -> DaysCalendar<BiDay> {
            let mut inverted_calendar = Vec::new();
    
            for (year, month, days) in &self.days_calendar {
                let inverted_days = days
                    .iter()
                    .map(|day| match day {
                        BiDay::Zero => BiDay::One,
                        BiDay::One => BiDay::Zero,
                    })
                    .collect::<Vec<BiDay>>();
                inverted_calendar.push((*year, *month, inverted_days));
            }
    
            DaysCalendar {
                days_calendar: inverted_calendar,
            }
        }

        pub fn take_n_biday(&self, n: usize) -> DaysCalendar<BiDay> {
            let mut result = DaysCalendar {
                days_calendar: Vec::new(),
            };
    
            let mut remaining = n;
            let mut days_iter = self.days_calendar.iter();
    
            while let Some((year, month, days)) = days_iter.next() {
                if remaining == 0 {
                    break;
                }
    
                let to_take = remaining.min(days.len());
                let new_days = days.iter().take(to_take).cloned().collect();
                result.days_calendar.push((*year, *month, new_days));
    
                remaining -= to_take;
            }
    
            result
        }
    }

    impl<T: PartialEq> PartialEq for DaysCalendar<T> {
        fn eq(&self, other: &Self) -> bool {
            self.days_calendar == other.days_calendar
        }
    }

    // Consult a calendar and consolidate days by year and month
    pub fn query_year_consolidate<T>(y: Year, dc: &DaysCalendar<T>) -> HashMap<(Year, Month), Vec<T>>
    where
        T: Copy,
    {
        dc.days_calendar
            .iter()
            .filter_map(|(year, month, days)| {
                if *year == y {
                    Some(((*year, *month), days.to_vec()))
                } else {
                    None
                }
            })
            .collect()
    }

    use std::collections::HashSet;

    pub fn extract_years_calendar<T>(dc: &DaysCalendar<T>) -> Vec<Year>
    where
        T: Clone,
    {
        let mut years: HashSet<Year> = HashSet::new();

        for (year, _, _) in dc.days_calendar.iter().cloned() {
            years.insert(year);
        }

        let mut year_vec: Vec<Year> = years.into_iter().collect();
        year_vec.sort();
        year_vec
    }

    // Extracts the years and months present in a calendar
    pub fn extract_year_month_calendar<T: Clone>(dc: &DaysCalendar<T>) -> Vec<(Year, Month)>
    where 
        T: Clone,
    {
        let mut result = Vec::new();
        for (y, m, _) in &dc.days_calendar {
            result.push((*y as Year, *m as Month));
        }
        result.sort_unstable();
        result.dedup();
        result
    }

    // Extracts the days of a specific month in a calendar
    pub fn extract_day_month_calendar<T: Clone>(
        year: Year,
        month: Month,
        dc: &DaysCalendar<T>,
    ) -> Option<Vec<T>> {
        // Finds the index of the element matching the given year and month
        let index = dc.days_calendar.binary_search_by_key(&(year, month), |(y, m, _)| (*y, *m));
        // If the element is found, the corresponding vector of days is returned.
        if let Ok(index) = index {
            Some(dc.days_calendar[index].2.clone())
        } else {
            None
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
    pub enum DaysCalendarOperator {
        DcOr,
        DcAnd,
        DcAdd,
        DcSustract,
        DcMatchAny,
    }

    pub fn apply_operator<F>(a: &[BiDay], b: &[BiDay], operator: F) -> Vec<BiDay>
    where F: Fn(BiDay, BiDay) -> BiDay
    {
        let mut result = Vec::with_capacity(a.len());
        result.extend(a.iter().zip(b.iter()).map(|(&a, &b)| operator(a, b)));
        result
    }

    pub fn or_biday_operation(a: BiDay, b: BiDay) -> BiDay {
        match (a, b) {
            (BiDay::One, _) | (_, BiDay::One) => BiDay::One,
            (BiDay::Zero, BiDay::Zero)        => BiDay::Zero,
            _                                 => BiDay::Zero,
        }
    }

    pub fn or_daycalendar(v1: &[BiDay], v2: &[BiDay]) -> Vec<BiDay> {
        apply_operator(v1, v2, or_biday_operation)
    }

    pub fn and_biday_operation(a: BiDay, b: BiDay) -> BiDay {
        match (a, b) {
            (BiDay::One, BiDay::One)    => BiDay::One,
            _                           => BiDay::Zero,
        }
    }

    pub fn and_daycalendar(v1: &[BiDay], v2: &[BiDay]) -> Vec<BiDay> {
        apply_operator(v1, v2, and_biday_operation)
    }

    fn minus_biday_operation(a: BiDay, b: BiDay) -> BiDay {
        match (a, b) {
            (BiDay::One, BiDay::One)    => BiDay::Zero,
            (BiDay::One, BiDay::Zero)   => BiDay::One,
            (BiDay::Zero, _)            => BiDay::Zero,
            _                           => BiDay::Zero,
        }
    }

    pub fn minus_daycalendar(v1: &[BiDay], v2: &[BiDay]) -> Vec<BiDay> {
        apply_operator(v1, v2, minus_biday_operation)
    }

    fn add_biday_operation(a: BiDay, b: BiDay) -> BiDay {
        match (a, b) {
            (BiDay::One, BiDay::One)    => BiDay::One,
            (BiDay::One, BiDay::Zero)   => BiDay::One,
            (BiDay::Zero, BiDay::One)   => BiDay::One,
            _                           => BiDay::Zero,
        }
    }

    pub fn add_daycalendar(v1: &[BiDay], v2: &[BiDay]) -> Vec<BiDay> {
        apply_operator(v1, v2, add_biday_operation)
    }

    fn match_biday_operation(a: BiDay, b: BiDay) -> BiDay {
        match (a, b) {
            (BiDay::One, BiDay::One)    => BiDay::One,
            (BiDay::Zero, BiDay::Zero)  => BiDay::One,
            _                           => BiDay::Zero,
        }
    }

    pub fn match_daycalendar(v1: &[BiDay], v2: &[BiDay]) -> Vec<BiDay> {
        apply_operator(v1, v2, match_biday_operation)
    }

    fn nomatch_biday_operation(a: BiDay, b: BiDay) -> BiDay {
        match (a, b) {
            (BiDay::One, BiDay::One)    => BiDay::Zero,
            (BiDay::Zero, BiDay::Zero)  => BiDay::Zero,
            _                           => BiDay::One,
        }
    }

    pub fn nomatch_daycalendar(v1: &[BiDay], v2: &[BiDay]) -> Vec<BiDay> {
        apply_operator(v1, v2, nomatch_biday_operation)
    }

    fn holes_biday_operation(a: BiDay, b: BiDay) -> BiDay {
        match (a, b) {
            (BiDay::Zero, BiDay::Zero)    => BiDay::One,
            _                             => BiDay::Zero,
        }
    }

    pub fn holes_daycalendar(v1: &[BiDay], v2: &[BiDay]) -> Vec<BiDay> {
        apply_operator(v1, v2, holes_biday_operation)
    }

    use itertools::Itertools;

    pub fn group_days_calendar<T: Clone>(calendar: DaysCalendar<T>) -> Vec<(Year, Month, Vec<Vec<T>>)> {
        // Group days by year and month
        let groups = calendar.days_calendar.iter().group_by(|(year, month, _)| (*year, *month));

        // Iterate on the groups and generate the new structure
        let mut result = Vec::new();
        for ((year, month), group) in groups.into_iter() {
            let days: Vec<Vec<T>> = group.map(|(_, _, days)| days.clone()).collect();
            result.push((year, month, days));
        }

        // Sort by year and month
        result.sort_by(|(year1, month1, _), (year2, month2, _)| {
            year1.cmp(year2).then_with(|| month1.cmp(month2))
        });

        result
    }


    use std::vec::Vec;

    /* "fully_" Complete a BiDay vector by incorporating the BiDay value until the n elements after the last value are completed
        -- 'n' is the number of elements
        -- 'k' is the value of the new BiDay elements
        -- 'v' is the vector of BiDay
    */
    pub fn complete_biday(n: usize, k: BiDay, mut v: Vec<BiDay>) -> Vec<BiDay> {
        if n >= v.len() {
            let ni = n - v.len();
            if ni == 0 {
                return v;
            } else {
                let mut new_v = Vec::with_capacity(n);
                new_v.extend(v);
                new_v.resize(n, k);
                return new_v;
            }
        } else {
            v.truncate(n);
            return v;
        }
    }

    pub fn normalize_biday(days: &[BiDay], year: Year, month: Month) -> Vec<BiDay> {
        let n = days_in_month(year, month).unwrap().into();
        if days.len() >= n {
            days[..n].to_vec()
        } else {
            complete_biday(n, BiDay::Zero, days.to_vec())
        }
    }

    pub fn resume<F>(calendar: &DaysCalendar<BiDay>, op: F) -> DaysCalendar<BiDay>
    where
        F: Fn(BiDay, BiDay) -> BiDay,
    {
        let mut days_calendar = calendar.days_calendar.clone();
        days_calendar.sort_by_key(|&(year, month, _)| (year, month));

        let mut res = Vec::new();
        let mut curr_year: Year = 0;
        let mut curr_month: Option<Month> = None;
        let mut curr_vec = Vec::new();
        let mut curr_val = &curr_vec;

        for &(year, month, ref vec) in &days_calendar {
            if year != curr_year || month != curr_month.unwrap() {
                if !curr_vec.is_empty() {
                    res.push((curr_year, curr_month.unwrap(), curr_val.to_vec()));
                }
                curr_year = year;
                curr_month = Some(month);
                curr_vec = normalize_biday(vec, curr_year, curr_month.unwrap()).clone();
                curr_val = &curr_vec;
            } else {
                let curr_val_norm = &normalize_biday(curr_val, curr_year, curr_month.unwrap());
                let new_val = curr_val_norm.iter().zip(normalize_biday(vec, curr_year, curr_month.unwrap()).iter()).map(|(a, b)| op(*a, *b)).collect::<Vec<BiDay>>();
                curr_vec = new_val;
                curr_val = &curr_vec;
            }
        }

        if !curr_vec.is_empty() {
            res.push((curr_year, curr_month.unwrap(), curr_val.to_vec()));
        }

        DaysCalendar { days_calendar: res }
    }

    pub fn replicate_pattern(pattern: &[BiDay], n: usize) -> Vec<BiDay> {
        let mut result = Vec::with_capacity(n);
        let pattern_len = pattern.len();
    
        for i in 0..n {
            result.push(pattern[i % pattern_len]);
        }
    
        result
    }
    
    pub fn biday_to_vec_day(calendar: DaysCalendar<BiDay>) -> Vec<(Year, Month, Vec<Day>)>
    {
        let mut result = Vec::new();
        let mut vec_days = Vec::new();
    
        for (year, month, days) in calendar.days_calendar.iter() {
            for (day, value) in days.iter().enumerate() {
                if *value == BiDay::One {
                    vec_days.push(day as u8 + 1);
                }
            }
            result.push((*year, *month as Month, vec_days.clone()));
            vec_days = Vec::new();
        }
    
        result
    }

    // Convert a DaysCalendar calendar from BiDay to Day days
    pub fn to_day<T>(calendar: DaysCalendar<BiDay>) -> DaysCalendar<Day> {
        let mut result = DaysCalendar {
            days_calendar: Vec::new(),
        };
    
        for (year, month, days) in calendar.days_calendar {
            let converted_days = days
                .iter()
                .enumerate()
                .map(|(day, &bit)| {
                    if bit != BiDay::Zero {
                        Some(day as u8 + 1 )
                    } else {
                        None
                    }
                })
                .filter_map(|x| x)
                .collect();
    
            result.days_calendar.push((year, month, converted_days));

        }
    
        result
    }

    // Convert a DaysCalendar DaysCalendar from Day days to BiDay days
    pub fn from_day(calendar: DaysCalendar<Day>) -> DaysCalendar<BiDay> {
        let mut result = DaysCalendar {
            days_calendar: Vec::new(),
        };
    
        for (year, month, days) in calendar.days_calendar {
            let n_days = days_in_month(year, month).unwrap_or(0);
            let mut bi_days = vec![BiDay::Zero; n_days as usize];
            for day in days {
                if let idx = day - 1 {
                    bi_days[idx as usize] = BiDay::One;
                }
            }
            result.days_calendar.push((year, month, bi_days));
        }
    
        result
    }

    impl DaysCalendar<Day> {
        pub fn new_days(data: Vec<(Year, Month, Vec<Day>)>) -> Self {
            Self {
                days_calendar: data,
            }
        }

        pub fn sort(&mut self) {
            // Sort days_calendar by Year and Month
            self.days_calendar.sort_unstable_by(|(y1, m1, _), (y2, m2, _)| {
                if y1 == y2 {
                    m1.cmp(m2)
                } else {
                    y1.cmp(y2)
                }
            });
    
            // Order the days of each entry
            for (_, _, days) in &mut self.days_calendar {
                days.sort_unstable();
            }
        }
    
        pub fn contains(&mut self, year: Year, month: Month, day: Day) -> bool {
            // Sort days_calendar before performing binary search
            self.sort();
    
            // Perform a binary search on days_calendar to find the given year and month
            let index = self.days_calendar.binary_search_by(|&(y, m, _)| {
                if y == year {
                    m.cmp(&month)
                } else {
                    y.cmp(&year)
                }
            });
    
            // If an item is found with the given year and month, check if it contains the day
            match index {
                Ok(i) => self.days_calendar[i].2.contains(&day),
                Err(_) => false,
            }
        }
    }

    use std::convert::TryInto;

    // Converts a DaysCalendar into a Date vector
    pub fn to_date(calendar: DaysCalendar<BiDay>) -> Vec<Date> {
        let total_days = calendar
            .days_calendar
            .iter()
            .map(|(_, _, days)| days.iter().filter(|&day| *day == BiDay::One).count())
            .sum();
        let mut dates = Vec::with_capacity(total_days);
    
        for (year, month, days) in calendar.days_calendar {
            let year_num: Year = year;
            let month_num: Month = month;
    
            for (day_index, day) in days.iter().enumerate() {
                if *day == BiDay::One {
                    let day_num = day_index as u8 + 1;
                    let date = Date::from_calendar_date(year_num.into(), month_num.to_time_month(), day_num).unwrap();
                    dates.push(date);
                }
            }
        }
    
        dates
    }

    use std::collections::BTreeMap;

    // CConverts a Date date vector to a DaysCalendar calendar
    pub fn from_date(dates: Vec<Date>) -> DaysCalendar<Day> {
        // Create an orderly map for storing years, months and days
        let mut year_month_map: BTreeMap<(Year, u8), Vec<Day>> = BTreeMap::new();
    
        // Iterate over the given dates and add them to the map
        for date in dates {
            let entry = year_month_map
                .entry((date.year().try_into().unwrap(), date.month().into()))
                .or_insert_with(Vec::new);
            let day = date.day() as Day;
            if !entry.contains(&day) {
                entry.push(day);
            }
        }
    
        // Sort and delete duplicate days in each month on the map
        for days in year_month_map.values_mut() {
            days.sort_unstable();
            days.dedup();
        }
    
        // Convert map to a vector of tuples (Year, Month, Vec<Day>)
        let days_calendar: Vec<(Year, Month, Vec<Day>)> = year_month_map
            .into_iter()
            .map(|((year, month_number), days)| (year, Month::from_index(month_number).unwrap(), days))
            .collect();
    
        // Return DaysCalendar<Day>
        DaysCalendar { days_calendar }
    }

    // Adds or subtracts days to a date vector
    pub fn add_days(dates: Vec<Date>, n: i32) -> Vec<Date> {
        let duration = Duration::days(n.abs() as i64);
        dates
            .into_iter()
            .map(|date| {
                if n >= 0 {
                    date.checked_add(duration).unwrap_or(MAX_DATE)
                } else {
                    date.checked_sub(duration).unwrap_or(MIN_DATE)
                }
            })
            .collect()
    }

    pub enum CalendarOperation {
        Union,
        Intersection,
        Difference,
    }

    pub enum RuleOperation {
        Add,
        Remove,
    }
    
    pub enum CalendarRule {
        None,
        Everyday,
        Workday(RuleOperation),
        Weekends(RuleOperation),
        FixedHolidays(RuleOperation),
        FloatingHolidays(RuleOperation),
        Holidays(RuleOperation),
        FirstDays(u8, RuleOperation),
        LastDays(u8, RuleOperation),
        FirstWeeks(u8, RuleOperation),
        LastWeeks(u8, RuleOperation),
        SpecificWeekdays(Vec<Weekday>),
        EveryNthDay(Vec<u8>, RuleOperation),
        DayOfMonth(Vec<u8>, RuleOperation),
        WeekOfMonth(Vec<u8>, RuleOperation),
        SpecificMonths(Vec<Month>, RuleOperation)

    }
    
    pub struct PeriodicCalendar {
        calendars: Vec<Calendar>,
    }

    pub struct Calendar {
        name: String,
        rules: Vec<CalendarRule>,
        dependent_on: Option<Box<Calendar>>,
        operation: Option<CalendarOperation>,
    }

    
  
}

