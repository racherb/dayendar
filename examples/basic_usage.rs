use dayendar::types::{Month, BiDay};
use dayendar::calendar::{DaysCalendar, replicate};

fn main() {

    let pattern: [BiDay; 2] = [BiDay::One, BiDay::Zero]; // pattern: One day on, the next day off.
    
    let my_calendar = replicate::<BiDay>(&pattern, DaysCalendar::singleton
        ( 2024, Month::January
        ).unwrap()
    );

    println!("\n {:?}\n", my_calendar);

}