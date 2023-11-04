use std::fmt;
use dayendar::types::{Year, Month, Weekday, BiDay};
use dayendar::abstracto::{AbstractCalendar, CalendarOperation, CalendarPattern};

///Output:
/// Not(
///   CustomDays([1, 25])
///   Or
///   Weekdays([Saturday, Sunday])
///   Or
///   CronPattern("* * * ? * *")
///)
//

fn main() {

    // Usando las funciones auxiliares
    let weekends: AbstractCalendar = AbstractCalendar::Pattern(CalendarPattern::Weekdays(vec![Weekday::Saturday, Weekday::Sunday]));
    let fixed_holidays: AbstractCalendar = AbstractCalendar::Pattern(CalendarPattern::CustomDays(vec![1, 25]));
    let floating_holidays: AbstractCalendar = AbstractCalendar::Pattern(CalendarPattern::CronPattern(String::from("* * * ? * *")));

    // Combina los patrones
    let combined_holidays: AbstractCalendar = fixed_holidays.or(weekends).or(floating_holidays);

    // Negar para obtener días hábiles
    let workdays: AbstractCalendar = combined_holidays.invert();
    
    let as_string: String = format!("{}", workdays);

    println!("{:?}", workdays);
    println!("\n {:?}\n", as_string);

}