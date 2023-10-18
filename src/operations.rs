/// The `utils` module contains simple but general functions that can support the development of dayendar business logic. 
pub mod utils {

    use crate::types::{Month, Year, BiDay};

    /// `is_leap` Determines whether a year is a leap year
    pub fn is_leap(year: Year) -> bool {
        (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
    }

    /// Gets the number of days in a given month
    pub fn days_in_month(year: Year, month: Month) -> Option<u8> {
        if month < Month::January || month > Month::December {
            return None;
        }
    
        // For year range between 1 and 9999
        if !(1..=9999).contains(&year) {
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

    /// Creates a vector of given size filled with a BiDay value
    pub fn generate_vec_days(size: usize, value: BiDay) -> Vec<BiDay> {
        let mut vec = Vec::with_capacity(size);
        vec.resize(size, value);
        vec
    }
}

/// The `binary` module provides functionality for binary calendar operations.
///
/// It provides elementary functions and operators to simplify and streamline 
/// operations between one or more calendars.
///
pub mod binary {

    use crate::utils::days_in_month;
    use crate::types::{
        Year, Month, BiDay,
        Date, Duration, MIN_DATE, MAX_DATE
    };

    /// Enumeration representing operators for calendars
    #[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
    pub enum DaysCalendarOperator {
        DcOr,
        DcAnd,
        DcAdd,
        DcSustract,
        DcMatchAny,
    }

    /// Applies an operator F on two BiDay types
    #[allow(dead_code)]
    pub fn apply_operator<F>(a: &[BiDay], b: &[BiDay], operator: F) -> Vec<BiDay>
    where F: Fn(BiDay, BiDay) -> BiDay
    {
        let mut result = Vec::with_capacity(a.len());
        result.extend(a.iter().zip(b.iter()).map(|(&a, &b)| operator(a, b)));
        result
    }

    /// OR operator between two BiDay types
    pub fn or_biday_operation(a: BiDay, b: BiDay) -> BiDay {
        match (a, b) {
            (BiDay::One, _) | (_, BiDay::One) => BiDay::One,
            (BiDay::Zero, BiDay::Zero)        => BiDay::Zero,
            //_                                 => BiDay::Zero,
        }
    }

    /// OR operator between two BiDay vectors
    #[allow(dead_code)]
    pub fn or_daycalendar(v1: &[BiDay], v2: &[BiDay]) -> Vec<BiDay> {
        apply_operator(v1, v2, or_biday_operation)
    }

    /// AND operator between two BiDays
    pub fn and_biday_operation(a: BiDay, b: BiDay) -> BiDay {
        match (a, b) {
            (BiDay::One, BiDay::One)    => BiDay::One,
            _                           => BiDay::Zero,
        }
    }

    /// AND operator between two BiDay vectors
    #[allow(dead_code)]
    pub fn and_daycalendar(v1: &[BiDay], v2: &[BiDay]) -> Vec<BiDay> {
        apply_operator(v1, v2, and_biday_operation)
    }

    /// Subtraction operator between two BiDays
    pub fn minus_biday_operation(a: BiDay, b: BiDay) -> BiDay {
        match (a, b) {
            (BiDay::One, BiDay::One)    => BiDay::Zero,
            (BiDay::One, BiDay::Zero)   => BiDay::One,
            (BiDay::Zero, _)            => BiDay::Zero,
            //_                           => BiDay::Zero,
        }
    }

    /// Subtract two vector of BiDay
    #[allow(dead_code)]
    pub(crate) fn minus_daycalendar(v1: &[BiDay], v2: &[BiDay]) -> Vec<BiDay> {
        apply_operator(v1, v2, minus_biday_operation)
    }

    /// Addition operator between two BiDays
    pub fn add_biday_operation(a: BiDay, b: BiDay) -> BiDay {
        match (a, b) {
            (BiDay::One, BiDay::One)    => BiDay::One,
            (BiDay::One, BiDay::Zero)   => BiDay::One,
            (BiDay::Zero, BiDay::One)   => BiDay::One,
            _                           => BiDay::Zero,
        }
    }

    /// Adds two vector of BiDay
    #[allow(dead_code)]
    pub(crate) fn add_daycalendar(v1: &[BiDay], v2: &[BiDay]) -> Vec<BiDay> {
        apply_operator(v1, v2, add_biday_operation)
    }

    /// Match operator between two BiDays
    pub fn match_biday_operation(a: BiDay, b: BiDay) -> BiDay {
        match (a, b) {
            (BiDay::One, BiDay::One)    => BiDay::One,
            (BiDay::Zero, BiDay::Zero)  => BiDay::One,
            _                           => BiDay::Zero,
        }
    }

    /// Find matches between two vector of BiDay
    #[allow(dead_code)]
    pub fn match_daycalendar(v1: &[BiDay], v2: &[BiDay]) -> Vec<BiDay> {
        apply_operator(v1, v2, match_biday_operation)
    }

    /// Non-match operator between two BiDays
    pub fn nomatch_biday_operation(a: BiDay, b: BiDay) -> BiDay {
        match (a, b) {
            (BiDay::One, BiDay::One)    => BiDay::Zero,
            (BiDay::Zero, BiDay::Zero)  => BiDay::Zero,
            _                           => BiDay::One,
        }
    }

    /// Finds non-matches between two vectors of BiDay
    #[allow(dead_code)]
    pub(crate) fn nomatch_daycalendar(v1: &[BiDay], v2: &[BiDay]) -> Vec<BiDay> {
        apply_operator(v1, v2, nomatch_biday_operation)
    }

    /// Finds "gaps" between two BiDays
    pub fn holes_biday_operation(a: BiDay, b: BiDay) -> BiDay {
        match (a, b) {
            (BiDay::Zero, BiDay::Zero)    => BiDay::One,
            _                             => BiDay::Zero,
        }
    }

    /// Find "gaps" between two calendars
    #[allow(dead_code)]
    pub(crate) fn holes_daycalendar(v1: &[BiDay], v2: &[BiDay]) -> Vec<BiDay> {
        apply_operator(v1, v2, holes_biday_operation)
    }


    use std::vec::Vec;

    /* "fully_" Complete a BiDay vector by incorporating the BiDay value until the n elements after the last value are completed
        -- 'n' is the number of elements
        -- 'k' is the value of the new BiDay elements
        -- 'v' is the vector of BiDay
    */
    /// Complete a BiDay vector by incorporating the BiDay value until the n elements after the last value are completed
    pub(crate) fn complete_biday(n: usize, k: BiDay, mut v: Vec<BiDay>) -> Vec<BiDay> {
        if n >= v.len() {
            let ni = n - v.len();
            if ni == 0 {
                v
            } else {
                let mut new_v = Vec::with_capacity(n);
                new_v.extend(v);
                new_v.resize(n, k);
                new_v
            }
        } else {
            v.truncate(n);
            v
        }
    }

    /// Normalizes a BiDay vector to a specific month and year
    pub fn normalize_biday(days: &[BiDay], year: Year, month: Month) -> Vec<BiDay> {
        let n = days_in_month(year, month).unwrap().into();
        if days.len() >= n {
            days[..n].to_vec()
        } else {
            complete_biday(n, BiDay::Zero, days.to_vec())
        }
    }

    /// Replicates a BiDay pattern n times
    pub(crate) fn replicate_pattern(pattern: &[BiDay], n: usize) -> Vec<BiDay> {
        let mut result = Vec::with_capacity(n);
        let pattern_len = pattern.len();
    
        for i in 0..n {
            result.push(pattern[i % pattern_len]);
        }
    
        result
    }
    
    

    /// Adds or subtracts days to a date vector
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


    #[cfg(test)]
    mod tests_binary_ops {
        
        use crate::types::*;
        use crate::utils::*;
        use crate::binary::*;

        #[test]
        fn test_is_leap() {
            assert_eq!(is_leap(2000), true);
            assert_eq!(is_leap(1900), false);
            assert_eq!(is_leap(2004), true);
            assert_eq!(is_leap(2005), false);
        }

        #[test]
        fn test_days_in_month() {
            assert_eq!(days_in_month(2000, Month::February), Some(29));
            assert_eq!(days_in_month(1900, Month::February), Some(28));
            assert_eq!(days_in_month(2004, Month::April), Some(30));
            assert_eq!(days_in_month(2005, Month::January), Some(31));
            assert_eq!(days_in_month(10000, Month::January), None);
            assert_eq!(days_in_month(2023, Month::January).unwrap(), 31);
            assert_eq!(days_in_month(2023, Month::February).unwrap(), 28);
            assert_eq!(days_in_month(2020, Month::February).unwrap(), 29);
            assert_eq!(days_in_month(2024, Month::February).unwrap(), 29);
            assert_eq!(days_in_month(2023, Month::April).unwrap(), 30);
            assert_eq!(days_in_month(2023, Month::December).unwrap(), 31);
            assert_eq!(days_in_month(2023, Month::May).unwrap(), 31);
            assert_eq!(days_in_month(2025, Month::August).unwrap(), 31);
            assert_eq!(days_in_month(2025, Month::February).unwrap(), 28);
        }

        #[test]
        fn test_generate_vec_days() {
            assert_eq!(generate_vec_days(3, BiDay::One), vec![BiDay::One, BiDay::One, BiDay::One]);
            assert_eq!(generate_vec_days(0, BiDay::One), vec![]);
        }

        #[test]
        fn test_or_biday_operation() {
            assert_eq!(or_biday_operation(BiDay::Zero, BiDay::Zero), BiDay::Zero);
            assert_eq!(or_biday_operation(BiDay::One, BiDay::Zero), BiDay::One);
            assert_eq!(or_biday_operation(BiDay::Zero, BiDay::One), BiDay::One);
            assert_eq!(or_biday_operation(BiDay::One, BiDay::One), BiDay::One);
        }

        #[test]
        fn test_and_biday_operation() {
            assert_eq!(and_biday_operation(BiDay::Zero, BiDay::Zero), BiDay::Zero);
            assert_eq!(and_biday_operation(BiDay::One, BiDay::Zero), BiDay::Zero);
            assert_eq!(and_biday_operation(BiDay::Zero, BiDay::One), BiDay::Zero);
            assert_eq!(and_biday_operation(BiDay::One, BiDay::One), BiDay::One);
        }

        #[test]
        fn test_minus_biday_operation() {
            assert_eq!(minus_biday_operation(BiDay::One, BiDay::One), BiDay::Zero);
            assert_eq!(minus_biday_operation(BiDay::One, BiDay::Zero), BiDay::One);
            assert_eq!(minus_biday_operation(BiDay::Zero, BiDay::One), BiDay::Zero);
            assert_eq!(minus_biday_operation(BiDay::Zero, BiDay::Zero), BiDay::Zero);
        }

        #[test]
        fn test_add_biday_operation() {
            assert_eq!(add_biday_operation(BiDay::One, BiDay::One), BiDay::One);
            assert_eq!(add_biday_operation(BiDay::One, BiDay::Zero), BiDay::One);
            assert_eq!(add_biday_operation(BiDay::Zero, BiDay::One), BiDay::One);
            assert_eq!(add_biday_operation(BiDay::Zero, BiDay::Zero), BiDay::Zero);
        }

        #[test]
        fn test_match_biday_operation() {
            assert_eq!(match_biday_operation(BiDay::One, BiDay::One), BiDay::One);
            assert_eq!(match_biday_operation(BiDay::Zero, BiDay::Zero), BiDay::One);
            assert_eq!(match_biday_operation(BiDay::One, BiDay::Zero), BiDay::Zero);
            assert_eq!(match_biday_operation(BiDay::Zero, BiDay::One), BiDay::Zero);
        }

        #[test]
        fn test_nomatch_biday_operation() {
            assert_eq!(nomatch_biday_operation(BiDay::One, BiDay::One), BiDay::Zero);
            assert_eq!(nomatch_biday_operation(BiDay::Zero, BiDay::Zero), BiDay::Zero);
            assert_eq!(nomatch_biday_operation(BiDay::One, BiDay::Zero), BiDay::One);
            assert_eq!(nomatch_biday_operation(BiDay::Zero, BiDay::One), BiDay::One);
        }

        #[test]
        fn test_complete_biday() {
            assert_eq!(complete_biday(5, BiDay::Zero, vec![BiDay::One, BiDay::One]), vec![BiDay::One, BiDay::One, BiDay::Zero, BiDay::Zero, BiDay::Zero]);
            assert_eq!(complete_biday(2, BiDay::Zero, vec![BiDay::One, BiDay::One, BiDay::Zero]), vec![BiDay::One, BiDay::One]);
        }

        #[test]
        fn test_normalize_biday_short() {
            let result = normalize_biday(&vec![BiDay::One, BiDay::Zero], 2000, Month::February);
            assert_eq!(result.len(), 29);
            assert_eq!(result[0], BiDay::One);
            assert_eq!(result[1], BiDay::Zero);
            assert_eq!(result[28], BiDay::Zero);
        }

        #[test]
        fn test_replicate_pattern() {
            let pattern = vec![BiDay::One, BiDay::Zero];
            let result = replicate_pattern(&pattern, 5);
            assert_eq!(result, vec![BiDay::One, BiDay::Zero, BiDay::One, BiDay::Zero, BiDay::One]);
        }

        #[test]
        fn test_add_days() {
            let dates = vec![date!(2022 - 01 - 01)];
            let result = add_days(dates.clone(), 30);
            assert_eq!(result, vec![date!(2022 - 01 - 31)]);

            let result_neg = add_days(dates, -30);
            assert_eq!(result_neg, vec![date!(2021 - 12 - 02)]);
        }

        #[test]
        fn test_add_daycalendar() {
            let v1 = vec![BiDay::Zero, BiDay::One, BiDay::Zero, BiDay::One];
            let v2 = vec![BiDay::One, BiDay::One, BiDay::Zero, BiDay::Zero];
            let expected_result = vec![BiDay::One, BiDay::One, BiDay::Zero, BiDay::One];
            assert_eq!(add_daycalendar(&v1, &v2), expected_result);

            let v3 = vec![
                BiDay::Zero,
                BiDay::Zero,
                BiDay::One,
                BiDay::One,
                BiDay::Zero,
                BiDay::Zero,
            ];
            let v4 = vec![
                BiDay::One,
                BiDay::One,
                BiDay::Zero,
                BiDay::Zero,
                BiDay::One,
                BiDay::One,
            ];
            let expected_result_2 = vec![
                BiDay::One,
                BiDay::One,
                BiDay::One,
                BiDay::One,
                BiDay::One,
                BiDay::One,
            ];
            assert_eq!(add_daycalendar(&v3, &v4), expected_result_2);

            let v5 = vec![BiDay::One, BiDay::One, BiDay::Zero, BiDay::Zero];
            let v6 = vec![BiDay::Zero, BiDay::Zero, BiDay::One, BiDay::One];
            let expected_result_3 = vec![BiDay::One, BiDay::One, BiDay::One, BiDay::One];
            assert_eq!(add_daycalendar(&v5, &v6), expected_result_3);

            let v7 = vec![BiDay::Zero, BiDay::One, BiDay::One, BiDay::Zero];
            let v8 = vec![BiDay::One, BiDay::Zero, BiDay::One, BiDay::Zero];
            let expected_result_4 = vec![BiDay::One, BiDay::One, BiDay::One, BiDay::Zero];
            assert_eq!(add_daycalendar(&v7, &v8), expected_result_4);
        }

        #[test]
        fn test_match_daycalendar() {
            let v1 = vec![BiDay::Zero, BiDay::One, BiDay::One, BiDay::Zero];
            let v2 = vec![BiDay::One, BiDay::Zero, BiDay::One, BiDay::Zero];
            let result = match_daycalendar(&v1, &v2);
            assert_eq!(
                result,
                vec![BiDay::Zero, BiDay::Zero, BiDay::One, BiDay::One]
            );

            let v1 = vec![BiDay::One, BiDay::One, BiDay::One, BiDay::One];
            let v2 = vec![BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero];
            let result = match_daycalendar(&v1, &v2);
            assert_eq!(
                result,
                vec![BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero]
            );

            let v1 = vec![BiDay::One, BiDay::One, BiDay::Zero, BiDay::Zero];
            let v2 = vec![BiDay::Zero, BiDay::One, BiDay::One, BiDay::Zero];
            let result = match_daycalendar(&v1, &v2);
            assert_eq!(
                result,
                vec![BiDay::Zero, BiDay::One, BiDay::Zero, BiDay::One]
            );
        }

        #[test]
        fn test_nomatch_daycalendar() {
            let v1 = vec![BiDay::Zero, BiDay::One, BiDay::Zero, BiDay::One];
            let v2 = vec![BiDay::One, BiDay::Zero, BiDay::Zero, BiDay::One];
            let expected = vec![BiDay::One, BiDay::One, BiDay::Zero, BiDay::Zero];
            assert_eq!(nomatch_daycalendar(&v1, &v2), expected);

            let v1 = vec![BiDay::One, BiDay::One, BiDay::Zero, BiDay::Zero];
            let v2 = vec![BiDay::One, BiDay::One, BiDay::Zero, BiDay::Zero];
            let expected = vec![BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero];
            assert_eq!(nomatch_daycalendar(&v1, &v2), expected);

            let v1 = vec![BiDay::One, BiDay::One, BiDay::One, BiDay::One];
            let v2 = vec![BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero];
            let expected = vec![BiDay::One, BiDay::One, BiDay::One, BiDay::One];
            assert_eq!(nomatch_daycalendar(&v1, &v2), expected);

            let v1 = vec![BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero];
            let v2 = vec![BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero];
            let expected = vec![BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero];
            assert_eq!(nomatch_daycalendar(&v1, &v2), expected);
        }

        #[test]
        fn test_complete_biday_empty_vector() {
            let v: Vec<BiDay> = Vec::new();
            let v_expected = vec![
                BiDay::Zero,
                BiDay::Zero,
                BiDay::Zero,
                BiDay::Zero,
                BiDay::Zero,
            ];
            assert_eq!(complete_biday(5, BiDay::Zero, v), v_expected);
        }

        #[test]
        fn test_complete_biday_n_greater_than_v() {
            let v = vec![BiDay::One, BiDay::Zero, BiDay::One];
            let v_expected = vec![
                BiDay::One,
                BiDay::Zero,
                BiDay::One,
                BiDay::Zero,
                BiDay::Zero,
                BiDay::Zero,
            ];
            assert_eq!(complete_biday(6, BiDay::Zero, v), v_expected);
        }

        #[test]
        fn test_complete_biday_n_smaller_than_v() {
            let v = vec![
                BiDay::Zero,
                BiDay::One,
                BiDay::Zero,
                BiDay::One,
                BiDay::Zero,
            ];
            let v_expected = vec![BiDay::Zero, BiDay::One, BiDay::Zero];
            assert_eq!(complete_biday(3, BiDay::Zero, v), v_expected);
        }

        #[test]
        fn test_complete_biday_n_equal_to_v() {
            let v = vec![
                BiDay::Zero,
                BiDay::One,
                BiDay::Zero,
                BiDay::One,
                BiDay::Zero,
            ];
            let v_expected = v.clone();
            assert_eq!(complete_biday(5, BiDay::Zero, v), v_expected);
        }


        

        
    
    
    }



}