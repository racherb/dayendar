
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::BTreeMap;

/// Module for advanced and efficient calendar operations
pub mod calendar {

    
    use crate::utils::{days_in_month, generate_vec_days};
    use crate::binary::{
        or_biday_operation, and_biday_operation,
        match_biday_operation, nomatch_biday_operation,
        replicate_pattern, normalize_biday, minus_biday_operation

    };
    use crate::types::{
        Year, Month, Day, BiDay, Weekday,
        Date, Duration
    };

    use itertools::Itertools;

    /// The `DaysCalendar` base type to handle calendar related operations.
    /// provides a set of methods and functions for working
    /// with dates and perform efficient calendar-related operations.
    #[derive(Debug, Clone, Eq)]
    pub struct DaysCalendar<T> {
        pub days_calendar: Vec<(Year, Month, Vec<T>)>,
    }

    impl DaysCalendar<BiDay> {
        /// Create a DaysCalendar of a single year month
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
        /// Creates a new DaysCalendar from raw data
        pub fn new(data: Vec<(Year, Month, Vec<BiDay>)>) -> Self {
            Self { days_calendar: data }
        }
        /// Create an empty DaysCalendar
        pub fn empty() -> DaysCalendar<BiDay> {
            DaysCalendar {
                days_calendar: Vec::new(),
            }
        }

        /*pub fn zeros() -> DaysCalendar<BiDay> {

        } */

        /// Combines two DaysCalendar types into one by making an extension of the calendars
        pub fn append(&self, other: &Self) -> DaysCalendar<BiDay> {
            let mut combined_days: Vec<(u16, Month, Vec<BiDay>)> = Vec::with_capacity(self.days_calendar.len() + other.days_calendar.len());
            combined_days.extend(self.days_calendar.iter().cloned());
            combined_days.extend(other.days_calendar.iter().cloned());
            DaysCalendar { days_calendar: combined_days }
        }

        /// Combines two DaysCalendar types based on the OR operator
        pub fn or(&self, other: &Self) -> DaysCalendar<BiDay> {

            let combined_days: DaysCalendar<BiDay> = self.append(other);
            resume(&combined_days, or_biday_operation) 
        
        }

        /// Combines two DaysCalendar types based on the AND operator
        pub fn and(&self, other: &Self) -> DaysCalendar<BiDay> {
            let combined_days: DaysCalendar<BiDay> = self.append(other);
            resume(&combined_days, and_biday_operation)
        }

        /// Combines two DaysCalendar types based on the SUSTRACT operator
        pub fn minus(&self, other: &Self) -> DaysCalendar<BiDay> {
            let combined_days: DaysCalendar<BiDay> = self.append(other);
            resume(&combined_days, minus_biday_operation)
        }

        /// Combines two DaysCalendar types based on calendar matches
        pub fn r#match(&self, other: &Self) -> DaysCalendar<BiDay> {
            let combined_days: DaysCalendar<BiDay> = self.append(other);
            resume(&combined_days, match_biday_operation)
        }

        /// Combines two DaysCalendar types based on calendar mismatches
        pub fn nomatch(&self, other: &Self) -> DaysCalendar<BiDay> {
            let combined_days: DaysCalendar<BiDay> = self.append(other);
            resume(&combined_days, nomatch_biday_operation)
        }
        
        /// Gets the days of a given month as BiDay
        pub fn get_days(&self, year: Year, month: Month) -> Option<&Vec<BiDay>> {
            self.days_calendar.iter().find_map(|(y, m, days)| {
                if *y == year && *m == month {
                    Some(days)
                } else {
                    None
                }
            })
        }
        
        /// Finds the next day on the calendar after a given date
        pub fn next_day(&self, year: Year, month: Month, day: Day) -> Option<Date> {
            let month_time = month.to_time_month().ok()?;
            let reference_date: Date = Date::from_calendar_date(year.into(), month_time, day).ok()?;

            let dates: Vec<Date> = to_date(self.clone());

            let mut next_date: Option<Date> = None;
            for date in dates.into_iter() {
                if date > reference_date {
                    next_date = Some(date);
                    break;
                }
            }

            next_date
        }

        /// Finds the previous day in the calendar before a given date 
        pub fn previous_day(&self, year: Year, month: Month, day: Day) -> Option<Date> {
            let month_time = month.to_time_month().ok()?;
            let reference_date: Date = Date::from_calendar_date(year.into(), month_time, day).ok()?;

            let dates: Vec<Date> = to_date(self.clone());

            let mut previous_date = None;
            for date in dates.into_iter().rev() {
                if date < reference_date {
                    previous_date = Some(date);
                    break;
                }
            }

            previous_date
        }

        /// Searches for the n-th day after/before a given date
        pub fn seek_nth_day(&self, year: Year, month: Month, day: Day, mut n: isize) -> Result<Option<Date>, String> {
            let month_time = month.to_time_month().map_err(|e| e.to_string())?;
            let reference_date: Date = Date::from_calendar_date(year.into(), month_time, day)
                .map_err(|e| e.to_string())?;
        
            let dates: Vec<Date> = to_date(self.clone());
        
            let target_date = match n.cmp(&0) {
                std::cmp::Ordering::Greater => {
                    dates.into_iter()
                        .find(|&date| {
                            if date > reference_date {
                                n -= 1;
                                n == 0
                            } else {
                                false
                            }
                        })
                },
                std::cmp::Ordering::Less => {
                    dates.into_iter()
                        .rev()
                        .find(|&date| {
                            if date < reference_date {
                                n += 1;
                                n == 0
                            } else {
                                false
                            }
                        })
                },
                std::cmp::Ordering::Equal => Some(reference_date),
            };
        
            Ok(target_date)
        }

        /// Filter a calendar by keeping only the specified days of the week
        pub fn and_weekdays(&self, weekdays: super::HashSet<Weekday>) -> Result<DaysCalendar<BiDay>, &'static str> {
            let mut filtered_days_calendar: Vec<(u16, Month, Vec<BiDay>)> = vec![];
        
            for (year, month, days) in &self.days_calendar {
                let mut filtered_days = vec![];
        
                for (day_index, bit) in days.iter().enumerate() {
                    let day_num = day_index as u8 + 1;
                    let month_time = month.to_time_month()?;
                    let date = Date::from_calendar_date((*year).into(), month_time, day_num)
                        .map_err(|_| "Invalid date")?;
                    let weekday: Weekday = date.weekday();
        
                    if *bit == BiDay::One && weekdays.contains(&weekday) {
                        filtered_days.push(BiDay::One);
                    } else {
                        filtered_days.push(BiDay::Zero);
                    }
                }
        
                filtered_days_calendar.push((*year, *month, filtered_days));
            }
        
            Ok(DaysCalendar {
                days_calendar: filtered_days_calendar,
            })
        }
        

        /// Add specified days of the `week` to a `DaysCalendar`
        pub fn or_weekdays(&self, weekdays: super::HashSet<Weekday>) -> Result<DaysCalendar<BiDay>, &'static str> {
            let mut filtered_days_calendar: Vec<(u16, Month, Vec<BiDay>)> = vec![];
        
            for (year, month, days) in &self.days_calendar {
                let mut filtered_days = vec![];
                for (day_index, bit) in days.iter().enumerate() {
                    let day_num = day_index as u8 + 1;
                    let month_time = month.to_time_month()?;
                    let date: Date = Date::from_calendar_date((*year).into(), month_time, day_num)
                        .map_err(|_| "Invalid date")?;
                    let weekday: Weekday = date.weekday();
        
                    if *bit == BiDay::One || (*bit == BiDay::Zero && weekdays.contains(&weekday)) {
                        filtered_days.push(BiDay::One);
                    } else {
                        filtered_days.push(BiDay::Zero);
                    }
                }
        
                filtered_days_calendar.push((*year, *month, filtered_days));
            }
        
            Ok(DaysCalendar {
                days_calendar: filtered_days_calendar,
            })
        }
        

        /// Filters a calendar by keeping only the specified ISO weeks
        pub fn and_iso_weeks(&self, weeks: Vec<u32>) -> Result<DaysCalendar<BiDay>, &'static str> {
            let mut new_calendar: DaysCalendar<BiDay> = self.clone();
        
            for (year, month, days) in new_calendar.days_calendar.iter_mut() {
                let first_day_of_month = Date::from_calendar_date(*year as i32, month.to_time_month()?, 1)
                    .map_err(|_| "Invalid date")?;
        
                for (day_index, day) in days.iter_mut().enumerate() {
                    let date = first_day_of_month + Duration::days(day_index as i64);
                    let week_number = date.iso_week() as u32;
        
                    if *day == BiDay::One && !weeks.contains(&week_number) {
                        *day = BiDay::Zero;
                    }
                }
            }
        
            Ok(new_calendar)
        }
        

        /// Adds specified ISO `weeks` to a `DaysCalendar` type
        pub fn or_iso_weeks(&self, weeks: Vec<u32>) -> Result<DaysCalendar<BiDay>, &'static str> {
            let mut new_calendar: DaysCalendar<BiDay> = self.clone();
        
            for (year, month, days) in new_calendar.days_calendar.iter_mut() {
                let first_day_of_month = Date::from_calendar_date(*year as i32, month.to_time_month()?, 1)
                    .map_err(|_| "Invalid date")?;
        
                for (day_index, day) in days.iter_mut().enumerate() {
                    let date = first_day_of_month + Duration::days(day_index as i64);
                    let week_number = date.iso_week() as u32;
        
                    if *day == BiDay::One || weeks.contains(&week_number) {
                        *day = BiDay::One;
                    }
                }
            }
        
            Ok(new_calendar)
        }        
        
        /// Excludes specific ISO `weeks` of type `DaysCalendar`
        pub fn not_iso_weeks(&self, weeks: Vec<u32>) -> Result<DaysCalendar<BiDay>, &'static str> {
            let mut new_calendar: DaysCalendar<BiDay> = self.clone();
        
            for (year, month, days) in new_calendar.days_calendar.iter_mut() {
                let first_day_of_month = Date::from_calendar_date(*year as i32, month.to_time_month()?, 1)
                    .map_err(|_| "Invalid date")?;
        
                for (day_index, day) in days.iter_mut().enumerate() {
                    let date: Date = first_day_of_month + Duration::days(day_index as i64);
                    let week_number = date.iso_week() as u32;
        
                    if *day == BiDay::One && !weeks.contains(&week_number) {
                        *day = BiDay::One;
                    } else {
                        *day = BiDay::Zero;
                    }
                }
            }
        
            Ok(new_calendar)
        }        
        
        /// Excludes specific `weekdays` of type `DaysCalendar`
        pub fn not_weekdays(&self, weekdays: super::HashSet<Weekday>) -> Result<DaysCalendar<BiDay>, &'static str> {
            let mut filtered_days_calendar: Vec<(u16, Month, Vec<BiDay>)> = vec![];
        
            for (year, month, days) in &self.days_calendar {
                let mut filtered_days = vec![];
                for (day_index, bit) in days.iter().enumerate() {
                    let day_num = day_index as u8 + 1;
                    let month_time = month.to_time_month()?;
                    let date = Date::from_calendar_date((*year).into(), month_time, day_num)
                        .map_err(|_| "Invalid date")?;
                    let weekday = date.weekday();
        
                    if *bit == BiDay::One && !weekdays.contains(&weekday) {
                        filtered_days.push(BiDay::One);
                    } else {
                        filtered_days.push(BiDay::Zero);
                    }
                }
        
                filtered_days_calendar.push((*year, *month, filtered_days));
            }
        
            Ok(DaysCalendar {
                days_calendar: filtered_days_calendar,
            })
        }        

        /// Generates a `DaysCalendar` type with `Zero` values of `BiDay`
        pub fn zeros(self) -> DaysCalendar<BiDay> {
            let pattern: [BiDay; 1] = [BiDay::Zero];
            replicate::<BiDay>(&pattern, self)
        }

        /// Generates a `DaysCalendar` type with `Ones` values of `BiDay`
        pub fn ones(self) -> DaysCalendar<BiDay> {
            let pattern: [BiDay; 1] = [BiDay::One];
            replicate::<BiDay>(&pattern, self)
        }

        /// Generates a DaysCalendar type with values of a BiDay ascending step.
        pub fn upward_step(self) -> DaysCalendar<BiDay> {
            let pattern: [BiDay; 2] = [BiDay::Zero, BiDay::One];
            replicate::<BiDay>(&pattern, self)
        }

        /// Generates a DaysCalendar type with values of a descending BiDay step.
        pub fn downward_step(self) -> DaysCalendar<BiDay> {
            let pattern: [BiDay; 2] = [BiDay::One, BiDay::Zero];
            replicate::<BiDay>(&pattern, self)
        }

        /// Inverts the BiDay values of a DaysCalendar type
        pub fn invert_biday(&self) -> DaysCalendar<BiDay> {
            let mut inverted_calendar: Vec<(u16, Month, Vec<BiDay>)> = Vec::new();

            for (year, month, days) in &self.days_calendar {
                let inverted_days: Vec<BiDay> = days
                    .iter()
                    .map(|day: &BiDay| match day {
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

        /// Takes n BiDay values from a given DaysCalendar
        pub fn take_n_biday(&self, n: usize) -> DaysCalendar<BiDay> {
            let mut result: DaysCalendar<BiDay> = DaysCalendar {
                days_calendar: Vec::new(),
            };

            let mut remaining = n;
            let days_iter: std::slice::Iter<'_, (u16, Month, Vec<BiDay>)> = self.days_calendar.iter();

            for (year, month, days) in days_iter {
                if remaining == 0 {
                    break;
                }

                let to_take = remaining.min(days.len());
                let new_days: Vec<BiDay> = days.iter().take(to_take).cloned().collect();
                result.days_calendar.push((*year, *month, new_days));

                remaining -= to_take;
            }

            result
        }

    }

    /// Replicates a `pattern` of days in a `DaysCalendar`
    pub fn replicate<T>(pattern: &[BiDay], calendar: DaysCalendar<BiDay>) -> DaysCalendar<BiDay>
    where T: Clone
    {
        let new_calendar: Vec<(u16, Month, Vec<BiDay>)> = calendar.days_calendar
            .iter()
            .map(|(year, month, days)| {
                let replicated_pattern = replicate_pattern(pattern, days.len());
                let new_days = days
                    .iter()
                    .enumerate()
                    .map(|(i, _)| replicated_pattern[i])
                    .collect::<Vec<BiDay>>();

                (*year, *month, new_days)
            })
            .collect::<Vec<(Year, Month, Vec<BiDay>)>>();

        DaysCalendar {
            days_calendar: new_calendar,
        }
    }

    impl<T: PartialEq> PartialEq for DaysCalendar<T> {
        /// Determines if two DaysCalendar types are the same
        fn eq(&self, other: &Self) -> bool {
            self.days_calendar == other.days_calendar
        }
    }

    /// Consult a calendar and consolidate days by year and month
    #[allow(dead_code)]
    pub fn query_year_consolidate<T>(y: Year, dc: &DaysCalendar<T>) -> super::HashMap<(Year, Month), Vec<T>>
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

    /// Gets the list of distinct years that contain a DaysCalendar type
    #[allow(dead_code)]
    pub fn extract_years_calendar<T>(dc: &DaysCalendar<T>) -> Vec<Year>
    where
        T: Clone,
    {
        let mut years: super::HashSet<Year> = super::HashSet::new();

        for (year, _, _) in dc.days_calendar.iter().cloned() {
            years.insert(year);
        }

        let mut year_vec: Vec<Year> = years.into_iter().collect();
        year_vec.sort();
        year_vec
    }

    /// Extracts the years and months present in a calendar
    #[allow(dead_code)]
    pub fn extract_year_month_calendar<T: Clone>(dc: &DaysCalendar<T>) -> Vec<(Year, Month)>
    where 
        T: Clone,
    {
        let mut result: Vec<(u16, Month)> = Vec::new();
        for (y, m, _) in &dc.days_calendar {
            result.push((*y as Year, *m as Month));
        }
        result.sort_unstable();
        result.dedup();
        result
    }

    /// Extracts the days of a specific month in a calendar
    #[allow(dead_code)]
    pub fn extract_day_month_calendar<T: Clone>(
        year: Year,
        month: Month,
        dc: &DaysCalendar<T>,
    ) -> Option<Vec<T>> {
        // Finds the index of the element matching the given year and month
        let index: Result<usize, usize> = dc.days_calendar.binary_search_by_key(&(year, month), |(y, m, _)| (*y, *m));
        // If the element is found, the corresponding vector of days is returned.
        if let Ok(index) = index {
            Some(dc.days_calendar[index].2.clone())
        } else {
            None
        }
    }

    /// Summarises or consolidates two DaysCalendar types based on a given operator
    pub fn resume<F>(calendar: &DaysCalendar<BiDay>, op: F) -> DaysCalendar<BiDay>
    where
        F: Fn(BiDay, BiDay) -> BiDay,
    {
        let mut days_calendar: Vec<(u16, Month, Vec<BiDay>)> = calendar.days_calendar.clone();
        days_calendar.sort_by_key(|&(year, month, _)| (year, month));

        let mut res: Vec<(u16, Month, Vec<BiDay>)> = Vec::new();
        let mut curr_year: Year = 0;
        let mut curr_month: Option<Month> = None;
        let mut curr_vec: Vec<BiDay> = Vec::new();
        let mut curr_val: &Vec<BiDay> = &curr_vec;

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
                let curr_val_norm: &Vec<BiDay> = &normalize_biday(curr_val, curr_year, curr_month.unwrap());
                let new_val: Vec<BiDay> = curr_val_norm.iter().zip(normalize_biday(vec, curr_year, curr_month.unwrap()).iter()).map(|(a, b)| op(*a, *b)).collect::<Vec<BiDay>>();
                curr_vec = new_val;
                curr_val = &curr_vec;
            }
        }

        if !curr_vec.is_empty() {
            res.push((curr_year, curr_month.unwrap(), curr_val.to_vec()));
        }

        DaysCalendar { days_calendar: res }
    }

    /// Convert a DaysCalendar from BiDay to Day days
    #[allow(dead_code)]
    pub fn to_day<T>(calendar: DaysCalendar<BiDay>) -> DaysCalendar<Day> {
        let mut result = DaysCalendar {
            days_calendar: Vec::new(),
        };
    
        for (year, month, days) in calendar.days_calendar {
            let converted_days = days
                .iter()
                .enumerate()
                .filter_map(|(day, &bit)| {
                    if bit != BiDay::Zero {
                       Some(day as u8 + 1)
                    } else {
                      None
                    }  
                  })
                .collect();
    
            result.days_calendar.push((year, month, converted_days));

        }
    
        result
    }

    /// Convert a DaysCalendar from Day days to BiDay days
    #[allow(dead_code)]
    pub fn from_day(calendar: DaysCalendar<Day>) -> DaysCalendar<BiDay> {
        let mut result: DaysCalendar<BiDay> = DaysCalendar {
            days_calendar: Vec::new(),
        };
    
        for (year, month, days) in calendar.days_calendar {
            let n_days = days_in_month(year, month).unwrap_or(0);
          
            let mut bi_days: Vec<BiDay> = vec![BiDay::Zero; n_days as usize];
          
            for day in days {
              let idx = day - 1;
              
              bi_days[idx as usize] = BiDay::One;
            }
          
            result.days_calendar.push((year, month, bi_days)); 
          }

        result
    }

    impl DaysCalendar<Day> {
        // 
        pub fn new_days(data: Vec<(Year, Month, Vec<Day>)>) -> Self {
            Self {
                days_calendar: data,
            }
        }

        /// Sort days_calendar by Year and Month
        pub fn sort(&mut self) {
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
    
        /// Determines whether a date is contained in a DaysCalendar type
        pub fn contains(&mut self, year: Year, month: Month, day: Day) -> bool {
            // Sort days_calendar before performing binary search
            self.sort();
    
            // Perform a binary search on days_calendar to find the given year and month
            let index: Result<usize, usize> = self.days_calendar.binary_search_by(|&(y, m, _)| {
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

    //use std::convert::TryInto;

    /// Converts a `DaysCalendar` into a `Date` vector
    #[allow(dead_code)]
    pub fn to_date(calendar: DaysCalendar<BiDay>) -> Vec<Date> {
        let mut dates: Vec<Date> = Vec::new();
    
        for (year, month, days) in calendar.days_calendar {
            let year_num: Year = year;
            let month_num = match month.to_time_month() {
                Ok(m) => m,
                Err(_) => continue, // If month conversion fails, skip this month
            };
    
            for (day_index, day) in days.iter().enumerate() {
                if *day == BiDay::One {
                    let day_num = day_index as u8 + 1;
                    if let Ok(date) = Date::from_calendar_date(year_num.into(), month_num, day_num) {
                        dates.push(date);
                    }
                    // If date creation fails, just skip this day
                }
            }
        }
    
        dates
    }    

    /// Converts a vector of `Date` to a `DaysCalendar` type
    #[allow(dead_code)]
    pub fn from_date(dates: Vec<Date>) -> DaysCalendar<Day> {
        // Create an orderly map for storing years, months and days
        let mut year_month_map: super::BTreeMap<(Year, u8), Vec<Day>> = super::BTreeMap::new();
    
        // Iterate over the given dates and add them to the map
        for date in dates {
            let entry = year_month_map
                .entry((date.year().try_into().unwrap(), date.month().into()))
                .or_default(); //.or_insert_with(Vec::new);
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

    /// Groups calendar days by year and month
    #[allow(dead_code)]
    pub fn group_days_calendar<T: Clone>(calendar: DaysCalendar<T>) -> Vec<(Year, Month, Vec<Vec<T>>)> {
        // Group days by year and month
        let groups = calendar.days_calendar.iter().group_by(|(year, month, _)| (*year, *month));

        // Iterate on the groups and generate the new structure
        let mut result: Vec<(u16, Month, Vec<Vec<T>>)> = Vec::new();
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

    /// Converts a BiDay DaysCalendar to a vector of days
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



}

pub mod abstracto {

    use std::fmt;
    use std::fmt::{Display, Formatter};

    use crate::types::{
        Year, Month, Day, BiDay, Weekday,
        Date
    };
   
    /// Enumeration of abstract calendar patterns
    #[derive(Debug, Clone)]
    pub enum CalendarPattern {
        /// No calendar day
        None,
        
        /// All calendar days
        Everyday,

        /// Only on odd days of the month
        OddDays,
    
        /// Only on even days of the month
        EvenDays,
    
        /// Calendar based on days of the week
        /// Monday to Sunday
        Weekdays(Vec<Weekday>),
    
        /// Only on specific days of the week
        /// Customized days represented by numbers (1 = Monday, 7 = Sunday)
        CustomWeekDays(Vec<u8>),

        /// Personalized calendar based on the days of the month
        /// From day 1 to day 30
        CustomDays(Vec<Day>),
    
        /// A customized sequence of BiDays
        CustomBiDay(Vec<BiDay>),
    
        /// Only on a specific day of the month, e.g., only on the 15th of each month
        SpecificDayOfMonth(u8),
    
        /// Only days falling in a specific week of the month, e.g., the first week of the month
        SpecificWeekOfMonth(u8),
    
        /// Fixed public holidays
        FixedHolidays(Vec<Date>),
    
        /// Floating holidays
        FloatingHolidays(Vec<Date>),
    
        // Seasonal patterns, e.g., summer or winter only
        //Seasons(Vec<Season>), // definir un enum `Season` con opciones como `Spring`, `Summer`, etc.
    
        /// Only days that fall in a specific week of the year, e.g. week 10 of the year
        SpecificWeekOfYear(u32),
    
        /// A pattern defined by a custom function
        CustomFunction(fn(Year, Month, Day) -> BiDay),
    
        /// A pattern defined by a "cron" expression
        CronPattern(String),
    }

    /// Enum for the abstract calendar, which can be a single pattern or a combined operation
    /// Example of use for the calendar "working days":
    /// Not(Or(Or(FestivosFijos, FestivosFlotantes), FinesDeSemana))
    #[derive(Debug)]
    pub enum AbstractCalendar {
        Pattern(CalendarPattern),
        Operation(CalendarOperation),
    }

    /// Enum for operators and transformations that can be applied to patterns
    #[derive(Debug)]
    pub enum CalendarOperation {
        Invert(Box<AbstractCalendar>),                       // Inversión
        And(Box<AbstractCalendar>, Box<AbstractCalendar>),   // Intersección
        Or(Box<AbstractCalendar>, Box<AbstractCalendar>),    // Unión
        Minus(Box<AbstractCalendar>, Box<AbstractCalendar>), // Sustracción
    }

    
    impl AbstractCalendar {
        // Function to combine two abstract calendars with the OR operator
        pub fn or(self, other: AbstractCalendar) -> AbstractCalendar {
            AbstractCalendar::Operation(CalendarOperation::Or(Box::new(self), Box::new(other)))
        }
    
        // Function to combine two abstract calendars with the AND operator
        pub fn and(self, other: AbstractCalendar) -> AbstractCalendar {
            AbstractCalendar::Operation(CalendarOperation::And(Box::new(self), Box::new(other)))
        }
    
        // Function to negate (NOT) or invert an abstract calendar
        pub fn invert(self) -> AbstractCalendar {
            AbstractCalendar::Operation(CalendarOperation::Invert(Box::new(self)))
        }

        fn pretty_print(&self, f: &mut fmt::Formatter, indent: usize) -> fmt::Result {
            match self {
                AbstractCalendar::Pattern(pattern) => {
                    write!(f, "{:indent$}{}", "", pattern)
                },
                AbstractCalendar::Operation(op) => {
                    op.pretty_print(f, indent)
                }
            }
        }

    }


    impl fmt::Display for AbstractCalendar {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                AbstractCalendar::Pattern(pattern) => write!(f, "{}", pattern),
                AbstractCalendar::Operation(op) => write!(f, "{}", op),
            }
        }
    }
    
    impl Display for CalendarOperation {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            self.pretty_print(f, 0)
        }
    }

    impl CalendarOperation {
        fn pretty_print(&self, f: &mut Formatter<'_>, indent: usize) -> fmt::Result {
            match self {
                CalendarOperation::Invert(op) => {
                    writeln!(f, "{}Invert(", " ".repeat(indent))?;
                    op.pretty_print(f, indent + 4)?;
                    write!(f, "\n{})", " ".repeat(indent))
                }
                CalendarOperation::And(op1, op2) | CalendarOperation::Or(op1, op2) | CalendarOperation::Minus(op1, op2) => {
                    let operator = match self {
                        CalendarOperation::And(_, _) => "And",
                        CalendarOperation::Or(_, _) => "Or",
                        CalendarOperation::Minus(_, _) => "Minus",
                        _ => unreachable!(),
                    };
                    
                    writeln!(f, "{}{}(", " ".repeat(indent), operator)?;
                    op1.pretty_print(f, indent + 4)?;
                    writeln!(f, ",")?;
                    op2.pretty_print(f, indent + 4)?;
                    write!(f, "\n{})", " ".repeat(indent))
                }
            }
        }
    }

    impl fmt::Display for CalendarPattern {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                CalendarPattern::Everyday => write!(f, "Everyday"),
                CalendarPattern::Weekdays(weekdays) => {
                    let weekdays_str: String = weekdays.iter()
                        .map(|day: &Weekday| day.to_string())
                        .collect::<Vec<String>>()
                        .join(", ");
                    write!(f, "Weekdays([{}])", weekdays_str)
                },
                CalendarPattern::CustomDays(days) => {
                    let days_str: String = days.iter()
                        .map(|day| day.to_string())
                        .collect::<Vec<String>>()
                        .join(", ");
                    write!(f, "CustomDays([{}])", days_str)
                },
                CalendarPattern::CustomBiDay(bidays) => {
                    let bidays_str: String = bidays.iter()
                        .map(|biday: &BiDay| match biday {
                            BiDay::Zero => "0",
                            BiDay::One => "1",
                        })
                        .collect::<Vec<&str>>()
                        .join(", ");
                    write!(f, "CustomBiDay([{}])", bidays_str)
                },
                CalendarPattern::CronPattern(cron) => write!(f, "CronPattern(\"{}\")", cron),
                _ => todo!(),
            }
        }
    }
    
    
    


    


}






// =========================================
// SECTION FOR UNIT TEST CODE ...
// =========================================
#[cfg(test)]
mod tests_calendar {
    
    use crate::utils::*;
    use crate::types::*;
    use crate::binary::*;
    use crate::calendar::*;
    use std::collections::HashMap;
    use std::collections::HashSet;
    use std::iter::FromIterator;
    use std::vec::Vec;
    use time::macros::date;
    use time::Date;
    use time::Duration;
    use time::Month as TimeMonth;
    use time::Weekday;

    #[test]
    fn test_query_year_consolidate() {
        let days_calendar = DaysCalendar {
            days_calendar: vec![
                (2021, Month::January, vec![1, 0, 1]),
                (2021, Month::February, vec![1, 1, 0]),
                (2022, Month::January, vec![0, 1, 0]),
            ],
        };
        let expected = vec![
            ((2021, Month::January), vec![1, 0, 1]),
            ((2021, Month::February), vec![1, 1, 0]),
        ]
        .into_iter()
        .collect::<HashMap<(Year, Month), Vec<u32>>>();
        let result = query_year_consolidate(2021, &days_calendar);
        assert_eq!(result, expected);
    }

    #[test]
    fn singleton_valid_month() {
        let year: Year = 2023;
        let month: Month = Month::March;
        let days_calendar: DaysCalendar<BiDay> = DaysCalendar::singleton(year, month).unwrap();
        assert_eq!(days_calendar.days_calendar.len(), 1);
        assert_eq!(days_calendar.days_calendar[0].0, year);
        assert_eq!(days_calendar.days_calendar[0].1, month);
        assert_eq!(
            days_calendar.days_calendar[0].2.len(),
            days_in_month(year, month).unwrap() as usize
        );
    }

    #[test]
    fn test_extract_years_calendar_empty() {
        let dc = DaysCalendar::<BiDay>::empty();
        let vec = extract_years_calendar(&dc);
        assert_eq!(vec.len(), 0);
    }

    #[test]
    fn test_extract_years_calendar_singleton() {
        let dc = DaysCalendar::<BiDay>::singleton(2022, Month::January).unwrap();
        let vec = extract_years_calendar(&dc);
        assert_eq!(vec.len(), 1);
        assert_eq!(vec[0], 2022);
    }

    #[test]
    fn test_extract_year_month_calendar_empty() {
        let dc: DaysCalendar<BiDay> = DaysCalendar {
            days_calendar: vec![],
        };
        let result = extract_year_month_calendar(&dc);
        let expected: Vec<(Year, Month)> = vec![];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_extract_year_month_calendar_single_month() {
        let dc = DaysCalendar {
            days_calendar: vec![(2022, Month::January, vec![1, 0, 1, 0, 1, 0, 1])],
        };
        let result = extract_year_month_calendar(&dc);
        let expected: Vec<(Year, Month)> = vec![(2022, Month::January)];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_extract_year_month_calendar_multiple_months() {
        let dc = DaysCalendar {
            days_calendar: vec![
                (2022, Month::January, vec![1, 0, 1, 0, 1, 0, 1]),
                (2023, Month::March, vec![0, 1, 0, 1, 0, 1, 0]),
            ],
        };
        let result = extract_year_month_calendar(&dc);
        let expected: Vec<(Year, Month)> = vec![(2022, Month::January), (2023, Month::March)];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_extract_year_month_calendar_duplicate_months() {
        let dc = DaysCalendar {
            days_calendar: vec![
                (2022, Month::January, vec![1, 0, 1, 0, 1, 0, 1]),
                (2022, Month::January, vec![0, 1, 0, 1, 0, 1, 0]),
            ],
        };
        let result = extract_year_month_calendar(&dc);
        let expected: Vec<(Year, Month)> = vec![(2022, Month::January)];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_extract_day_month_calendar_empty() {
        let dc: DaysCalendar<BiDay> = DaysCalendar {
            days_calendar: vec![],
        };
        let result = extract_day_month_calendar(2022, Month::January, &dc);
        let expected: Option<Vec<BiDay>> = None;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_extract_day_month_calendar_not_found() {
        let dc = DaysCalendar {
            days_calendar: vec![
                (
                    2022,
                    Month::January,
                    vec![
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::One,
                    ],
                ),
                (
                    2023,
                    Month::March,
                    vec![
                        BiDay::Zero,
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::One,
                        BiDay::Zero,
                    ],
                ),
            ],
        };
        let result = extract_day_month_calendar(2022, Month::March, &dc);
        let expected: Option<Vec<BiDay>> = None;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_extract_day_month_calendar_single() {
        let dc = DaysCalendar {
            days_calendar: vec![
                (
                    2022,
                    Month::January,
                    vec![
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::One,
                    ],
                ),
                (
                    2023,
                    Month::March,
                    vec![
                        BiDay::Zero,
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::One,
                        BiDay::Zero,
                    ],
                ),
            ],
        };
        let result = extract_day_month_calendar(2022, Month::January, &dc).unwrap();
        let expected: Vec<BiDay> = vec![
            BiDay::One,
            BiDay::Zero,
            BiDay::One,
            BiDay::Zero,
            BiDay::One,
            BiDay::Zero,
            BiDay::One,
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_extract_day_month_calendar_multiple() {
        let dc = DaysCalendar {
            days_calendar: vec![
                (
                    2022,
                    Month::January,
                    vec![
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::One,
                    ],
                ),
                (
                    2022,
                    Month::February,
                    vec![
                        BiDay::One,
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::One,
                    ],
                ),
                (
                    2023,
                    Month::March,
                    vec![
                        BiDay::Zero,
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::One,
                        BiDay::Zero,
                    ],
                ),
            ],
        };
        let result = extract_day_month_calendar(2022, Month::February, &dc).unwrap();
        let expected: Vec<BiDay> = vec![
            BiDay::One,
            BiDay::One,
            BiDay::Zero,
            BiDay::Zero,
            BiDay::One,
            BiDay::Zero,
            BiDay::One,
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_or_daycalendar() {
        let v1 = vec![BiDay::One, BiDay::Zero, BiDay::One, BiDay::Zero];
        let v2 = vec![BiDay::Zero, BiDay::One, BiDay::One, BiDay::Zero];
        let result = or_daycalendar(&v1, &v2);
        assert_eq!(
            result,
            vec![BiDay::One, BiDay::One, BiDay::One, BiDay::Zero]
        );

        let v1 = vec![BiDay::Zero, BiDay::Zero, BiDay::One, BiDay::One];
        let v2 = vec![BiDay::Zero, BiDay::One, BiDay::One, BiDay::Zero];
        let result = or_daycalendar(&v1, &v2);
        assert_eq!(
            result,
            vec![BiDay::Zero, BiDay::One, BiDay::One, BiDay::One]
        );

        let v1 = vec![BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero];
        let expected_result = vec![BiDay::Zero, BiDay::One, BiDay::One, BiDay::Zero];
        assert_eq!(or_daycalendar(&v1, &v2), expected_result);

        let v1 = vec![
            BiDay::Zero,
            BiDay::One,
            BiDay::Zero,
            BiDay::One,
            BiDay::Zero,
        ];
        let v2 = vec![
            BiDay::One,
            BiDay::Zero,
            BiDay::Zero,
            BiDay::Zero,
            BiDay::One,
        ];
        let expected_result = vec![BiDay::One, BiDay::One, BiDay::Zero, BiDay::One, BiDay::One];
        assert_eq!(or_daycalendar(&v1, &v2), expected_result);

        let v1 = vec![BiDay::One, BiDay::Zero, BiDay::One];
        let v2 = vec![BiDay::One, BiDay::One, BiDay::Zero];
        let expected_result = vec![BiDay::One, BiDay::One, BiDay::One];
        assert_eq!(or_daycalendar(&v1, &v2), expected_result);
    }

    #[test]
    fn test_resume_days_with_and_operator() {
        let days = vec![
            (
                2022,
                Month::February,
                vec![BiDay::One, BiDay::Zero, BiDay::Zero, BiDay::One],
            ),
            (
                2022,
                Month::January,
                vec![BiDay::One, BiDay::Zero, BiDay::One, BiDay::Zero],
            ),
            (
                2022,
                Month::January,
                vec![BiDay::Zero, BiDay::Zero, BiDay::One, BiDay::One],
            ),
        ];
        let calendar = DaysCalendar {
            days_calendar: days,
        };
        let res = resume(&calendar, |a, b| and_biday_operation(a, b));
        assert_eq!(
            res,
            DaysCalendar {
                days_calendar: vec![
                    (
                        2022,
                        Month::January,
                        vec![
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::One,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero
                        ]
                    ),
                    (
                        2022,
                        Month::February,
                        vec![
                            BiDay::One,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::One,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero
                        ]
                    )
                ]
            }
        );
    }

    #[test]
    fn test_resume_days_with_or_operator() {
        let days = vec![
            (
                2022,
                Month::February,
                vec![BiDay::One, BiDay::Zero, BiDay::Zero, BiDay::One],
            ),
            (
                2022,
                Month::January,
                vec![BiDay::One, BiDay::Zero, BiDay::One, BiDay::Zero],
            ),
            (
                2022,
                Month::January,
                vec![BiDay::Zero, BiDay::Zero, BiDay::One, BiDay::One],
            ),
        ];
        let calendar = DaysCalendar {
            days_calendar: days,
        };
        let res = resume(&calendar, |a, b| or_biday_operation(a, b));
        assert_eq!(
            res,
            DaysCalendar {
                days_calendar: vec![
                    (
                        2022,
                        Month::January,
                        vec![
                            BiDay::One,
                            BiDay::Zero,
                            BiDay::One,
                            BiDay::One,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero
                        ]
                    ),
                    (
                        2022,
                        Month::February,
                        vec![
                            BiDay::One,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::One,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero
                        ]
                    )
                ]
            }
        );
    }

    #[test]
    fn test_resume_with_empty_calendar() {
        let calendar = DaysCalendar {
            days_calendar: vec![],
        };
        let res = resume(&calendar, |a, b| and_biday_operation(a, b));
        assert_eq!(
            res,
            DaysCalendar {
                days_calendar: vec![]
            }
        );
    }

    #[test]
    fn test_resume_with_single_day() {
        let days = vec![(
            2022,
            Month::January,
            vec![
                BiDay::One,
                BiDay::Zero,
                BiDay::Zero,
                BiDay::Zero,
                BiDay::Zero,
                BiDay::Zero,
                BiDay::Zero,
                BiDay::Zero,
                BiDay::Zero,
                BiDay::Zero,
                BiDay::Zero,
                BiDay::Zero,
                BiDay::Zero,
                BiDay::Zero,
                BiDay::Zero,
                BiDay::Zero,
                BiDay::Zero,
                BiDay::Zero,
                BiDay::Zero,
                BiDay::Zero,
                BiDay::Zero,
                BiDay::Zero,
                BiDay::Zero,
                BiDay::Zero,
                BiDay::Zero,
                BiDay::Zero,
                BiDay::Zero,
                BiDay::Zero,
                BiDay::Zero,
                BiDay::Zero,
                BiDay::Zero,
            ],
        )];
        let calendar = DaysCalendar {
            days_calendar: days,
        };
        let res = resume(&calendar, |a, b| and_biday_operation(a, b));
        assert_eq!(res, calendar);
    }

    #[test]
    fn test_resume_with_multiple_days() {
        let days = vec![
            (
                2022,
                Month::January,
                vec![BiDay::One, BiDay::Zero, BiDay::Zero, BiDay::Zero],
            ),
            (
                2022,
                Month::January,
                vec![BiDay::Zero, BiDay::One, BiDay::Zero, BiDay::Zero],
            ),
            (
                2022,
                Month::February,
                vec![BiDay::One, BiDay::One, BiDay::Zero, BiDay::Zero],
            ),
            (
                2022,
                Month::February,
                vec![BiDay::Zero, BiDay::Zero, BiDay::One, BiDay::Zero],
            ),
            (
                2023,
                Month::January,
                vec![BiDay::One, BiDay::One, BiDay::One, BiDay::Zero],
            ),
            (
                2023,
                Month::February,
                vec![BiDay::Zero, BiDay::One, BiDay::One, BiDay::One],
            ),
            (
                2023,
                Month::February,
                vec![BiDay::One, BiDay::Zero, BiDay::Zero, BiDay::One],
            ),
        ];
        let calendar = DaysCalendar {
            days_calendar: days,
        };
        let res = resume(&calendar, |a, b| and_biday_operation(a, b));
        assert_eq!(
            res,
            DaysCalendar {
                days_calendar: vec![
                    (
                        2022,
                        Month::January,
                        vec![
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero
                        ]
                    ),
                    (
                        2022,
                        Month::February,
                        vec![
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero
                        ]
                    ),
                    (
                        2023,
                        Month::January,
                        vec![
                            BiDay::One,
                            BiDay::One,
                            BiDay::One,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero
                        ]
                    ),
                    (
                        2023,
                        Month::February,
                        vec![
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::One,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero
                        ]
                    ),
                ]
            }
        );
    }

    #[test]
    fn test_resume_with_or_operator() {
        let days = vec![
            (
                2022,
                Month::January,
                vec![BiDay::One, BiDay::Zero, BiDay::Zero, BiDay::Zero],
            ),
            (
                2022,
                Month::January,
                vec![BiDay::Zero, BiDay::One, BiDay::Zero, BiDay::Zero],
            ),
            (
                2022,
                Month::February,
                vec![BiDay::One, BiDay::One, BiDay::Zero, BiDay::Zero],
            ),
            (
                2022,
                Month::February,
                vec![BiDay::Zero, BiDay::Zero, BiDay::One, BiDay::Zero],
            ),
            (
                2023,
                Month::January,
                vec![BiDay::One, BiDay::One, BiDay::One, BiDay::Zero],
            ),
            (
                2023,
                Month::February,
                vec![BiDay::Zero, BiDay::One, BiDay::One, BiDay::One],
            ),
            (
                2023,
                Month::February,
                vec![BiDay::One, BiDay::Zero, BiDay::Zero, BiDay::One],
            ),
        ];
        let calendar = DaysCalendar {
            days_calendar: days,
        };
        let res = resume(&calendar, |a, b| or_biday_operation(a, b));
        assert_eq!(
            res,
            DaysCalendar {
                days_calendar: vec![
                    (
                        2022,
                        Month::January,
                        vec![
                            BiDay::One,
                            BiDay::One,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero
                        ]
                    ),
                    (
                        2022,
                        Month::February,
                        vec![
                            BiDay::One,
                            BiDay::One,
                            BiDay::One,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero
                        ]
                    ),
                    (
                        2023,
                        Month::January,
                        vec![
                            BiDay::One,
                            BiDay::One,
                            BiDay::One,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero
                        ]
                    ),
                    (
                        2023,
                        Month::February,
                        vec![
                            BiDay::One,
                            BiDay::One,
                            BiDay::One,
                            BiDay::One,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero,
                            BiDay::Zero
                        ]
                    ),
                ]
            }
        );
    }

    #[test]
    fn test_add_empty_calendars() {
        let empty_cal = DaysCalendar {
            days_calendar: Vec::new(),
        };
        let res = empty_cal.append(&empty_cal);
        assert_eq!(res, empty_cal);
    }

    #[test]
    fn test_append_empty_calendar_to_nonempty_calendar() {
        let nonempty_cal = DaysCalendar {
            days_calendar: vec![
                (
                    2022,
                    Month::January,
                    vec![
                        BiDay::One,
                        BiDay::One,
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                    ],
                ),
                (
                    2022,
                    Month::February,
                    vec![
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::One,
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                    ],
                ),
                (
                    2022,
                    Month::March,
                    vec![
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                    ],
                ),
            ],
        };
        let empty_cal = DaysCalendar {
            days_calendar: Vec::new(),
        };
        let res = nonempty_cal.append(&empty_cal);
        assert_eq!(res, nonempty_cal);
    }

    #[test]
    fn test_append_nonempty_calendar_to_empty_calendar() {
        let nonempty_cal = DaysCalendar {
            days_calendar: vec![
                (
                    2022,
                    Month::January,
                    vec![
                        BiDay::One,
                        BiDay::One,
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                    ],
                ),
                (
                    2022,
                    Month::February,
                    vec![
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::One,
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                    ],
                ),
                (
                    2022,
                    Month::March,
                    vec![
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                    ],
                ),
            ],
        };
        let empty_cal = DaysCalendar {
            days_calendar: Vec::new(),
        };
        let res = empty_cal.append(&nonempty_cal);
        assert_eq!(res, nonempty_cal);
    }

    #[test]
    fn test_append_nonempty_calendars_same_month() {
        let cal1 = DaysCalendar {
            days_calendar: vec![(
                2022,
                Month::January,
                vec![BiDay::One, BiDay::One, BiDay::One, BiDay::Zero],
            )],
        };
        let cal2 = DaysCalendar {
            days_calendar: vec![(
                2022,
                Month::January,
                vec![BiDay::Zero, BiDay::One, BiDay::Zero, BiDay::One],
            )],
        };
        let expected_res = DaysCalendar {
            days_calendar: vec![
                (
                    2022,
                    Month::January,
                    vec![BiDay::One, BiDay::One, BiDay::One, BiDay::Zero],
                ),
                (
                    2022,
                    Month::January,
                    vec![BiDay::Zero, BiDay::One, BiDay::Zero, BiDay::One],
                ),
            ],
        };
        let res = cal1.append(&cal2);
        assert_eq!(res, expected_res);
    }

    #[test]
    fn test_append_nonempty_calendars_different_months() {
        let cal1 = DaysCalendar {
            days_calendar: vec![(
                2022,
                Month::January,
                vec![BiDay::One, BiDay::One, BiDay::One, BiDay::Zero],
            )],
        };
        let cal2 = DaysCalendar {
            days_calendar: vec![(
                2022,
                Month::February,
                vec![BiDay::Zero, BiDay::One, BiDay::Zero, BiDay::One],
            )],
        };
        let expected_res = DaysCalendar {
            days_calendar: vec![
                (
                    2022,
                    Month::January,
                    vec![BiDay::One, BiDay::One, BiDay::One, BiDay::Zero],
                ),
                (
                    2022,
                    Month::February,
                    vec![BiDay::Zero, BiDay::One, BiDay::Zero, BiDay::One],
                ),
            ],
        };
        let res = cal1.append(&cal2);
        assert_eq!(res, expected_res);
    }

    #[test]
    fn test_or() {
        let days1 = vec![
            (
                2022,
                Month::February,
                vec![
                    BiDay::One,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::One,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                ],
            ),
            (
                2022,
                Month::January,
                vec![
                    BiDay::One,
                    BiDay::Zero,
                    BiDay::One,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                ],
            ),
        ];
        let days2 = vec![
            (
                2022,
                Month::February,
                vec![
                    BiDay::Zero,
                    BiDay::One,
                    BiDay::One,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                ],
            ),
            (
                2022,
                Month::January,
                vec![
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::One,
                    BiDay::One,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                ],
            ),
        ];
        let calendar1 = DaysCalendar {
            days_calendar: days1,
        };
        let calendar2 = DaysCalendar {
            days_calendar: days2,
        };

        let result = calendar1.or(&calendar2);

        let expected_days = vec![
            (
                2022,
                Month::January,
                vec![
                    BiDay::One,
                    BiDay::Zero,
                    BiDay::One,
                    BiDay::One,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                ],
            ),
            (
                2022,
                Month::February,
                vec![
                    BiDay::One,
                    BiDay::One,
                    BiDay::One,
                    BiDay::One,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                ],
            ),
        ];
        let expected_calendar = DaysCalendar {
            days_calendar: expected_days,
        };

        assert_eq!(result, expected_calendar);
    }

    #[test]
    fn test_and() {
        let days1 = vec![
            (
                2022,
                Month::February,
                vec![
                    BiDay::One,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::One,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                ],
            ),
            (
                2022,
                Month::January,
                vec![
                    BiDay::One,
                    BiDay::Zero,
                    BiDay::One,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                ],
            ),
        ];
        let days2 = vec![
            (
                2022,
                Month::February,
                vec![
                    BiDay::Zero,
                    BiDay::One,
                    BiDay::One,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                ],
            ),
            (
                2022,
                Month::January,
                vec![
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::One,
                    BiDay::One,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                ],
            ),
        ];
        let calendar1 = DaysCalendar {
            days_calendar: days1,
        };
        let calendar2 = DaysCalendar {
            days_calendar: days2,
        };

        let result = calendar1.and(&calendar2);

        let expected_days = vec![
            (
                2022,
                Month::January,
                vec![
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::One,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                ],
            ),
            (
                2022,
                Month::February,
                vec![
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                ],
            ),
        ];
        let expected_calendar = DaysCalendar {
            days_calendar: expected_days,
        };

        assert_eq!(result, expected_calendar);
    }

    #[test]
    fn test_match() {
        let days1 = vec![
            (
                2022,
                Month::February,
                vec![BiDay::One, BiDay::Zero, BiDay::Zero, BiDay::One],
            ),
            (
                2022,
                Month::January,
                vec![BiDay::One, BiDay::Zero, BiDay::One, BiDay::Zero],
            ),
        ];
        let days2 = vec![
            (
                2022,
                Month::February,
                vec![BiDay::Zero, BiDay::One, BiDay::One, BiDay::Zero],
            ),
            (
                2022,
                Month::January,
                vec![BiDay::Zero, BiDay::Zero, BiDay::One, BiDay::One],
            ),
        ];
        let calendar1 = DaysCalendar {
            days_calendar: days1,
        };
        let calendar2 = DaysCalendar {
            days_calendar: days2,
        };

        let result = calendar1.r#match(&calendar2);

        let expected_days = vec![
            (
                2022,
                Month::January,
                vec![
                    BiDay::Zero,
                    BiDay::One,
                    BiDay::One,
                    BiDay::Zero,
                    BiDay::One,
                    BiDay::One,
                    BiDay::One,
                    BiDay::One,
                    BiDay::One,
                    BiDay::One,
                    BiDay::One,
                    BiDay::One,
                    BiDay::One,
                    BiDay::One,
                    BiDay::One,
                    BiDay::One,
                    BiDay::One,
                    BiDay::One,
                    BiDay::One,
                    BiDay::One,
                    BiDay::One,
                    BiDay::One,
                    BiDay::One,
                    BiDay::One,
                    BiDay::One,
                    BiDay::One,
                    BiDay::One,
                    BiDay::One,
                    BiDay::One,
                    BiDay::One,
                    BiDay::One,
                ],
            ),
            (
                2022,
                Month::February,
                vec![
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::One,
                    BiDay::One,
                    BiDay::One,
                    BiDay::One,
                    BiDay::One,
                    BiDay::One,
                    BiDay::One,
                    BiDay::One,
                    BiDay::One,
                    BiDay::One,
                    BiDay::One,
                    BiDay::One,
                    BiDay::One,
                    BiDay::One,
                    BiDay::One,
                    BiDay::One,
                    BiDay::One,
                    BiDay::One,
                    BiDay::One,
                    BiDay::One,
                    BiDay::One,
                    BiDay::One,
                    BiDay::One,
                    BiDay::One,
                ],
            ),
        ];
        let expected_calendar = DaysCalendar {
            days_calendar: expected_days,
        };

        assert_eq!(result, expected_calendar);
    }

    #[test]
    fn test_nomatch() {
        let days1 = vec![
            (
                2022,
                Month::February,
                vec![BiDay::One, BiDay::Zero, BiDay::Zero, BiDay::One],
            ),
            (
                2022,
                Month::January,
                vec![BiDay::One, BiDay::Zero, BiDay::One, BiDay::Zero],
            ),
        ];
        let days2 = vec![
            (
                2022,
                Month::February,
                vec![BiDay::Zero, BiDay::One, BiDay::One, BiDay::Zero],
            ),
            (
                2022,
                Month::January,
                vec![BiDay::Zero, BiDay::Zero, BiDay::One, BiDay::One],
            ),
        ];
        let calendar1 = DaysCalendar {
            days_calendar: days1,
        };
        let calendar2 = DaysCalendar {
            days_calendar: days2,
        };

        let result = calendar1.nomatch(&calendar2);

        let expected_days = vec![
            (
                2022,
                Month::January,
                vec![
                    BiDay::One,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::One,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                ],
            ),
            (
                2022,
                Month::February,
                vec![
                    BiDay::One,
                    BiDay::One,
                    BiDay::One,
                    BiDay::One,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                    BiDay::Zero,
                ],
            ),
        ];
        let expected_calendar = DaysCalendar {
            days_calendar: expected_days,
        };

        assert_eq!(result, expected_calendar);
    }

    #[test]
    fn test_replicate_pattern_single() {
        let pattern: [BiDay; 1] = [BiDay::One];
        let n = 5;
        let expected = vec![BiDay::One, BiDay::One, BiDay::One, BiDay::One, BiDay::One];
        assert_eq!(replicate_pattern(&pattern, n), expected);
    }

    #[test]
    fn test_replicate_pattern_two() {
        let pattern: [BiDay; 2] = [BiDay::One, BiDay::Zero];
        let n = 5;
        let expected = vec![BiDay::One, BiDay::Zero, BiDay::One, BiDay::Zero, BiDay::One];
        assert_eq!(replicate_pattern(&pattern, n), expected);
    }

    #[test]
    fn test_replicate_pattern_two_reverse() {
        let pattern: [BiDay; 2] = [BiDay::Zero, BiDay::One];
        let n = 5;
        let expected = vec![
            BiDay::Zero,
            BiDay::One,
            BiDay::Zero,
            BiDay::One,
            BiDay::Zero,
        ];
        assert_eq!(replicate_pattern(&pattern, n), expected);
    }

    #[test]
    fn test_replicate_pattern_multi() {
        let pattern: [BiDay; 5] = [BiDay::One, BiDay::Zero, BiDay::Zero, BiDay::One, BiDay::One];
        let n = 4;
        let expected = vec![BiDay::One, BiDay::Zero, BiDay::Zero, BiDay::One];
        assert_eq!(replicate_pattern(&pattern, n), expected);
    }

    #[test]
    fn test_replicate_pattern_multi_longer() {
        let pattern: [BiDay; 5] = [BiDay::One, BiDay::Zero, BiDay::Zero, BiDay::One, BiDay::One];
        let n = 10;
        let expected = vec![
            BiDay::One,
            BiDay::Zero,
            BiDay::Zero,
            BiDay::One,
            BiDay::One,
            BiDay::One,
            BiDay::Zero,
            BiDay::Zero,
            BiDay::One,
            BiDay::One,
        ];
        assert_eq!(replicate_pattern(&pattern, n), expected);
    }

    #[test]
    fn test_replicate_with_empty_calendar() {
        let calendar = DaysCalendar {
            days_calendar: Vec::new(),
        };
        let pattern: [BiDay; 3] = [BiDay::Zero, BiDay::One, BiDay::One];
        let result = replicate::<BiDay>(&pattern, calendar);
        assert_eq!(result.days_calendar, Vec::new());
    }

    #[test]
    fn test_replicate_with_single_day() {
        let calendar = DaysCalendar {
            days_calendar: vec![(2023, Month::January, vec![BiDay::Zero])],
        };
        let pattern: [BiDay; 3] = [BiDay::Zero, BiDay::One, BiDay::Zero];
        let result = replicate::<BiDay>(&pattern, calendar);
        assert_eq!(
            result.days_calendar,
            vec![(2023, Month::January, vec![BiDay::Zero]),]
        );
    }

    #[test]
    fn test_replicate_with_multiple_days() {
        let calendar = DaysCalendar {
            days_calendar: vec![
                (
                    2023,
                    Month::January,
                    vec![BiDay::Zero, BiDay::One, BiDay::One, BiDay::One, BiDay::One],
                ),
                (
                    2023,
                    Month::February,
                    vec![BiDay::One, BiDay::One, BiDay::One],
                ),
                (
                    2023,
                    Month::March,
                    vec![BiDay::One, BiDay::Zero, BiDay::Zero, BiDay::One],
                ),
            ],
        };
        let pattern: [BiDay; 3] = [BiDay::Zero, BiDay::One, BiDay::One];
        let result = replicate::<BiDay>(&pattern, calendar);
        assert_eq!(
            result.days_calendar,
            vec![
                (
                    2023,
                    Month::January,
                    vec![BiDay::Zero, BiDay::One, BiDay::One, BiDay::Zero, BiDay::One]
                ),
                (
                    2023,
                    Month::February,
                    vec![BiDay::Zero, BiDay::One, BiDay::One]
                ),
                (
                    2023,
                    Month::March,
                    vec![BiDay::Zero, BiDay::One, BiDay::One, BiDay::Zero]
                ),
            ]
        );
    }

    #[test]
    fn test_replicate_with_large_pattern() {
        let calendar = DaysCalendar {
            days_calendar: vec![
                (
                    2023,
                    Month::January,
                    vec![BiDay::Zero, BiDay::One, BiDay::One],
                ),
                (
                    2023,
                    Month::February,
                    vec![BiDay::One, BiDay::One, BiDay::One],
                ),
            ],
        };
        let pattern: [BiDay; 6] = [
            BiDay::Zero,
            BiDay::One,
            BiDay::One,
            BiDay::One,
            BiDay::One,
            BiDay::One,
        ];
        let result = replicate::<BiDay>(&pattern, calendar);
        assert_eq!(
            result.days_calendar,
            vec![
                (
                    2023,
                    Month::January,
                    vec![BiDay::Zero, BiDay::One, BiDay::One]
                ),
                (
                    2023,
                    Month::February,
                    vec![BiDay::Zero, BiDay::One, BiDay::One]
                ),
            ]
        );
    }

    #[test]
    fn test_zeros() {
        // Create a DaysCalendar object with some sample data
        let calendar = DaysCalendar {
            days_calendar: vec![
                (
                    2023,
                    Month::January,
                    vec![
                        BiDay::One,
                        BiDay::One,
                        BiDay::One,
                        BiDay::One,
                        BiDay::One,
                        BiDay::One,
                        BiDay::One,
                    ],
                ),
                (
                    2023,
                    Month::February,
                    vec![
                        BiDay::One,
                        BiDay::One,
                        BiDay::One,
                        BiDay::One,
                        BiDay::One,
                        BiDay::One,
                        BiDay::One,
                    ],
                ),
                (
                    2023,
                    Month::March,
                    vec![
                        BiDay::One,
                        BiDay::One,
                        BiDay::One,
                        BiDay::One,
                        BiDay::One,
                        BiDay::One,
                        BiDay::One,
                    ],
                ),
            ],
        };

        // Call the zeros() function on the DaysCalendar object
        let result = calendar.zeros();

        // Validate the result
        assert_eq!(
            result.days_calendar,
            vec![
                (
                    2023,
                    Month::January,
                    vec![
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero
                    ]
                ),
                (
                    2023,
                    Month::February,
                    vec![
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero
                    ]
                ),
                (
                    2023,
                    Month::March,
                    vec![
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero
                    ]
                ),
            ]
        );
    }

    #[test]
    fn test_zeros_full() {
        let calendar = DaysCalendar::<BiDay> {
            days_calendar: vec![],
        };
        let zeros_calendar = calendar.zeros();
        assert_eq!(zeros_calendar.days_calendar.len(), 0);

        let days = vec![
            (2023, Month::March, vec![BiDay::Zero; 31]),
            (2023, Month::April, vec![BiDay::Zero; 30]),
        ];
        let calendar = DaysCalendar::<BiDay> {
            days_calendar: days,
        };
        let zeros_calendar = calendar.zeros();
        assert_eq!(zeros_calendar.days_calendar.len(), 2);
        assert_eq!(zeros_calendar.days_calendar[0].2.len(), 31);
        assert_eq!(zeros_calendar.days_calendar[1].2.len(), 30);
        for (_, _, days) in &zeros_calendar.days_calendar {
            for day in days {
                assert_eq!(*day, BiDay::Zero);
            }
        }
    }

    #[test]
    fn test_ones() {
        let calendar = DaysCalendar::<BiDay> {
            days_calendar: vec![],
        };
        let ones_calendar = calendar.ones();
        assert_eq!(ones_calendar.days_calendar.len(), 0);

        let days = vec![
            (2023, Month::March, vec![BiDay::Zero; 31]),
            (2023, Month::April, vec![BiDay::Zero; 30]),
        ];
        let calendar = DaysCalendar::<BiDay> {
            days_calendar: days,
        };
        let ones_calendar = calendar.ones();
        assert_eq!(ones_calendar.days_calendar.len(), 2);
        assert_eq!(ones_calendar.days_calendar[0].2.len(), 31);
        assert_eq!(ones_calendar.days_calendar[1].2.len(), 30);
        for (_, _, days) in &ones_calendar.days_calendar {
            for day in days {
                assert_eq!(*day, BiDay::One);
            }
        }
    }

    #[test]
    fn test_upward_step_with_empty_calendar() {
        let calendar = DaysCalendar::empty();
        let expected_calendar = DaysCalendar::empty();
        let result_calendar = calendar.upward_step();
        assert_eq!(result_calendar, expected_calendar);
    }

    #[test]
    fn test_upward_step_with_single_day_calendar() {
        let mut calendar: DaysCalendar<BiDay> =
            DaysCalendar::singleton(2023, Month::March).unwrap();
        let expected_calendar = DaysCalendar {
            days_calendar: vec![(
                2023,
                Month::March,
                vec![
                    BiDay::Zero,
                    BiDay::One,
                    BiDay::Zero,
                    BiDay::One,
                    BiDay::Zero,
                    BiDay::One,
                    BiDay::Zero,
                    BiDay::One,
                    BiDay::Zero,
                    BiDay::One,
                    BiDay::Zero,
                    BiDay::One,
                    BiDay::Zero,
                    BiDay::One,
                    BiDay::Zero,
                    BiDay::One,
                    BiDay::Zero,
                    BiDay::One,
                    BiDay::Zero,
                    BiDay::One,
                    BiDay::Zero,
                    BiDay::One,
                    BiDay::Zero,
                    BiDay::One,
                    BiDay::Zero,
                    BiDay::One,
                    BiDay::Zero,
                    BiDay::One,
                    BiDay::Zero,
                    BiDay::One,
                    BiDay::Zero,
                ],
            )],
        };
        let result_calendar = calendar.upward_step();
        assert_eq!(result_calendar, expected_calendar);
    }

    #[test]
    fn test_upward_step_with_multi_day_calendar() {
        let cal1 = DaysCalendar::singleton(2023, Month::March).unwrap();
        let cal2 = DaysCalendar::singleton(2023, Month::May).unwrap();
        let calendar = cal1.append(&cal2);

        let expected_calendar = DaysCalendar {
            days_calendar: vec![
                (
                    2023,
                    Month::March,
                    vec![
                        BiDay::Zero,
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::One,
                        BiDay::Zero,
                    ],
                ),
                (
                    2023,
                    Month::May,
                    vec![
                        BiDay::Zero,
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::One,
                        BiDay::Zero,
                    ],
                ),
            ],
        };
        let result_calendar = calendar.upward_step();
        assert_eq!(result_calendar, expected_calendar);
    }

    #[test]
    fn test_downward_step_with_multi_day_calendar() {
        let cal1 = DaysCalendar::singleton(2023, Month::March).unwrap();
        let cal2 = DaysCalendar::singleton(2023, Month::May).unwrap();
        let calendar = cal1.append(&cal2);

        let expected_calendar = DaysCalendar {
            days_calendar: vec![
                (
                    2023,
                    Month::March,
                    vec![
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::One,
                    ],
                ),
                (
                    2023,
                    Month::May,
                    vec![
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::One,
                    ],
                ),
            ],
        };
        let result_calendar = calendar.downward_step();
        assert_eq!(result_calendar, expected_calendar);
    }

    fn test_new() {
        let data = vec![
            (
                2022,
                Month::January,
                vec![BiDay::One, BiDay::One, BiDay::Zero],
            ),
            (
                2022,
                Month::February,
                vec![BiDay::Zero, BiDay::One, BiDay::One],
            ),
            (
                2022,
                Month::March,
                vec![BiDay::One, BiDay::Zero, BiDay::One],
            ),
        ];
        let calendar: DaysCalendar<BiDay> = DaysCalendar::new(data.clone());
        assert_eq!(calendar.days_calendar, data);
    }

    #[test]
    fn test_new_empty() {
        let calendar = DaysCalendar::<BiDay>::new(Vec::new());
        assert_eq!(
            calendar.days_calendar,
            Vec::<(Year, Month, Vec<BiDay>)>::new()
        );
    }

    #[test]
    fn test_to_day() {
        let bi_data = vec![
            (
                2022,
                Month::January,
                vec![BiDay::Zero, BiDay::Zero, BiDay::One],
            ),
            (
                2022,
                Month::February,
                vec![BiDay::One, BiDay::Zero, BiDay::One],
            ),
            (
                2022,
                Month::March,
                vec![BiDay::Zero, BiDay::One, BiDay::One],
            ),
        ];
        let calendar_bi = DaysCalendar::new(bi_data.clone());

        let day_data = vec![
            (2022 as Year, Month::January as Month, vec![3 as u8]),
            (
                2022 as Year,
                Month::February as Month,
                vec![1 as u8, 3 as u8],
            ),
            (2022 as Year, Month::March as Month, vec![2 as u8, 3 as u8]),
        ];
        let calendar_day = DaysCalendar::new_days(day_data.clone());

        assert_eq!(to_day::<BiDay>(calendar_bi), calendar_day);
    }

    #[test]
    fn test_to_day_empty() {
        let calendar_bi = DaysCalendar::<BiDay>::new(Vec::new());
        let calendar_day = DaysCalendar::<Day>::new_days(Vec::new());

        assert_eq!(to_day::<BiDay>(calendar_bi), calendar_day);
    }

    fn create_test_calendar_1() -> DaysCalendar<BiDay> {
        DaysCalendar {
            days_calendar: vec![
                (
                    2023,
                    Month::January,
                    vec![BiDay::Zero, BiDay::One, BiDay::One],
                ),
                (
                    2023,
                    Month::February,
                    vec![BiDay::One, BiDay::Zero, BiDay::One],
                ),
                (
                    2023,
                    Month::March,
                    vec![BiDay::Zero, BiDay::Zero, BiDay::One],
                ),
            ],
        }
    }

    #[test]
    fn test_invert_biday_values() {
        let calendar = create_test_calendar_1();
        let inverted_calendar = calendar.invert_biday();

        let expected_inverted_calendar = DaysCalendar {
            days_calendar: vec![
                (
                    2023,
                    Month::January,
                    vec![BiDay::One, BiDay::Zero, BiDay::Zero],
                ),
                (
                    2023,
                    Month::February,
                    vec![BiDay::Zero, BiDay::One, BiDay::Zero],
                ),
                (
                    2023,
                    Month::March,
                    vec![BiDay::One, BiDay::One, BiDay::Zero],
                ),
            ],
        };

        assert_eq!(inverted_calendar, expected_inverted_calendar);
    }

    fn setup_calendar() -> DaysCalendar<BiDay> {
        DaysCalendar {
            days_calendar: vec![
                (
                    2023,
                    Month::January,
                    vec![BiDay::Zero, BiDay::One, BiDay::Zero],
                ),
                (
                    2023,
                    Month::February,
                    vec![BiDay::One, BiDay::Zero, BiDay::One],
                ),
                (2023, Month::March, vec![BiDay::Zero, BiDay::One]),
            ],
        }
    }

    #[test]
    fn test_take_n_biday_zero() {
        let calendar = setup_calendar();
        let result = calendar.take_n_biday(0);
        assert_eq!(result.days_calendar.len(), 0);
    }

    #[test]
    fn test_take_n_biday_less_than_month() {
        let calendar = setup_calendar();
        let result = calendar.take_n_biday(2);
        let expected = DaysCalendar {
            days_calendar: vec![(2023, Month::January, vec![BiDay::Zero, BiDay::One])],
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn test_take_n_biday_across_months() {
        let calendar = setup_calendar();
        let result = calendar.take_n_biday(5);
        let expected = DaysCalendar {
            days_calendar: vec![
                (
                    2023,
                    Month::January,
                    vec![BiDay::Zero, BiDay::One, BiDay::Zero],
                ),
                (2023, Month::February, vec![BiDay::One, BiDay::Zero]),
            ],
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn test_take_n_biday_more_than_total() {
        let calendar = setup_calendar();
        let result = calendar.take_n_biday(10);
        let expected = setup_calendar();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_to_date_empty_calendar() {
        let calendar = DaysCalendar {
            days_calendar: vec![],
        };
        let dates = to_date(calendar);
        assert_eq!(dates, vec![]);
    }

    #[test]
    fn test_to_date_single_month() {
        let calendar = DaysCalendar {
            days_calendar: vec![(
                2023,
                Month::March,
                vec![BiDay::Zero, BiDay::One, BiDay::Zero],
            )],
        };
        let dates = to_date(calendar);
        assert_eq!(
            dates,
            vec![Date::from_calendar_date(2023, Month::March.to_time_month().unwrap(), 2).unwrap(),]
        );
    }

    #[test]
    fn test_to_date_multiple_months() {
        let calendar = DaysCalendar {
            days_calendar: vec![
                (2023, Month::February, vec![BiDay::Zero, BiDay::One]),
                (
                    2023,
                    Month::March,
                    vec![BiDay::Zero, BiDay::One, BiDay::Zero],
                ),
            ],
        };
        let dates = to_date(calendar);
        assert_eq!(
            dates,
            vec![
                Date::from_calendar_date(2023, Month::February.to_time_month().unwrap(), 2).unwrap(),
                Date::from_calendar_date(2023, Month::March.to_time_month().unwrap(), 2).unwrap(),
            ]
        );
    }

    #[test]
    fn test_to_date_multiple_years() {
        let calendar = DaysCalendar {
            days_calendar: vec![
                (2022, Month::December, vec![BiDay::Zero, BiDay::One]),
                (
                    2023,
                    Month::January,
                    vec![BiDay::Zero, BiDay::One, BiDay::Zero],
                ),
            ],
        };
        let dates = to_date(calendar);
        assert_eq!(
            dates,
            vec![
                Date::from_calendar_date(2022, Month::December.to_time_month().unwrap(), 2).unwrap(),
                Date::from_calendar_date(2023, Month::January.to_time_month().unwrap(), 2).unwrap(),
            ]
        );
    }

    #[test]
    fn test_month_to_index() {
        assert_eq!(Month::January.to_index(), 1);
        assert_eq!(Month::February.to_index(), 2);
        assert_eq!(Month::March.to_index(), 3);
        assert_eq!(Month::April.to_index(), 4);
        assert_eq!(Month::May.to_index(), 5);
        assert_eq!(Month::June.to_index(), 6);
        assert_eq!(Month::July.to_index(), 7);
        assert_eq!(Month::August.to_index(), 8);
        assert_eq!(Month::September.to_index(), 9);
        assert_eq!(Month::October.to_index(), 10);
        assert_eq!(Month::November.to_index(), 11);
        assert_eq!(Month::December.to_index(), 12);
    }

    #[test]
    fn test_from_date_empty() {
        let dates: Vec<Date> = vec![];
        let calendar = from_date(dates);
        assert!(calendar.days_calendar.is_empty());
    }

    #[test]
    fn test_from_date_single_date() {
        let dates = vec![date!(2022 - 03 - 30)];
        let calendar = from_date(dates);
        assert_eq!(calendar.days_calendar.len(), 1);
        assert_eq!(calendar.days_calendar[0], (2022, Month::March, vec![30]));
    }

    #[test]
    fn test_from_date_same_month() {
        let dates = vec![
            date!(2022 - 03 - 30),
            date!(2022 - 03 - 15),
            date!(2022 - 03 - 20),
        ];
        let calendar = from_date(dates);
        assert_eq!(calendar.days_calendar.len(), 1);
        assert_eq!(
            calendar.days_calendar[0],
            (2022, Month::March, vec![15, 20, 30])
        );
    }

    #[test]
    fn test_from_date_different_months() {
        let dates = vec![
            date!(2022 - 03 - 30),
            date!(2022 - 02 - 15),
            date!(2022 - 01 - 20),
        ];
        let calendar = from_date(dates);
        assert_eq!(calendar.days_calendar.len(), 3);
        assert_eq!(calendar.days_calendar[0], (2022, Month::January, vec![20]));
        assert_eq!(calendar.days_calendar[1], (2022, Month::February, vec![15]));
        assert_eq!(calendar.days_calendar[2], (2022, Month::March, vec![30]));
    }

    #[test]
    fn test_from_date_different_years() {
        let dates = vec![
            date!(2023 - 03 - 30),
            date!(2022 - 03 - 15),
            date!(2021 - 03 - 20),
        ];
        let calendar = from_date(dates);
        assert_eq!(calendar.days_calendar.len(), 3);
        assert_eq!(calendar.days_calendar[0], (2021, Month::March, vec![20]));
        assert_eq!(calendar.days_calendar[1], (2022, Month::March, vec![15]));
        assert_eq!(calendar.days_calendar[2], (2023, Month::March, vec![30]));
    }

    #[test]
    fn test_from_date_duplicate_dates() {
        let dates = vec![
            date!(2022 - 03 - 30),
            date!(2022 - 03 - 15),
            date!(2022 - 03 - 30),
        ];
        let calendar = from_date(dates);
        assert_eq!(calendar.days_calendar.len(), 1);
        assert_eq!(
            calendar.days_calendar[0],
            (2022, Month::March, vec![15, 30])
        );
    }

    #[test]
    fn test_from_day() {
        let input_calendar = DaysCalendar {
            days_calendar: vec![
                (2020, Month::January, vec![1, 15, 31]),
                (2020, Month::February, vec![14, 29]),
                (2020, Month::March, vec![1, 31]),
            ],
        };

        let expected_calendar = DaysCalendar {
            days_calendar: vec![
                (
                    2020,
                    Month::January,
                    vec![
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::One,
                    ],
                ),
                (
                    2020,
                    Month::February,
                    vec![
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::One,
                    ],
                ),
                (
                    2020,
                    Month::March,
                    vec![
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::One,
                    ],
                ),
            ],
        };

        let output_calendar = from_day(input_calendar);

        assert_eq!(output_calendar, expected_calendar);
    }

    #[test]
    fn test_move_days() {
        let dates = vec![
            date!(2020 - 01 - 01),
            date!(2020 - 02 - 29),
            date!(2020 - 12 - 31),
        ];

        let expected_add_5 = vec![
            date!(2020 - 01 - 06),
            date!(2020 - 03 - 05),
            date!(2021 - 01 - 05),
        ];

        let expected_sub_5 = vec![
            date!(2019 - 12 - 27),
            date!(2020 - 02 - 24),
            date!(2020 - 12 - 26),
        ];

        assert_eq!(add_days(dates.clone(), 5), expected_add_5);
        assert_eq!(add_days(dates.clone(), -5), expected_sub_5);
        assert_eq!(add_days(dates.clone(), 0), dates);
    }

    #[test]
    fn test_move_days_edge_cases() {
        let dates = vec![date!(0001 - 01 - 01), date!(9999 - 12 - 31)];

        let expected_add_max = vec![MAX_DATE, MAX_DATE];

        let expected_sub_max = vec![MIN_DATE, MIN_DATE];

        assert_eq!(add_days(dates.clone(), i32::MAX), expected_add_max);
        assert_eq!(add_days(dates.clone(), i32::MIN + 1), expected_sub_max);
    }

    fn create_unsorted_days_calendar() -> DaysCalendar<Day> {
        DaysCalendar {
            days_calendar: vec![
                (2020, Month::March, vec![1, 3, 5]),
                (2020, Month::January, vec![15, 1, 31]),
                (2020, Month::February, vec![29, 14, 1]),
            ],
        }
    }

    #[test]
    fn test_sort() {
        let mut days_calendar = create_unsorted_days_calendar();
        days_calendar.sort();

        let expected_calendar = DaysCalendar {
            days_calendar: vec![
                (2020, Month::January, vec![1, 15, 31]),
                (2020, Month::February, vec![1, 14, 29]),
                (2020, Month::March, vec![1, 3, 5]),
            ],
        };

        assert_eq!(days_calendar, expected_calendar);
    }

    #[test]
    fn test_contains() {
        let mut days_calendar = create_unsorted_days_calendar();

        assert_eq!(days_calendar.contains(2020, Month::January, 1), true);
        assert_eq!(days_calendar.contains(2020, Month::January, 15), true);
        assert_eq!(days_calendar.contains(2020, Month::January, 31), true);

        assert_eq!(days_calendar.contains(2020, Month::February, 1), true);
        assert_eq!(days_calendar.contains(2020, Month::February, 14), true);
        assert_eq!(days_calendar.contains(2020, Month::February, 29), true);

        assert_eq!(days_calendar.contains(2020, Month::March, 1), true);
        assert_eq!(days_calendar.contains(2020, Month::March, 3), true);
        assert_eq!(days_calendar.contains(2020, Month::March, 5), true);

        assert_eq!(days_calendar.contains(2020, Month::January, 2), false);
        assert_eq!(days_calendar.contains(2020, Month::February, 2), false);
        assert_eq!(days_calendar.contains(2020, Month::March, 2), false);
    }

    fn sample_calendar() -> DaysCalendar<BiDay> {
        DaysCalendar {
            days_calendar: vec![
                (
                    2020,
                    Month::January,
                    vec![
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::One,
                    ],
                ),
                (
                    2020,
                    Month::February,
                    vec![
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::One,
                    ],
                ),
                (
                    2020,
                    Month::March,
                    vec![
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::One,
                    ],
                ),
            ],
        }
    }

    #[test]
    fn test_next_day() {
        let calendar = sample_calendar();
        let date = Date::from_calendar_date(2020, Month::January.to_time_month().unwrap(), 1).unwrap();
        assert_eq!(
            calendar.next_day(2020, Month::January, 1),
            Some(date!(2020 - 01 - 15))
        );
        assert_eq!(
            calendar.next_day(2020, Month::January, 15),
            Some(date!(2020 - 01 - 30))
        );
        assert_eq!(
            calendar.next_day(2020, Month::February, 14),
            Some(date!(2020 - 02 - 28))
        );
        assert_eq!(
            calendar.next_day(2020, Month::February, 28),
            Some(date!(2020 - 03 - 01))
        );
        assert_eq!(
            calendar.next_day(2020, Month::March, 1),
            Some(date!(2020 - 03 - 30))
        );
    }

    #[test]
    fn test_next_day_edge_cases() {
        let calendar = sample_calendar();

        // No next day available
        assert_eq!(calendar.next_day(2020, Month::March, 30), None);

        // Invalid date input
        assert_eq!(
            calendar.next_day(2020, Month::January, 31),
            Some(date!(2020 - 02 - 14))
        );
        assert_eq!(
            calendar.next_day(2020, Month::February, 28),
            Some(date!(2020 - 03 - 01))
        );
        assert_eq!(calendar.next_day(2020, Month::April, 31), None);

        // Test with leap year
        let mut leap_year_calendar = calendar.clone();
        leap_year_calendar
            .days_calendar
            .push((2020, Month::February, vec![BiDay::One]));
        let leap_date =
            Date::from_calendar_date(2020, Month::February.to_time_month().unwrap(), 29).unwrap();
        assert_eq!(
            leap_year_calendar.next_day(2020, Month::February, 28),
            Some(date!(2020 - 03 - 01))
        );

        // Test with non-leap year
        let non_leap_date =
            Date::from_calendar_date(2019, Month::February.to_time_month().unwrap(), 28).unwrap();
        assert_eq!(
            calendar.next_day(2019, Month::February, 28),
            Some(date!(2020 - 01 - 01))
        );
    }

    fn create_test_calendar() -> DaysCalendar<BiDay> {
        DaysCalendar {
            days_calendar: vec![
                (
                    2020,
                    Month::January,
                    vec![
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::Zero,
                    ],
                ),
                (
                    2020,
                    Month::February,
                    vec![
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::One,
                    ],
                ),
                (
                    2020,
                    Month::March,
                    vec![
                        BiDay::One,
                        BiDay::Zero,
                        BiDay::Zero,
                        BiDay::One,
                        BiDay::Zero,
                    ],
                ),
            ],
        }
    }

    #[test]
    fn test_previous_day_basic() {
        let calendar = create_test_calendar();
        let prev_day = calendar.previous_day(2020, Month::January, 3);
        assert_eq!(prev_day, Some(date!(2020 - 01 - 01)));

        let prev_day = calendar.previous_day(2020, Month::February, 5);
        assert_eq!(prev_day, Some(date!(2020 - 02 - 03)));

        let prev_day = calendar.previous_day(2020, Month::March, 4);
        assert_eq!(prev_day, Some(date!(2020 - 03 - 01)));
    }

    #[test]
    fn test_previous_day_not_found() {
        let calendar = create_test_calendar();
        let prev_day = calendar.previous_day(2020, Month::January, 1);
        assert_eq!(prev_day, None);
    }

    #[test]
    fn test_previous_day_edge_cases() {
        let calendar = create_test_calendar();

        let prev_day = calendar.previous_day(2020, Month::January, 31);
        assert_eq!(prev_day, Some(date!(2020 - 01 - 03)));

        let prev_day = calendar.previous_day(2020, Month::March, 31);
        assert_eq!(prev_day, Some(date!(2020 - 03 - 04)));
    }

    #[test]
    fn test_seek_nth_day_days() {
        let calendar = create_test_calendar();

        match calendar.seek_nth_day(2020, Month::January, 1, 2) {
            Ok(new_date) => assert_eq!(new_date, Some(date!(2020 - 02 - 03))),
            Err(_) => panic!("Error while seeking nth day"),
        }

        match calendar.seek_nth_day(2020, Month::February, 5, -1) {
            Ok(new_date) => assert_eq!(new_date, Some(date!(2020 - 02 - 03))),
            Err(_) => panic!("Error while seeking nth day"),
        }

        match calendar.seek_nth_day(2020, Month::March, 4, 0) {
            Ok(new_date) => assert_eq!(new_date, Some(date!(2020 - 03 - 04))),
            Err(_) => panic!("Error while seeking nth day"),
        }
    }


    #[test]
    fn test_seek_nth_day_not_found() {
        let calendar = create_test_calendar();

        let new_date = calendar.seek_nth_day(2020, Month::January, 1, -1);
        assert!(new_date.expect("Invalid").is_none());

        let new_date = calendar.seek_nth_day(2020, Month::March, 1, 5);
        assert!(new_date.is_ok());
    }

    fn generate_sample_calendar() -> DaysCalendar<BiDay> {
        DaysCalendar {
            days_calendar: vec![
                (2020, Month::January, vec![BiDay::One; 31]),
                (2020, Month::February, vec![BiDay::One; 29]),
            ],
        }
    }

    #[test]
    fn test_or_weekdays_empty_input() {
        let calendar = generate_sample_calendar();
        let weekdays = HashSet::new();
        let result = calendar.or_weekdays(weekdays);

        assert_eq!(calendar, result.unwrap());
    }

    #[test]
    fn test_or_weekdays_single_weekday() {
        let calendar = generate_sample_calendar();
        let mut weekdays = HashSet::new();
        weekdays.insert(Weekday::Monday);

        let result = calendar.or_weekdays(weekdays.clone()).unwrap();

        for (year, month, days) in result.days_calendar {
            for (day_index, bit) in days.iter().enumerate() {
                let day_num = day_index as u8 + 1;
                let date =
                    Date::from_calendar_date(year.into(), month.to_time_month().unwrap(), day_num).unwrap();
                let weekday = date.weekday();

                if weekdays.contains(&weekday) {
                    assert_eq!(*bit, BiDay::One);
                } else {
                    assert_eq!(*bit, BiDay::One);
                }
            }
        }
    }

    #[test]
    fn test_or_weekdays_multiple_weekdays() {
        let calendar = generate_sample_calendar();
        let mut weekdays = HashSet::new();
        weekdays.insert(Weekday::Monday);
        weekdays.insert(Weekday::Wednesday);
        weekdays.insert(Weekday::Friday);

        let result = calendar.or_weekdays(weekdays.clone()).unwrap();

        for (year, month, days) in result.days_calendar {
            for (day_index, bit) in days.iter().enumerate() {
                let day_num = day_index as u8 + 1;
                let date =
                    Date::from_calendar_date(year.into(), month.to_time_month().unwrap(), day_num).unwrap();
                let weekday = date.weekday();

                if weekdays.contains(&weekday) {
                    assert_eq!(*bit, BiDay::One);
                } else {
                    assert_eq!(*bit, BiDay::One);
                }
            }
        }
    }

    #[test]
    fn test_or_iso_weeks_empty_input() {
        let calendar = generate_sample_calendar();
        let weeks = vec![];
        let result = calendar.or_iso_weeks(weeks).unwrap();

        assert_eq!(calendar, result);
    }

    #[test]
    fn test_or_iso_weeks_single_week() {
        let calendar = generate_sample_calendar();
        let weeks = vec![5];

        let result = calendar.or_iso_weeks(weeks.clone()).unwrap();

        for (year, month, days) in result.days_calendar {
            let first_day_of_month =
                Date::from_calendar_date(year.into(), month.to_time_month().unwrap(), 1).unwrap();
            let days_in_month = days.len();

            for day_index in 0..days_in_month {
                let date = first_day_of_month + Duration::days(day_index as i64);
                let week_number = date.iso_week() as u32;

                if weeks.contains(&week_number) {
                    assert_eq!(days[day_index], BiDay::One);
                } else {
                    assert_eq!(days[day_index], BiDay::One);
                }
            }
        }
    }

    #[test]
    fn test_or_iso_weeks_multiple_weeks() {
        let calendar = generate_sample_calendar();
        let weeks = vec![2, 5, 7];

        let result = calendar.or_iso_weeks(weeks.clone()).unwrap();

        for (year, month, days) in result.days_calendar {
            let first_day_of_month =
                Date::from_calendar_date(year.into(), month.to_time_month().unwrap(), 1).unwrap();
            let days_in_month = days.len();

            for day_index in 0..days_in_month {
                let date = first_day_of_month + Duration::days(day_index as i64);
                let week_number = date.iso_week() as u32;

                if weeks.contains(&week_number) {
                    assert_eq!(days[day_index], BiDay::One);
                } else {
                    assert_eq!(days[day_index], BiDay::One);
                }
            }
        }
    }

    #[test]
    fn test_or_iso_weeks_all_weeks() {
        let calendar = generate_sample_calendar();
        let weeks = (1..=53).collect::<Vec<u32>>();

        let result = calendar.or_iso_weeks(weeks).unwrap();

        for (year, month, days) in result.days_calendar {
            for (_, bit) in days.iter().enumerate() {
                assert_eq!(*bit, BiDay::One);
            }
        }
    }

    #[test]
    fn test_and_weekdays_empty_input() {
        let calendar = generate_sample_calendar();
        let weekdays: HashSet<Weekday> = HashSet::new();
        let result = calendar.and_weekdays(weekdays).unwrap();

        for (year, month, days) in result.days_calendar {
            for (_, bit) in days.iter().enumerate() {
                assert_eq!(*bit, BiDay::Zero);
            }
        }
    }

    #[test]
    fn test_and_weekdays_single_weekday() {
        let calendar = generate_sample_calendar();
        let mut weekdays = HashSet::new();
        weekdays.insert(Weekday::Saturday);

        let result = calendar.and_weekdays(weekdays.clone()).unwrap();

        for (year, month, days) in result.days_calendar {
            let mut day_num = 1;
            for (_, bit) in days.iter().enumerate() {
                let date =
                    Date::from_calendar_date(year.into(), month.to_time_month().unwrap(), day_num).unwrap();
                let weekday = date.weekday();
                day_num += 1;

                if weekdays.contains(&weekday) {
                    assert_eq!(*bit, BiDay::One);
                } else {
                    assert_eq!(*bit, BiDay::Zero);
                }
            }
        }
    }

    #[test]
    fn test_and_weekdays_multiple_weekdays() {
        let calendar = generate_sample_calendar();
        let mut weekdays = HashSet::new();
        weekdays.insert(Weekday::Tuesday);
        weekdays.insert(Weekday::Thursday);
        weekdays.insert(Weekday::Sunday);

        let result = calendar.and_weekdays(weekdays.clone()).unwrap();

        for (year, month, days) in result.days_calendar {
            let mut day_num = 1;
            for (_, bit) in days.iter().enumerate() {
                let date =
                    Date::from_calendar_date(year.into(), month.to_time_month().unwrap(), day_num).unwrap();
                let weekday = date.weekday();
                day_num += 1;

                if weekdays.contains(&weekday) {
                    assert_eq!(*bit, BiDay::One);
                } else {
                    assert_eq!(*bit, BiDay::Zero);
                }
            }
        }
    }

    fn create_sample_calendar() -> DaysCalendar<BiDay> {
        let year = 2023;
        let month = Month::March;
        let days = vec![
            BiDay::One,
            BiDay::Zero,
            BiDay::One,
            BiDay::Zero,
            BiDay::One,
            BiDay::Zero,
            BiDay::One,
            BiDay::Zero,
            BiDay::One,
            BiDay::Zero,
            BiDay::One,
            BiDay::Zero,
            BiDay::One,
            BiDay::Zero,
            BiDay::One,
            BiDay::Zero,
            BiDay::One,
            BiDay::Zero,
            BiDay::One,
            BiDay::Zero,
            BiDay::One,
            BiDay::Zero,
            BiDay::One,
            BiDay::Zero,
            BiDay::One,
            BiDay::Zero,
            BiDay::One,
            BiDay::Zero,
            BiDay::One,
            BiDay::Zero,
            BiDay::One,
        ];
        DaysCalendar {
            days_calendar: vec![(year, month, days)],
        }
    }

    #[test]
    fn test_not_weekdays() {
        let calendar = create_sample_calendar();
        let mut weekdays = HashSet::new();
        weekdays.insert(Weekday::Monday);
        weekdays.insert(Weekday::Saturday);
        weekdays.insert(Weekday::Friday);

        let new_calendar = calendar.not_weekdays(weekdays).unwrap();
        let expected_days = vec![
            BiDay::One,
            BiDay::Zero,
            BiDay::Zero,
            BiDay::Zero,
            BiDay::One,
            BiDay::Zero,
            BiDay::One,
            BiDay::Zero,
            BiDay::One,
            BiDay::Zero,
            BiDay::Zero,
            BiDay::Zero,
            BiDay::Zero,
            BiDay::Zero,
            BiDay::One,
            BiDay::Zero,
            BiDay::Zero,
            BiDay::Zero,
            BiDay::One,
            BiDay::Zero,
            BiDay::One,
            BiDay::Zero,
            BiDay::One,
            BiDay::Zero,
            BiDay::Zero,
            BiDay::Zero,
            BiDay::Zero,
            BiDay::Zero,
            BiDay::One,
            BiDay::Zero,
            BiDay::Zero,
        ];

        assert_eq!(new_calendar.days_calendar[0].2, expected_days);
    }

    #[test]
    fn test_not_weekdays_empty_set() {
        let calendar = create_sample_calendar();
        let weekdays = HashSet::new();

        let new_calendar = calendar.not_weekdays(weekdays);

        assert_eq!(new_calendar, Ok(calendar));
    }

    use std::iter::repeat;

    #[test]
    fn test_not_iso_weeks() {
        let calendar = create_sample_calendar();
        let weeks_to_exclude = vec![1, 5, 52];

        let calendar = DaysCalendar {
            days_calendar: vec![
                (2023, Month::January, vec![BiDay::One; 31]),
                (2023, Month::December, vec![BiDay::One; 31]),
            ],
        };

        let filtered_calendar = calendar.not_iso_weeks(weeks_to_exclude.clone()).unwrap();

        for (year, month, days) in filtered_calendar.days_calendar {
            let first_day_of_month =
                Date::from_calendar_date(year as i32, month.to_time_month().unwrap(), 1).unwrap();
            let days_in_month = days.len();
            for day_index in 0..days_in_month {
                let date = first_day_of_month + Duration::days(day_index as i64);
                let week_number = date.iso_week() as u32;

                if weeks_to_exclude.contains(&week_number) {
                    assert_eq!(days[day_index], BiDay::Zero);
                } else {
                    assert_eq!(days[day_index], BiDay::One);
                }
            }
        }
    }

    #[test]
    fn test_group_days_calendar_single_entry() {
        let days: Vec<(Year, Month, Vec<BiDay>)> = vec![(
            2022,
            Month::February,
            vec![BiDay::One, BiDay::Zero, BiDay::Zero, BiDay::One],
        )];
        let result = group_days_calendar(DaysCalendar {
            days_calendar: days,
        });
        let expected: Vec<(Year, Month, Vec<Vec<BiDay>>)> = vec![(
            2022,
            Month::February,
            vec![vec![BiDay::One, BiDay::Zero, BiDay::Zero, BiDay::One]],
        )];
        assert_eq!(result, expected);
    }


    #[test]
    fn test_biday_to_vec_day_single_month() {
        let year: Year = 2022;
        let month: Month = Month::January;
        let days = vec![
            BiDay::Zero,
            BiDay::One,
            BiDay::Zero,
            BiDay::One,
            BiDay::Zero,
            BiDay::One,
            BiDay::Zero,
        ];
        let calendar: DaysCalendar<BiDay> = DaysCalendar::<BiDay>::new(vec![(year, month, days)]);
        let expected_days = vec![(2022 as Year, Month::January as Month, vec![2, 4, 6])];
        let actual_days = biday_to_vec_day(calendar);
        assert_eq!(actual_days, expected_days);
    }

    #[test]
    fn test_biday_to_vec_day_multiple_months() {
        let year = 2022;
        let month1 = Month::January;
        let days1 = vec![
            BiDay::Zero,
            BiDay::One,
            BiDay::Zero,
            BiDay::One,
            BiDay::Zero,
            BiDay::One,
            BiDay::Zero,
        ];
        let calendar1 = DaysCalendar::<BiDay>::new(vec![(year, month1, days1)]);

        let month2 = Month::March;
        let days2 = vec![BiDay::One, BiDay::Zero, BiDay::One, BiDay::Zero, BiDay::One];
        let calendar2 = DaysCalendar::<BiDay>::new(vec![(year, month2, days2)]);

        let extended_calendar = calendar1.append(&calendar2);

        let expected_days = vec![
            (2022 as Year, Month::January as Month, vec![2, 4, 6]),
            (2022 as Year, Month::March as Month, vec![1, 3, 5]),
        ];
        let actual_days = biday_to_vec_day(extended_calendar);
        assert_eq!(actual_days, expected_days);
    }

    #[test]
    fn test_group_days_calendar_empty() {
        let days: Vec<(Year, Month, Vec<BiDay>)> = vec![];
        let result = group_days_calendar(DaysCalendar {
            days_calendar: days,
        });
        let expected: Vec<(Year, Month, Vec<Vec<BiDay>>)> = vec![];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_group_days_calendar_multiple_entries_same_month() {
        let days: Vec<(Year, Month, Vec<BiDay>)> = vec![
            (
                2022,
                Month::January,
                vec![BiDay::One, BiDay::Zero, BiDay::One, BiDay::Zero],
            ),
            (
                2022,
                Month::January,
                vec![BiDay::Zero, BiDay::Zero, BiDay::One, BiDay::One],
            ),
        ];
        let result = group_days_calendar(DaysCalendar {
            days_calendar: days,
        });
        let expected: Vec<(Year, Month, Vec<Vec<BiDay>>)> = vec![(
            2022,
            Month::January,
            vec![
                vec![BiDay::One, BiDay::Zero, BiDay::One, BiDay::Zero],
                vec![BiDay::Zero, BiDay::Zero, BiDay::One, BiDay::One],
            ],
        )];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_group_days_calendar_multiple_entries_different_months() {
        let days: Vec<(Year, Month, Vec<BiDay>)> = vec![
            (
                2022,
                Month::February,
                vec![BiDay::One, BiDay::Zero, BiDay::Zero, BiDay::One],
            ),
            (
                2022,
                Month::January,
                vec![BiDay::One, BiDay::Zero, BiDay::One, BiDay::Zero],
            ),
            (
                2022,
                Month::January,
                vec![BiDay::Zero, BiDay::Zero, BiDay::One, BiDay::One],
            ),
        ];
        let result = group_days_calendar(DaysCalendar {
            days_calendar: days,
        });
        let expected: Vec<(Year, Month, Vec<Vec<BiDay>>)> = vec![
            (
                2022,
                Month::January,
                vec![
                    vec![BiDay::One, BiDay::Zero, BiDay::One, BiDay::Zero],
                    vec![BiDay::Zero, BiDay::Zero, BiDay::One, BiDay::One],
                ],
            ),
            (
                2022,
                Month::February,
                vec![vec![BiDay::One, BiDay::Zero, BiDay::Zero, BiDay::One]],
            ),
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_days_present() {
        let calendar = DaysCalendar {
            days_calendar: vec![
                (2023, Month::January, vec![BiDay::One, BiDay::Zero, BiDay::One]),
                (2023, Month::February, vec![BiDay::Zero, BiDay::Zero]),
            ],
        };

        let days = calendar.get_days(2023, Month::January);
        assert_eq!(days, Some(&vec![BiDay::One, BiDay::Zero, BiDay::One]));
    }

    #[test]
    fn test_get_days_absent() {
        let calendar = DaysCalendar {
            days_calendar: vec![
                (2023, Month::January, vec![BiDay::One, BiDay::Zero, BiDay::One]),
            ],
        };

        let days = calendar.get_days(2023, Month::February);
        assert_eq!(days, None);
    }

    #[test]
    fn test_get_days_data_integrity() {
        let calendar = DaysCalendar {
            days_calendar: vec![
                (2023, Month::January, vec![BiDay::One, BiDay::Zero, BiDay::One]),
                (2023, Month::February, vec![BiDay::Zero, BiDay::Zero]),
            ],
        };

        let days_jan = calendar.get_days(2023, Month::January).unwrap();
        assert_eq!(days_jan.len(), 3);
        assert_eq!(days_jan[0], BiDay::One);
        assert_eq!(days_jan[1], BiDay::Zero);
        assert_eq!(days_jan[2], BiDay::One);

        let days_feb = calendar.get_days(2023, Month::February).unwrap();
        assert_eq!(days_feb.len(), 2);
        assert_eq!(days_feb[0], BiDay::Zero);
        assert_eq!(days_feb[1], BiDay::Zero);
    }



}