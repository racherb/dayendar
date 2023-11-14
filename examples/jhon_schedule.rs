use dayendar::types::{Month, Weekday};
use dayendar::calendar::{
    DaysCalendar,
    from_day,
    biday_to_vec_day
};
use std::collections::HashSet;

/// Determine which days John should come to the office in January 2023.
/// John usually comes on Mondays and Thursdays.
/// He should avoid days when his boss has board meetings.
/// Also, consider the hackathon days on 30th and 31st January 2023.
fn main() {
    // Days of the week John usually comes to the office.
    let mut john_work_days = HashSet::new();
    john_work_days.insert(Weekday::Monday);
    john_work_days.insert(Weekday::Thursday);

    // Days when John's boss has board meetings.
    let boss_meeting_days = vec![(2023, Month::January, vec![9, 11, 13, 16, 23, 24, 28])];
    let boss_meetings_calendar = from_day(
        DaysCalendar {
            days_calendar: boss_meeting_days
        }
    );

    // Hackathon days in January 2023.
    let hackathon_days = from_day(
        DaysCalendar {
            days_calendar: vec![(2023, Month::January, vec![30, 31])]
        }
    );

    // Determine John's office days considering his workdays, boss's meeting days, and hackathon days.
    let john_office_days = DaysCalendar::singleton(2023, Month::January).unwrap()
      .and_weekdays(john_work_days).expect("Invalid")
      .minus(&boss_meetings_calendar)
      .minus(&hackathon_days);

    // Convert the final calendar to a vector of days.
    let office_days_list = biday_to_vec_day(john_office_days.clone());
    
    println!("\nJohn's Office Days in January 2023:\n {:?}\n", office_days_list);
}
