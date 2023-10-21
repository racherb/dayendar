use dayendar::types::{Month, Weekday};
use dayendar::calendar::{
    DaysCalendar,
    from_day,
    biday_to_vec_day
};

use std::collections::HashSet;

/// Jhon must come to the office on Mondays and Thursdays during the month of January 2023.
/// Except on days when his boss has a board meeting, 
/// How do you plan ahead for John's visits to the office during January?
/// Note: Please note that the 30th and 31st are public holidays.
fn main() {

    let mut jhon_base_calendar = HashSet::new();
    jhon_base_calendar.insert(Weekday::Monday);
    jhon_base_calendar.insert(Weekday::Tuesday);

    let boss_board_meeting = vec![(2023, Month::January, vec![9, 11, 13, 16, 23, 24, 28])];

    let boss_board_meeting_calendar = from_day(
        DaysCalendar {
            days_calendar: boss_board_meeting
        }
    );

    let fest_days = from_day(
        DaysCalendar {
            days_calendar: vec![(2023, Month::January, vec![25, 30, 31])]
        }
    );

    let jhon_office_boss_schedule = DaysCalendar::singleton(2023, Month::January).unwrap()
      .and_weekdays(jhon_base_calendar)
      .minus(&boss_board_meeting_calendar)
      .minus(&fest_days);

    let forecast = biday_to_vec_day(jhon_office_boss_schedule.clone());
    
    println!("\nForecast: Calendar of available options\n {:?}\n", forecast);

}