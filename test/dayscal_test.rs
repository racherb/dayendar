#[cfg(test)]
mod tests {
    //use super::*;
    use dayendar::calendar::*;
    use std::collections::HashMap;
    use std::vec::Vec;
    use time::Date;
    use time::Month as TimeMonth;
    use time::macros::date;
    use time::Weekday;
    use time::Duration;
    use std::iter::FromIterator;
    use std::collections::HashSet;

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
    fn test_days_in_month() {
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
        let dc:DaysCalendar<BiDay> = DaysCalendar { days_calendar: vec![] };
        let result = extract_year_month_calendar(&dc);
        let expected: Vec<(Year, Month)> = vec![];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_extract_year_month_calendar_single_month() {
        let dc = DaysCalendar { days_calendar: vec![(2022, Month::January, vec![1, 0, 1, 0, 1, 0, 1])] };
        let result = extract_year_month_calendar(&dc);
        let expected: Vec<(Year, Month)> = vec![(2022, Month::January)];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_extract_year_month_calendar_multiple_months() {
        let dc = DaysCalendar { days_calendar: vec![(2022, Month::January, vec![1, 0, 1, 0, 1, 0, 1]), (2023, Month::March, vec![0, 1, 0, 1, 0, 1, 0])] };
        let result = extract_year_month_calendar(&dc);
        let expected: Vec<(Year, Month)> = vec![(2022, Month::January), (2023, Month::March)];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_extract_year_month_calendar_duplicate_months() {
        let dc = DaysCalendar { days_calendar: vec![(2022, Month::January, vec![1, 0, 1, 0, 1, 0, 1]), (2022, Month::January, vec![0, 1, 0, 1, 0, 1, 0])] };
        let result = extract_year_month_calendar(&dc);
        let expected: Vec<(Year, Month)> = vec![(2022, Month::January)];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_extract_day_month_calendar_empty() {
        let dc:DaysCalendar<BiDay> = DaysCalendar { days_calendar: vec![] };
        let result = extract_day_month_calendar(2022, Month::January, &dc);
        let expected: Option<Vec<BiDay>> = None;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_extract_day_month_calendar_not_found() {
        let dc = DaysCalendar { 
            days_calendar: vec![
                (2022, Month::January, vec![BiDay::One, BiDay::Zero, BiDay::One, BiDay::Zero, BiDay::One, BiDay::Zero, BiDay::One]), 
                (2023, Month::March, vec![BiDay::Zero, BiDay::One, BiDay::Zero, BiDay::One, BiDay::Zero, BiDay::One, BiDay::Zero])
            ] 
        };
        let result = extract_day_month_calendar(2022, Month::March, &dc);
        let expected: Option<Vec<BiDay>> = None;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_extract_day_month_calendar_single() {
        let dc = DaysCalendar { 
            days_calendar: vec![
                (2022, Month::January, vec![BiDay::One, BiDay::Zero, BiDay::One, BiDay::Zero, BiDay::One, BiDay::Zero, BiDay::One]), 
                (2023, Month::March, vec![BiDay::Zero, BiDay::One, BiDay::Zero, BiDay::One, BiDay::Zero, BiDay::One, BiDay::Zero])
            ] 
        };
        let result = extract_day_month_calendar(2022, Month::January, &dc).unwrap();
        let expected: Vec<BiDay> = vec![BiDay::One, BiDay::Zero, BiDay::One, BiDay::Zero, BiDay::One, BiDay::Zero, BiDay::One];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_extract_day_month_calendar_multiple() {
        let dc = DaysCalendar { 
            days_calendar: vec![
                (2022, Month::January, vec![BiDay::One, BiDay::Zero, BiDay::One, BiDay::Zero, BiDay::One, BiDay::Zero, BiDay::One]), 
                (2022, Month::February, vec![BiDay::One, BiDay::One, BiDay::Zero, BiDay::Zero, BiDay::One, BiDay::Zero, BiDay::One]), 
                (2023, Month::March, vec![BiDay::Zero, BiDay::One, BiDay::Zero, BiDay::One, BiDay::Zero, BiDay::One, BiDay::Zero])
            ] 
        };
        let result = extract_day_month_calendar(2022, Month::February, &dc).unwrap();
        let expected: Vec<BiDay> = vec![BiDay::One, BiDay::One, BiDay::Zero, BiDay::Zero, BiDay::One, BiDay::Zero, BiDay::One];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_or_daycalendar() {
        let v1 = vec![BiDay::One, BiDay::Zero, BiDay::One, BiDay::Zero];
        let v2 = vec![BiDay::Zero, BiDay::One, BiDay::One, BiDay::Zero];
        let result = or_daycalendar(&v1, &v2);
        assert_eq!(result, vec![BiDay::One, BiDay::One, BiDay::One, BiDay::Zero]);

        let v1 = vec![BiDay::Zero, BiDay::Zero, BiDay::One, BiDay::One];
        let v2 = vec![BiDay::Zero, BiDay::One, BiDay::One, BiDay::Zero];
        let result = or_daycalendar(&v1, &v2);
        assert_eq!(result, vec![BiDay::Zero, BiDay::One, BiDay::One, BiDay::One]);

        let v1 = vec![BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero];
        let expected_result = vec![BiDay::Zero, BiDay::One, BiDay::One, BiDay::Zero];
        assert_eq!(or_daycalendar(&v1, &v2), expected_result);
        
        let v1 = vec![BiDay::Zero, BiDay::One, BiDay::Zero, BiDay::One, BiDay::Zero];
        let v2 = vec![BiDay::One, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::One];
        let expected_result = vec![BiDay::One, BiDay::One, BiDay::Zero, BiDay::One, BiDay::One];
        assert_eq!(or_daycalendar(&v1, &v2), expected_result);

        let v1 = vec![BiDay::One, BiDay::Zero, BiDay::One];
        let v2 = vec![BiDay::One, BiDay::One, BiDay::Zero];
        let expected_result = vec![BiDay::One, BiDay::One, BiDay::One];
        assert_eq!(or_daycalendar(&v1, &v2), expected_result);
    }

    #[test]
    fn test_minus_daycalendar() {
        let v1 = vec![BiDay::One, BiDay::One, BiDay::Zero, BiDay::Zero, BiDay::One, BiDay::Zero, BiDay::One, BiDay::Zero];
        let v2 = vec![BiDay::One, BiDay::Zero, BiDay::Zero, BiDay::One, BiDay::Zero, BiDay::Zero, BiDay::One, BiDay::One];
        let expected_output = vec![BiDay::Zero, BiDay::One, BiDay::Zero, BiDay::Zero, BiDay::One, BiDay::Zero, BiDay::Zero, BiDay::Zero];

        assert_eq!(minus_daycalendar(&v1, &v2), expected_output);
    }

    #[test]
    fn test_add_daycalendar() {
        let v1 = vec![BiDay::Zero, BiDay::One, BiDay::Zero, BiDay::One];
        let v2 = vec![BiDay::One, BiDay::One, BiDay::Zero, BiDay::Zero];
        let expected_result = vec![BiDay::One, BiDay::One, BiDay::Zero, BiDay::One];
        assert_eq!(add_daycalendar(&v1, &v2), expected_result);

        let v3 = vec![BiDay::Zero, BiDay::Zero, BiDay::One, BiDay::One, BiDay::Zero, BiDay::Zero];
        let v4 = vec![BiDay::One, BiDay::One, BiDay::Zero, BiDay::Zero, BiDay::One, BiDay::One];
        let expected_result_2 = vec![BiDay::One, BiDay::One, BiDay::One, BiDay::One, BiDay::One, BiDay::One];
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
        assert_eq!(result, vec![BiDay::Zero, BiDay::Zero, BiDay::One, BiDay::One]);

        let v1 = vec![BiDay::One, BiDay::One, BiDay::One, BiDay::One];
        let v2 = vec![BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero];
        let result = match_daycalendar(&v1, &v2);
        assert_eq!(result, vec![BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero]);

        let v1 = vec![BiDay::One, BiDay::One, BiDay::Zero, BiDay::Zero];
        let v2 = vec![BiDay::Zero, BiDay::One, BiDay::One, BiDay::Zero];
        let result = match_daycalendar(&v1, &v2);
        assert_eq!(result, vec![BiDay::Zero, BiDay::One, BiDay::Zero, BiDay::One]);
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
        let v:Vec<BiDay> = Vec::new();
        let v_expected = vec![BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero];
        assert_eq!(complete_biday(5, BiDay::Zero, v), v_expected);
    }

    #[test]
    fn test_complete_biday_n_greater_than_v() {
        let v = vec![BiDay::One, BiDay::Zero, BiDay::One];
        let v_expected = vec![BiDay::One, BiDay::Zero, BiDay::One, BiDay::Zero, BiDay::Zero, BiDay::Zero];
        assert_eq!(complete_biday(6, BiDay::Zero, v), v_expected);
    }

    #[test]
    fn test_complete_biday_n_smaller_than_v() {
        let v = vec![BiDay::Zero, BiDay::One, BiDay::Zero, BiDay::One, BiDay::Zero];
        let v_expected = vec![BiDay::Zero, BiDay::One, BiDay::Zero];
        assert_eq!(complete_biday(3, BiDay::Zero, v), v_expected);
    }

    #[test]
    fn test_complete_biday_n_equal_to_v() {
        let v = vec![BiDay::Zero, BiDay::One, BiDay::Zero, BiDay::One, BiDay::Zero];
        let v_expected = v.clone();
        assert_eq!(complete_biday(5, BiDay::Zero, v), v_expected);
    }

    #[test]
    fn test_group_days_calendar_empty() {
        let days: Vec<(Year, Month, Vec<BiDay>)> = vec![];
        let result = group_days_calendar(DaysCalendar { days_calendar: days });
        let expected: Vec<(Year, Month, Vec<Vec<BiDay>>)> = vec![];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_group_days_calendar_single_entry() {
        let days: Vec<(Year, Month, Vec<BiDay>)> = vec![
            (2022, Month::February, vec![BiDay::One, BiDay::Zero, BiDay::Zero, BiDay::One]),
        ];
        let result = group_days_calendar(DaysCalendar { days_calendar: days });
        let expected: Vec<(Year, Month, Vec<Vec<BiDay>>)> = vec![
            (2022, Month::February, vec![vec![BiDay::One, BiDay::Zero, BiDay::Zero, BiDay::One]]),
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_group_days_calendar_multiple_entries_same_month() {
        let days: Vec<(Year, Month, Vec<BiDay>)> = vec![
            (2022, Month::January, vec![BiDay::One, BiDay::Zero, BiDay::One, BiDay::Zero]),
            (2022, Month::January, vec![BiDay::Zero, BiDay::Zero, BiDay::One, BiDay::One]),
        ];
        let result = group_days_calendar(DaysCalendar { days_calendar: days });
        let expected: Vec<(Year, Month, Vec<Vec<BiDay>>)> = vec![
            (2022, Month::January, vec![vec![BiDay::One, BiDay::Zero, BiDay::One, BiDay::Zero], vec![BiDay::Zero, BiDay::Zero, BiDay::One, BiDay::One]]),
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_group_days_calendar_multiple_entries_different_months() {
        let days: Vec<(Year, Month, Vec<BiDay>)> = vec![
            (2022, Month::February, vec![BiDay::One, BiDay::Zero, BiDay::Zero, BiDay::One]),
            (2022, Month::January, vec![BiDay::One, BiDay::Zero, BiDay::One, BiDay::Zero]),
            (2022, Month::January, vec![BiDay::Zero, BiDay::Zero, BiDay::One, BiDay::One]),
        ];
        let result = group_days_calendar(DaysCalendar { days_calendar: days });
        let expected: Vec<(Year, Month, Vec<Vec<BiDay>>)> = vec![
            (2022, Month::January, vec![vec![BiDay::One, BiDay::Zero, BiDay::One, BiDay::Zero], vec![BiDay::Zero, BiDay::Zero, BiDay::One, BiDay::One]]),
            (2022, Month::February, vec![vec![BiDay::One, BiDay::Zero, BiDay::Zero, BiDay::One]]),
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_resume_days_with_and_operator() {
        let days = vec![
            (2022, Month::February, vec![BiDay::One,BiDay::Zero,BiDay::Zero,BiDay::One]),
            (2022, Month::January, vec![BiDay::One,BiDay::Zero,BiDay::One,BiDay::Zero]),
            (2022, Month::January, vec![BiDay::Zero,BiDay::Zero,BiDay::One,BiDay::One]),
        ];
        let calendar = DaysCalendar { days_calendar: days };
        let res = resume(&calendar, |a, b| and_biday_operation(a, b));
        assert_eq!(res, DaysCalendar { 
            days_calendar: vec![
                (2022, Month::January, vec![BiDay::Zero,BiDay::Zero,BiDay::One,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero]), 
                (2022, Month::February, vec![BiDay::One,BiDay::Zero,BiDay::Zero,BiDay::One,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero])
            ]
        });
    }
    
    #[test]
    fn test_resume_days_with_or_operator() {
        let days = vec![
            (2022, Month::February, vec![BiDay::One,BiDay::Zero,BiDay::Zero,BiDay::One]),
            (2022, Month::January, vec![BiDay::One,BiDay::Zero,BiDay::One,BiDay::Zero]),
            (2022, Month::January, vec![BiDay::Zero,BiDay::Zero,BiDay::One,BiDay::One]),
        ];
        let calendar = DaysCalendar { days_calendar: days };
        let res = resume(&calendar, |a, b| or_biday_operation(a, b));
        assert_eq!(res, DaysCalendar { 
            days_calendar: vec![
                (2022, Month::January, vec![BiDay::One,BiDay::Zero,BiDay::One,BiDay::One,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero]), 
                (2022, Month::February, vec![BiDay::One,BiDay::Zero,BiDay::Zero,BiDay::One,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero])
            ]
        });
    }

    #[test]
    fn test_resume_with_empty_calendar() {
        let calendar = DaysCalendar { days_calendar: vec![] };
        let res = resume(&calendar, |a, b| and_biday_operation(a, b));
        assert_eq!(res, DaysCalendar { days_calendar: vec![] });
    }

    #[test]
    fn test_resume_with_single_day() {
        let days = vec![(2022, Month::January, vec![BiDay::One,BiDay::Zero,BiDay::Zero,BiDay::Zero, BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero])];
        let calendar = DaysCalendar { days_calendar: days };
        let res = resume(&calendar, |a, b| and_biday_operation(a, b));
        assert_eq!(res, calendar);
    }

    #[test]
    fn test_resume_with_multiple_days() {
        let days = vec![
            (2022, Month::January, vec![BiDay::One,BiDay::Zero,BiDay::Zero,BiDay::Zero]),
            (2022, Month::January, vec![BiDay::Zero,BiDay::One,BiDay::Zero,BiDay::Zero]),
            (2022, Month::February, vec![BiDay::One,BiDay::One,BiDay::Zero,BiDay::Zero]),
            (2022, Month::February, vec![BiDay::Zero,BiDay::Zero,BiDay::One,BiDay::Zero]),
            (2023, Month::January, vec![BiDay::One,BiDay::One,BiDay::One,BiDay::Zero]),
            (2023, Month::February, vec![BiDay::Zero,BiDay::One,BiDay::One,BiDay::One]),
            (2023, Month::February, vec![BiDay::One,BiDay::Zero,BiDay::Zero,BiDay::One]),
        ];
        let calendar = DaysCalendar { days_calendar: days };
        let res = resume(&calendar, |a, b| and_biday_operation(a, b));
        assert_eq!(res, DaysCalendar {
            days_calendar: vec![
                (2022, Month::January, vec![BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero]),
                (2022, Month::February, vec![BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero]),
                (2023, Month::January, vec![BiDay::One,BiDay::One,BiDay::One,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero]),
                (2023, Month::February, vec![BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::One,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero]),
            ]
        });
    }

    #[test]
    fn test_resume_with_or_operator() {
        let days = vec![
            (2022, Month::January, vec![BiDay::One,BiDay::Zero,BiDay::Zero,BiDay::Zero]),
            (2022, Month::January, vec![BiDay::Zero,BiDay::One,BiDay::Zero,BiDay::Zero]),
            (2022, Month::February, vec![BiDay::One,BiDay::One,BiDay::Zero,BiDay::Zero]),
            (2022, Month::February, vec![BiDay::Zero,BiDay::Zero,BiDay::One,BiDay::Zero]),
            (2023, Month::January, vec![BiDay::One,BiDay::One,BiDay::One,BiDay::Zero]),
            (2023, Month::February, vec![BiDay::Zero,BiDay::One,BiDay::One,BiDay::One]),
            (2023, Month::February, vec![BiDay::One,BiDay::Zero,BiDay::Zero,BiDay::One]),
        ];
        let calendar = DaysCalendar { days_calendar: days };
        let res = resume(&calendar, |a, b| or_biday_operation(a, b));
        assert_eq!(res, DaysCalendar {
            days_calendar: vec![
                (2022, Month::January, vec![BiDay::One,BiDay::One,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero]),
                (2022, Month::February, vec![BiDay::One,BiDay::One,BiDay::One,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero]),
                (2023, Month::January, vec![BiDay::One,BiDay::One,BiDay::One,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero]),
                (2023, Month::February, vec![BiDay::One,BiDay::One,BiDay::One,BiDay::One,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero]),
            ]
        });
    }

    #[test]
    fn test_add_empty_calendars() {
        let empty_cal = DaysCalendar { days_calendar: Vec::new() };
        let res = empty_cal.append(&empty_cal);
        assert_eq!(res, empty_cal);
    }

    #[test]
    fn test_append_empty_calendar_to_nonempty_calendar() {
        let nonempty_cal = DaysCalendar { 
            days_calendar: vec![
                (2022, Month::January, vec![BiDay::One, BiDay::One, BiDay::One, BiDay::Zero, BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero]),
                (2022, Month::February, vec![BiDay::Zero, BiDay::Zero, BiDay::One, BiDay::One, BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero]),
                (2022, Month::March, vec![BiDay::One, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero]),
            ]
        };
        let empty_cal = DaysCalendar { days_calendar: Vec::new() };
        let res = nonempty_cal.append(&empty_cal);
        assert_eq!(res, nonempty_cal);
    }

    #[test]
    fn test_append_nonempty_calendar_to_empty_calendar() {
        let nonempty_cal = DaysCalendar { 
            days_calendar: vec![
                (2022, Month::January, vec![BiDay::One, BiDay::One, BiDay::One, BiDay::Zero, BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero]),
                (2022, Month::February, vec![BiDay::Zero, BiDay::Zero, BiDay::One, BiDay::One, BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero]),
                (2022, Month::March, vec![BiDay::One, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero]),
            ]
        };
        let empty_cal = DaysCalendar { days_calendar: Vec::new() };
        let res = empty_cal.append(&nonempty_cal);
        assert_eq!(res, nonempty_cal);
    }

    #[test]
    fn test_append_nonempty_calendars_same_month() {
        let cal1 = DaysCalendar { 
            days_calendar: vec![
                (2022, Month::January, vec![BiDay::One, BiDay::One, BiDay::One, BiDay::Zero]),
            ]
        };
        let cal2 = DaysCalendar { 
            days_calendar: vec![
                (2022, Month::January, vec![BiDay::Zero, BiDay::One, BiDay::Zero, BiDay::One]),
            ]
        };
        let expected_res = DaysCalendar { 
            days_calendar: vec![
                (2022, Month::January, vec![BiDay::One, BiDay::One, BiDay::One, BiDay::Zero]),
                (2022, Month::January, vec![BiDay::Zero, BiDay::One, BiDay::Zero, BiDay::One]),
            ]
        };
        let res = cal1.append(&cal2);
        assert_eq!(res, expected_res);
    }

    #[test]
    fn test_append_nonempty_calendars_different_months() {
        let cal1 = DaysCalendar { 
            days_calendar: vec![
                (2022, Month::January, vec![BiDay::One, BiDay::One, BiDay::One, BiDay::Zero]),
            ]
        };
        let cal2 = DaysCalendar { 
            days_calendar: vec![
                (2022, Month::February , vec![BiDay::Zero, BiDay::One, BiDay::Zero, BiDay::One]),
            ]
        };
        let expected_res = DaysCalendar { 
            days_calendar: vec![
                (2022, Month::January, vec![BiDay::One, BiDay::One, BiDay::One, BiDay::Zero]),
                (2022, Month::February, vec![BiDay::Zero, BiDay::One, BiDay::Zero, BiDay::One]),
            ]
        };
        let res = cal1.append(&cal2);
        assert_eq!(res, expected_res);
    }

    #[test]
    fn test_or() {
        let days1 = vec![
            (2022, Month::February, vec![BiDay::One, BiDay::Zero, BiDay::Zero, BiDay::One, BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero]),
            (2022, Month::January, vec![BiDay::One, BiDay::Zero, BiDay::One, BiDay::Zero, BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero]),
        ];
        let days2 = vec![
            (2022, Month::February, vec![BiDay::Zero, BiDay::One, BiDay::One, BiDay::Zero, BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero]),
            (2022, Month::January, vec![BiDay::Zero, BiDay::Zero, BiDay::One, BiDay::One, BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero]),
        ];
        let calendar1 = DaysCalendar { days_calendar: days1 };
        let calendar2 = DaysCalendar { days_calendar: days2 };

        let result = calendar1.or(&calendar2);

        let expected_days = vec![
            (2022, Month::January, vec![BiDay::One, BiDay::Zero, BiDay::One, BiDay::One, BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero]),
            (2022, Month::February, vec![BiDay::One, BiDay::One, BiDay::One, BiDay::One, BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero]),
        ];
        let expected_calendar = DaysCalendar { days_calendar: expected_days };

        assert_eq!(result, expected_calendar);
    }

    #[test]
    fn test_and() {
        let days1 = vec![
            (2022, Month::February, vec![BiDay::One, BiDay::Zero, BiDay::Zero, BiDay::One, BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero]),
            (2022, Month::January, vec![BiDay::One, BiDay::Zero, BiDay::One, BiDay::Zero, BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero]),
        ];
        let days2 = vec![
            (2022, Month::February, vec![BiDay::Zero, BiDay::One, BiDay::One, BiDay::Zero, BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero]),
            (2022, Month::January, vec![BiDay::Zero, BiDay::Zero, BiDay::One, BiDay::One, BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero]),
        ];
        let calendar1 = DaysCalendar { days_calendar: days1 };
        let calendar2 = DaysCalendar { days_calendar: days2 };

        let result = calendar1.and(&calendar2);

        let expected_days = vec![(2022, Month::January, vec![BiDay::Zero, BiDay::Zero, BiDay::One, BiDay::Zero, BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero]), (2022, Month::February, vec![BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero])];
        let expected_calendar = DaysCalendar { days_calendar: expected_days };

        assert_eq!(result, expected_calendar);
    }

    #[test]
    fn test_match() {
        let days1 = vec![
            (2022, Month::February, vec![BiDay::One, BiDay::Zero, BiDay::Zero, BiDay::One]),
            (2022, Month::January, vec![BiDay::One, BiDay::Zero, BiDay::One, BiDay::Zero]),
        ];
        let days2 = vec![
            (2022, Month::February, vec![BiDay::Zero, BiDay::One, BiDay::One, BiDay::Zero]),
            (2022, Month::January, vec![BiDay::Zero, BiDay::Zero, BiDay::One, BiDay::One]),
        ];
        let calendar1 = DaysCalendar { days_calendar: days1 };
        let calendar2 = DaysCalendar { days_calendar: days2 };

        let result = calendar1.r#match(&calendar2);

        let expected_days = vec![
            (2022, Month::January, vec![BiDay::Zero, BiDay::One, BiDay::One, BiDay::Zero, BiDay::One,BiDay::One,BiDay::One,BiDay::One,BiDay::One,BiDay::One,BiDay::One,BiDay::One,BiDay::One,BiDay::One,BiDay::One,BiDay::One,BiDay::One,BiDay::One,BiDay::One,BiDay::One,BiDay::One,BiDay::One,BiDay::One,BiDay::One,BiDay::One,BiDay::One,BiDay::One,BiDay::One,BiDay::One,BiDay::One,BiDay::One]),
            (2022, Month::February, vec![BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::One,BiDay::One,BiDay::One,BiDay::One,BiDay::One,BiDay::One,BiDay::One,BiDay::One,BiDay::One,BiDay::One,BiDay::One,BiDay::One,BiDay::One,BiDay::One,BiDay::One,BiDay::One,BiDay::One,BiDay::One,BiDay::One,BiDay::One,BiDay::One,BiDay::One,BiDay::One,BiDay::One]),
        ];
        let expected_calendar = DaysCalendar { days_calendar: expected_days };

        assert_eq!(result, expected_calendar);
    }

    #[test]
    fn test_nomatch() {
        let days1 = vec![
            (2022, Month::February, vec![BiDay::One, BiDay::Zero, BiDay::Zero, BiDay::One]),
            (2022, Month::January, vec![BiDay::One, BiDay::Zero, BiDay::One, BiDay::Zero]),
        ];
        let days2 = vec![
            (2022, Month::February, vec![BiDay::Zero, BiDay::One, BiDay::One, BiDay::Zero]),
            (2022, Month::January, vec![BiDay::Zero, BiDay::Zero, BiDay::One, BiDay::One]),
        ];
        let calendar1 = DaysCalendar { days_calendar: days1 };
        let calendar2 = DaysCalendar { days_calendar: days2 };

        let result = calendar1.nomatch(&calendar2);

        let expected_days = vec![
            (2022, Month::January, vec![BiDay::One, BiDay::Zero, BiDay::Zero, BiDay::One, BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero]),
            (2022, Month::February, vec![BiDay::One, BiDay::One, BiDay::One, BiDay::One, BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero,BiDay::Zero]),
        ];
        let expected_calendar = DaysCalendar { days_calendar: expected_days };

        assert_eq!(result, expected_calendar);
    }

    #[test]
    fn test_replicate_pattern_single() {
        let pattern: [BiDay; 1]  = [BiDay::One];
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
        let expected = vec![BiDay::Zero, BiDay::One, BiDay::Zero, BiDay::One, BiDay::Zero];
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
        let expected = vec![BiDay::One, BiDay::Zero, BiDay::Zero, BiDay::One, BiDay::One, BiDay::One, BiDay::Zero, BiDay::Zero, BiDay::One, BiDay::One];
        assert_eq!(replicate_pattern(&pattern, n), expected);
    }

    #[test]
    fn test_replicate_with_empty_calendar() {
        let calendar = DaysCalendar { days_calendar: Vec::new() };
        let pattern: [BiDay; 3] = [BiDay::Zero, BiDay::One, BiDay::One];
        let result = replicate::<BiDay>(&pattern, calendar);
        assert_eq!(result.days_calendar, Vec::new());
    }

    #[test]
    fn test_replicate_with_single_day() {
        let calendar = DaysCalendar {
            days_calendar: vec![
                (2023, Month::January, vec![BiDay::Zero]),
            ],
        };
        let pattern: [BiDay; 3] = [BiDay::Zero, BiDay::One, BiDay::Zero];
        let result = replicate::<BiDay>(&pattern, calendar);
        assert_eq!(result.days_calendar, vec![
            (2023, Month::January, vec![BiDay::Zero]),
        ]);
    }

    #[test]
    fn test_replicate_with_multiple_days() {
        let calendar = DaysCalendar {
            days_calendar: vec![
                (2023, Month::January, vec![BiDay::Zero, BiDay::One, BiDay::One, BiDay::One, BiDay::One]),
                (2023, Month::February, vec![BiDay::One, BiDay::One, BiDay::One]),
                (2023, Month::March, vec![BiDay::One, BiDay::Zero, BiDay::Zero, BiDay::One]),
            ],
        };
        let pattern: [BiDay; 3] = [BiDay::Zero, BiDay::One, BiDay::One];
        let result = replicate::<BiDay>(&pattern, calendar);
        assert_eq!(result.days_calendar, vec![
            (2023, Month::January, vec![BiDay::Zero, BiDay::One, BiDay::One, BiDay::Zero, BiDay::One]),
            (2023, Month::February, vec![BiDay::Zero, BiDay::One, BiDay::One]),
            (2023, Month::March, vec![BiDay::Zero, BiDay::One, BiDay::One, BiDay::Zero]),
        ]);
    }

    #[test]
    fn test_replicate_with_large_pattern() {
        let calendar = DaysCalendar {
            days_calendar: vec![
                (2023, Month::January, vec![BiDay::Zero, BiDay::One, BiDay::One]),
                (2023, Month::February, vec![BiDay::One, BiDay::One, BiDay::One]),
            ],
        };
        let pattern: [BiDay; 6] = [BiDay::Zero, BiDay::One, BiDay::One, BiDay::One, BiDay::One, BiDay::One];
        let result = replicate::<BiDay>(&pattern, calendar);
        assert_eq!(result.days_calendar, vec![
            (2023, Month::January, vec![BiDay::Zero, BiDay::One, BiDay::One]),
            (2023, Month::February, vec![BiDay::Zero, BiDay::One, BiDay::One]),
        ]);
    }

    #[test]
    fn test_zeros() {
        // Create a DaysCalendar object with some sample data
        let calendar = DaysCalendar {
            days_calendar: vec![
                (2023, Month::January, vec![BiDay::One, BiDay::One, BiDay::One, BiDay::One, BiDay::One, BiDay::One, BiDay::One]),
                (2023, Month::February, vec![BiDay::One, BiDay::One, BiDay::One, BiDay::One, BiDay::One, BiDay::One, BiDay::One]),
                (2023, Month::March, vec![BiDay::One, BiDay::One, BiDay::One, BiDay::One, BiDay::One, BiDay::One, BiDay::One]),
            ],
        };

        // Call the zeros() function on the DaysCalendar object
        let result = calendar.zeros();

        // Validate the result
        assert_eq!(
            result.days_calendar,
            vec![
                (2023, Month::January, vec![BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero]),
                (2023, Month::February, vec![BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero]),
                (2023, Month::March, vec![BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero]),
            ]
        );
    }

    #[test]
    fn test_zeros_full() {
        let calendar = DaysCalendar::<BiDay> { days_calendar: vec![] };
        let zeros_calendar = calendar.zeros();
        assert_eq!(zeros_calendar.days_calendar.len(), 0);

        let days = vec![(2023, Month::March, vec![BiDay::Zero; 31]), (2023, Month::April, vec![BiDay::Zero; 30])];
        let calendar = DaysCalendar::<BiDay> { days_calendar: days };
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
        let calendar = DaysCalendar::<BiDay> { days_calendar: vec![] };
        let ones_calendar = calendar.ones();
        assert_eq!(ones_calendar.days_calendar.len(), 0);

        let days = vec![(2023, Month::March, vec![BiDay::Zero; 31]), (2023, Month::April, vec![BiDay::Zero; 30])];
        let calendar = DaysCalendar::<BiDay> { days_calendar: days };
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
        let mut calendar: DaysCalendar<BiDay> = DaysCalendar::singleton(2023, Month::March).unwrap();
        let expected_calendar = DaysCalendar {
            days_calendar: vec![(2023, Month::March, vec![BiDay::Zero,BiDay::One,BiDay::Zero,BiDay::One,BiDay::Zero,BiDay::One,BiDay::Zero,BiDay::One,BiDay::Zero,BiDay::One,BiDay::Zero,BiDay::One,BiDay::Zero,BiDay::One,BiDay::Zero,BiDay::One,BiDay::Zero,BiDay::One,BiDay::Zero,BiDay::One,BiDay::Zero,BiDay::One,BiDay::Zero,BiDay::One,BiDay::Zero,BiDay::One,BiDay::Zero,BiDay::One,BiDay::Zero,BiDay::One,BiDay::Zero])],
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
                (2023, Month::March, vec![BiDay::Zero,BiDay::One,BiDay::Zero,BiDay::One,BiDay::Zero,BiDay::One,BiDay::Zero,BiDay::One,BiDay::Zero,BiDay::One,BiDay::Zero,BiDay::One,BiDay::Zero,BiDay::One,BiDay::Zero,BiDay::One,BiDay::Zero,BiDay::One,BiDay::Zero,BiDay::One,BiDay::Zero,BiDay::One,BiDay::Zero,BiDay::One,BiDay::Zero,BiDay::One,BiDay::Zero,BiDay::One,BiDay::Zero,BiDay::One,BiDay::Zero]),
                (2023, Month::May, vec![BiDay::Zero,BiDay::One,BiDay::Zero,BiDay::One,BiDay::Zero,BiDay::One,BiDay::Zero,BiDay::One,BiDay::Zero,BiDay::One,BiDay::Zero,BiDay::One,BiDay::Zero,BiDay::One,BiDay::Zero,BiDay::One,BiDay::Zero,BiDay::One,BiDay::Zero,BiDay::One,BiDay::Zero,BiDay::One,BiDay::Zero,BiDay::One,BiDay::Zero,BiDay::One,BiDay::Zero,BiDay::One,BiDay::Zero,BiDay::One,BiDay::Zero]),
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
                (2023, Month::March, vec![BiDay::One,BiDay::Zero,BiDay::One,BiDay::Zero,BiDay::One,BiDay::Zero,BiDay::One,BiDay::Zero,BiDay::One,BiDay::Zero,BiDay::One,BiDay::Zero,BiDay::One,BiDay::Zero,BiDay::One,BiDay::Zero,BiDay::One,BiDay::Zero,BiDay::One,BiDay::Zero,BiDay::One,BiDay::Zero,BiDay::One,BiDay::Zero,BiDay::One,BiDay::Zero,BiDay::One,BiDay::Zero,BiDay::One,BiDay::Zero,BiDay::One]),
                (2023, Month::May, vec![BiDay::One,BiDay::Zero,BiDay::One,BiDay::Zero,BiDay::One,BiDay::Zero,BiDay::One,BiDay::Zero,BiDay::One,BiDay::Zero,BiDay::One,BiDay::Zero,BiDay::One,BiDay::Zero,BiDay::One,BiDay::Zero,BiDay::One,BiDay::Zero,BiDay::One,BiDay::Zero,BiDay::One,BiDay::Zero,BiDay::One,BiDay::Zero,BiDay::One,BiDay::Zero,BiDay::One,BiDay::Zero,BiDay::One,BiDay::Zero,BiDay::One]),
            ],
        };
        let result_calendar = calendar.downward_step();
        assert_eq!(result_calendar, expected_calendar);
    }

    fn test_new() {
        let data = vec![
            (2022, Month::January, vec![BiDay::One, BiDay::One, BiDay::Zero]),
            (2022, Month::February, vec![BiDay::Zero, BiDay::One, BiDay::One]),
            (2022, Month::March, vec![BiDay::One, BiDay::Zero, BiDay::One]),
        ];
        let calendar: DaysCalendar<BiDay> = DaysCalendar::new(data.clone());
        assert_eq!(calendar.days_calendar, data);
    }

    #[test]
    fn test_new_empty() {
        let calendar = DaysCalendar::<BiDay>::new(Vec::new());
        assert_eq!(calendar.days_calendar, Vec::<(Year, Month, Vec<BiDay>)>::new());
    }

    #[test]
    fn test_biday_to_vec_day_single_month() {
        let year: Year = 2022;
        let month: Month = Month::January;
        let days = vec![BiDay::Zero, BiDay::One, BiDay::Zero, BiDay::One, BiDay::Zero, BiDay::One, BiDay::Zero];
        let calendar: DaysCalendar<BiDay> = DaysCalendar::<BiDay>::new(vec![(year, month, days)]);
        let expected_days = vec![
            (2022 as Year, Month::January as Month, vec![2, 4, 6]),
        ];
        let actual_days = biday_to_vec_day(calendar);
        assert_eq!(actual_days, expected_days);
    }
    
    #[test]
    fn test_biday_to_vec_day_multiple_months() {
        let year = 2022;
        let month1 = Month::January;
        let days1 = vec![BiDay::Zero, BiDay::One, BiDay::Zero, BiDay::One, BiDay::Zero, BiDay::One, BiDay::Zero];
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
    fn test_to_day() {
        let bi_data = vec![
            (2022, Month::January, vec![BiDay::Zero, BiDay::Zero, BiDay::One]),
            (2022, Month::February, vec![BiDay::One, BiDay::Zero, BiDay::One]),
            (2022, Month::March, vec![BiDay::Zero, BiDay::One, BiDay::One]),
        ];
        let calendar_bi = DaysCalendar::new(bi_data.clone());

        let day_data = vec![
            (2022 as Year, Month::January as Month, vec![3 as u8]),
            (2022 as Year, Month::February as Month, vec![1 as u8, 3 as u8]),
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
                (2023, Month::January, vec![BiDay::Zero, BiDay::One, BiDay::One]),
                (2023, Month::February, vec![BiDay::One, BiDay::Zero, BiDay::One]),
                (2023, Month::March, vec![BiDay::Zero, BiDay::Zero, BiDay::One]),
            ],
        }
    }

    #[test]
    fn test_invert_biday_values() {
        let calendar = create_test_calendar_1();
        let inverted_calendar = calendar.invert_biday();

        let expected_inverted_calendar = DaysCalendar {
            days_calendar: vec![
                (2023, Month::January, vec![BiDay::One, BiDay::Zero, BiDay::Zero]),
                (2023, Month::February, vec![BiDay::Zero, BiDay::One, BiDay::Zero]),
                (2023, Month::March, vec![BiDay::One, BiDay::One, BiDay::Zero]),
            ],
        };

        assert_eq!(inverted_calendar, expected_inverted_calendar);
    }

    fn setup_calendar() -> DaysCalendar<BiDay> {
        DaysCalendar {
            days_calendar: vec![
                (2023, Month::January, vec![BiDay::Zero, BiDay::One, BiDay::Zero]),
                (2023, Month::February, vec![BiDay::One, BiDay::Zero, BiDay::One]),
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
            days_calendar: vec![
                (2023, Month::January, vec![BiDay::Zero, BiDay::One]),
            ],
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn test_take_n_biday_across_months() {
        let calendar = setup_calendar();
        let result = calendar.take_n_biday(5);
        let expected = DaysCalendar {
            days_calendar: vec![
                (2023, Month::January, vec![BiDay::Zero, BiDay::One, BiDay::Zero]),
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
            days_calendar: vec![(2023, Month::March, vec![BiDay::Zero, BiDay::One, BiDay::Zero])],
        };
        let dates = to_date(calendar);
        assert_eq!(
            dates,
            vec![
                Date::from_calendar_date(2023, Month::March.to_time_month(), 2).unwrap(),
            ]
        );
    }

    #[test]
    fn test_to_date_multiple_months() {
        let calendar = DaysCalendar {
            days_calendar: vec![
                (2023, Month::February, vec![BiDay::Zero, BiDay::One]),
                (2023, Month::March, vec![BiDay::Zero, BiDay::One, BiDay::Zero]),
            ],
        };
        let dates = to_date(calendar);
        assert_eq!(
            dates,
            vec![
                Date::from_calendar_date(2023, Month::February.to_time_month(), 2).unwrap(),
                Date::from_calendar_date(2023, Month::March.to_time_month(), 2).unwrap(),
            ]
        );
    }

    #[test]
    fn test_to_date_multiple_years() {
        let calendar = DaysCalendar {
            days_calendar: vec![
                (2022, Month::December, vec![BiDay::Zero, BiDay::One]),
                (2023, Month::January, vec![BiDay::Zero, BiDay::One, BiDay::Zero]),
            ],
        };
        let dates = to_date(calendar);
        assert_eq!(
            dates,
            vec![
                Date::from_calendar_date(2022, Month::December.to_time_month(), 2).unwrap(),
                Date::from_calendar_date(2023, Month::January.to_time_month(), 2).unwrap(),
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
        let dates = vec![date!(2022-03-30)];
        let calendar = from_date(dates);
        assert_eq!(calendar.days_calendar.len(), 1);
        assert_eq!(calendar.days_calendar[0], (2022, Month::March, vec![30]));
    }

    #[test]
    fn test_from_date_same_month() {
        let dates = vec![date!(2022-03-30), date!(2022-03-15), date!(2022-03-20)];
        let calendar = from_date(dates);
        assert_eq!(calendar.days_calendar.len(), 1);
        assert_eq!(calendar.days_calendar[0], (2022, Month::March, vec![15, 20, 30]));
    }

    #[test]
    fn test_from_date_different_months() {
        let dates = vec![date!(2022-03-30), date!(2022-02-15), date!(2022-01-20)];
        let calendar = from_date(dates);
        assert_eq!(calendar.days_calendar.len(), 3);
        assert_eq!(calendar.days_calendar[0], (2022, Month::January, vec![20]));
        assert_eq!(calendar.days_calendar[1], (2022, Month::February, vec![15]));
        assert_eq!(calendar.days_calendar[2], (2022, Month::March, vec![30]));
    }

    #[test]
    fn test_from_date_different_years() {
        let dates = vec![date!(2023-03-30), date!(2022-03-15), date!(2021-03-20)];
        let calendar = from_date(dates);
        assert_eq!(calendar.days_calendar.len(), 3);
        assert_eq!(calendar.days_calendar[0], (2021, Month::March, vec![20]));
        assert_eq!(calendar.days_calendar[1], (2022, Month::March, vec![15]));
        assert_eq!(calendar.days_calendar[2], (2023, Month::March, vec![30]));
    }

    #[test]
    fn test_from_date_duplicate_dates() {
        let dates = vec![date!(2022-03-30), date!(2022-03-15), date!(2022-03-30)];
        let calendar = from_date(dates);
        assert_eq!(calendar.days_calendar.len(), 1);
        assert_eq!(calendar.days_calendar[0], (2022, Month::March, vec![15, 30]));
    }

    #[test]
    fn test_from_day() {
        let input_calendar = DaysCalendar {
            days_calendar: vec![
                (
                    2020,
                    Month::January,
                    vec![1, 15, 31],
                ),
                (
                    2020,
                    Month::February,
                    vec![14, 29],
                ),
                (
                    2020,
                    Month::March,
                    vec![1, 31],
                ),
            ],
        };

        let expected_calendar = DaysCalendar {
            days_calendar: vec![
                (
                    2020,
                    Month::January,
                    vec![
                        BiDay::One, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero,
                        BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero,
                        BiDay::Zero, BiDay::Zero, BiDay::One, BiDay::Zero, BiDay::Zero, BiDay::Zero,
                        BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero,
                        BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero,
                        BiDay::One,
                    ],
                ),
                (
                    2020,
                    Month::February,
                    vec![
                        BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero,
                        BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero,
                        BiDay::Zero, BiDay::One, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero,
                        BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero,
                        BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::One,
                    ],
                ),
                (
                    2020,
                    Month::March,
                    vec![
                        BiDay::One, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero,
                        BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero,
                        BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero,
                        BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero,
                        BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero,
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
        let dates = vec![
            date!(0001 - 01 - 01),
            date!(9999 - 12 - 31),
        ];

        let expected_add_max = vec![
            MAX_DATE,
            MAX_DATE,
        ];

        let expected_sub_max = vec![
            MIN_DATE,
            MIN_DATE,
        ];

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
                (2020, Month::January, vec![BiDay::One, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::One, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::One]),
                (2020, Month::February, vec![BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::One, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::One]),
                (2020, Month::March, vec![BiDay::One, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::One]),
            ],
        }
    }

    #[test]
    fn test_next_day() {
        let calendar = sample_calendar();
        let date = Date::from_calendar_date(2020, Month::January.to_time_month(), 1).unwrap();
        assert_eq!(calendar.next_day(2020, Month::January, 1), Some(date!(2020-01-15)));
        assert_eq!(calendar.next_day(2020, Month::January, 15), Some(date!(2020-01-30)));
        assert_eq!(calendar.next_day(2020, Month::February, 14), Some(date!(2020-02-28)));
        assert_eq!(calendar.next_day(2020, Month::February, 28), Some(date!(2020-03-01)));
        assert_eq!(calendar.next_day(2020, Month::March, 1), Some(date!(2020-03-30)));
    }

    #[test]
    fn test_next_day_edge_cases() {
        let calendar = sample_calendar();

        // No next day available
        assert_eq!(calendar.next_day(2020, Month::March, 30), None);

        // Invalid date input
        assert_eq!(calendar.next_day(2020, Month::January, 31), Some(date!(2020-02-14)));
        assert_eq!(calendar.next_day(2020, Month::February, 28), Some(date!(2020-03-01)));
        assert_eq!(calendar.next_day(2020, Month::April, 31), None);

        // Test with leap year
        let mut leap_year_calendar = calendar.clone();
        leap_year_calendar.days_calendar.push((2020, Month::February, vec![BiDay::One]));
        let leap_date = Date::from_calendar_date(2020, Month::February.to_time_month(), 29).unwrap();
        assert_eq!(leap_year_calendar.next_day(2020, Month::February, 28), Some(date!(2020-03-01)));
        
        // Test with non-leap year
        let non_leap_date = Date::from_calendar_date(2019, Month::February.to_time_month(), 28).unwrap();
        assert_eq!(calendar.next_day(2019, Month::February, 28), Some(date!(2020-01-01)));
        
    }

    fn create_test_calendar() -> DaysCalendar<BiDay> {
        DaysCalendar {
            days_calendar: vec![
                (2020, Month::January, vec![BiDay::One, BiDay::Zero, BiDay::One, BiDay::Zero, BiDay::Zero]),
                (2020, Month::February, vec![BiDay::Zero, BiDay::Zero, BiDay::One, BiDay::Zero, BiDay::One]),
                (2020, Month::March, vec![BiDay::One, BiDay::Zero, BiDay::Zero, BiDay::One, BiDay::Zero]),
            ],
        }
    }

    #[test]
    fn test_previous_day_basic() {
        let calendar = create_test_calendar();
        let prev_day = calendar.previous_day(2020, Month::January, 3);
        assert_eq!(prev_day, Some(date!(2020-01-01)));

        let prev_day = calendar.previous_day(2020, Month::February, 5);
        assert_eq!(prev_day, Some(date!(2020-02-03)));

        let prev_day = calendar.previous_day(2020, Month::March, 4);
        assert_eq!(prev_day, Some(date!(2020-03-01)));
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
        assert_eq!(prev_day, Some(date!(2020-01-03)));

        let prev_day = calendar.previous_day(2020, Month::March, 31);
        assert_eq!(prev_day, Some(date!(2020-03-04)));
    }

    #[test]
    fn test_seek_nth_day_days() {
        let calendar = create_test_calendar();

        let new_date = calendar.seek_nth_day(2020, Month::January, 1, 2).unwrap();
        assert_eq!(new_date, date!(2020-02-03));

        let new_date = calendar.seek_nth_day(2020, Month::February, 5, -1).unwrap();
        assert_eq!(new_date, date!(2020-02-03));

        let new_date = calendar.seek_nth_day(2020, Month::March, 4, 0).unwrap();
        assert_eq!(new_date, date!(2020-03-04));
    }

    #[test]
    fn test_seek_nth_day_not_found() {
        let calendar = create_test_calendar();

        let new_date = calendar.seek_nth_day(2020, Month::January, 1, -1);
        assert!(new_date.is_none());

        let new_date = calendar.seek_nth_day(2020, Month::March, 1, 5);
        assert!(new_date.is_none());
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

        assert_eq!(calendar, result);
    }

    #[test]
    fn test_or_weekdays_single_weekday() {
        let calendar = generate_sample_calendar();
        let mut weekdays = HashSet::new();
        weekdays.insert(Weekday::Monday);

        let result = calendar.or_weekdays(weekdays.clone());

        for (year, month, days) in result.days_calendar {
            for (day_index, bit) in days.iter().enumerate() {
                let day_num = day_index as u8 + 1;
                let date = Date::from_calendar_date(year.into(), month.to_time_month(), day_num).unwrap();
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

        let result = calendar.or_weekdays(weekdays.clone());

        for (year, month, days) in result.days_calendar {
            for (day_index, bit) in days.iter().enumerate() {
                let day_num = day_index as u8 + 1;
                let date = Date::from_calendar_date(year.into(), month.to_time_month(), day_num).unwrap();
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
        let result = calendar.or_iso_weeks(weeks);

        assert_eq!(calendar, result);
    }

    #[test]
    fn test_or_iso_weeks_single_week() {
        let calendar = generate_sample_calendar();
        let weeks = vec![5];

        let result = calendar.or_iso_weeks(weeks.clone());

        for (year, month, days) in result.days_calendar {
            let first_day_of_month = Date::from_calendar_date(year.into(), month.to_time_month(), 1).unwrap();
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

        let result = calendar.or_iso_weeks(weeks.clone());

        for (year, month, days) in result.days_calendar {
            let first_day_of_month = Date::from_calendar_date(year.into(), month.to_time_month(), 1).unwrap();
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

        let result = calendar.or_iso_weeks(weeks);

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
        let result = calendar.and_weekdays(weekdays);

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

        let result = calendar.and_weekdays(weekdays.clone());

        for (year, month, days) in result.days_calendar {
            let mut day_num = 1;
            for (_, bit) in days.iter().enumerate() {
                let date = Date::from_calendar_date(year.into(), month.to_time_month(), day_num).unwrap();
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

        let result = calendar.and_weekdays(weekdays.clone());

        for (year, month, days) in result.days_calendar {
            let mut day_num = 1;
            for (_, bit) in days.iter().enumerate() {
                let date = Date::from_calendar_date(year.into(), month.to_time_month(), day_num).unwrap();
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
            BiDay::One, BiDay::Zero, BiDay::One, BiDay::Zero, BiDay::One,
            BiDay::Zero, BiDay::One, BiDay::Zero, BiDay::One, BiDay::Zero,
            BiDay::One, BiDay::Zero, BiDay::One, BiDay::Zero, BiDay::One,
            BiDay::Zero, BiDay::One, BiDay::Zero, BiDay::One, BiDay::Zero,
            BiDay::One, BiDay::Zero, BiDay::One, BiDay::Zero, BiDay::One,
            BiDay::Zero, BiDay::One, BiDay::Zero, BiDay::One, BiDay::Zero,
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

        let new_calendar = calendar.not_weekdays(weekdays);
        let expected_days = vec![
            BiDay::One, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::One,
            BiDay::Zero, BiDay::One, BiDay::Zero, BiDay::One, BiDay::Zero,
            BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::One,
            BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::One, BiDay::Zero,
            BiDay::One, BiDay::Zero, BiDay::One, BiDay::Zero, BiDay::Zero,
            BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::One, BiDay::Zero,
            BiDay::Zero,
        ];

        assert_eq!(new_calendar.days_calendar[0].2, expected_days);
    }

    #[test]
    fn test_not_weekdays_empty_set() {
        let calendar = create_sample_calendar();
        let weekdays = HashSet::new();

        let new_calendar = calendar.not_weekdays(weekdays);

        assert_eq!(new_calendar, calendar);
    }


    use std::iter::repeat;

    #[test]
    fn test_not_iso_weeks() {
        let calendar = create_sample_calendar();
        let weeks_to_exclude = vec![1, 5, 52];

        let calendar = DaysCalendar {
            days_calendar: vec![(2023, Month::January, vec![BiDay::One; 31])
            , (2023, Month::December, vec![BiDay::One; 31])]

        };

        let filtered_calendar = calendar.not_iso_weeks(weeks_to_exclude.clone());

        for (year, month, days) in filtered_calendar.days_calendar {
            let first_day_of_month = Date::from_calendar_date(year as i32, month.to_time_month(), 1).unwrap();
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


}



