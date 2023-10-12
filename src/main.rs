use dayendar::calendar::*;
use time::macros::date;
use time::Weekday;
use std::iter::FromIterator;
use std::collections::HashSet;

fn main() {
    use std::vec::Vec;
    
    fn create_sample_calendar() -> DaysCalendar<BiDay> {
        let year = 2023;
        let month = Month::March;
        let days = vec![
            BiDay::One, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::One,
            BiDay::Zero, BiDay::One, BiDay::Zero, BiDay::One, BiDay::Zero,
            BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::One,
            BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::One, BiDay::Zero,
            BiDay::One, BiDay::Zero, BiDay::One, BiDay::Zero, BiDay::Zero,
            BiDay::Zero, BiDay::Zero, BiDay::Zero, BiDay::One, BiDay::Zero,
            BiDay::Zero,
        ];
        DaysCalendar {
            days_calendar: vec![(year, month, days)],
        }
    }
    let calendar = create_sample_calendar();
        let mut weekdays = HashSet::new();
        weekdays.insert(Weekday::Monday);
        weekdays.insert(Weekday::Saturday);
        weekdays.insert(Weekday::Friday);

        let new_calendar = calendar.not_weekdays(weekdays);

    print!(">>>>: {:?} \n", calendar);
    print!(">>>>: {:?} \n", new_calendar);


    //print!(">>>>: {:?} \n\n", calendar.previous_day(2020, Month::February, 1));
    //print!(">>>>: {:?}\n", to_date(calendar));

}
