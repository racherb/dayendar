# Dayendar = Days + Calendar

## Description

Rust Library for Advanced and Efficient Calendar Operations.

![Version](https://img.shields.io/badge/version-0.1.2-blue)
[![Rust Report Card](https://rust-reportcard.xuri.me/badge/github.com/racherb/dayendar)](https://rust-reportcard.xuri.me/report/github.com/racherb/dayendar)
[![dependency status](https://deps.rs/repo/github/racherb/dayendar/status.svg)](https://deps.rs/repo/github/racherb/dayendar)

## Why Dayendar

There are three reasons:

- There is currently no robust and complete library for working with calendars in Rust that uses the DaysCalendar structure. This makes it a unique and valuable solution.
- It provides high-level operators and utilities specific for calendar manipulation that are not available in standard libraries.
- Its DaysCalendar representation and data structures are optimized for performance in common calendar tasks like combining, filtering, finding days, etc.

## Features

- Flexible calendar representation as **DaysCalendar<T>** allows modeling days in different forms (BiDay, Day, etc).
- Powerful operators to combine and transform calendars efficiently. Allows creating complex calendars from simpler ones.
- Advanced filtering by ISO week, weekday, etc. Useful for applications like schedules, availability, etc.
- Contains utilities for normalizing, inverting, finding previous/next days that ease common calendar manipulation tasks.
- Designed with Rust, focused on performance and safety. Use of data structures like Vec, HashSet, BTreeMap provides efficiency.
- Well documented with Doc comments. Easy to understand and extend.
- Open source, implementations can be inspected and contributed to.

## Instalation

With Rust Cargo:

```bash
cargo add dayendar
```

Or add the following line to your `Cargo.toml` file:

```toml
[dependencies]
dayendar = "0.1.2"
```

## Usage

```rust
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
```

## Example

**Case**: Jhon must come to the office on Mondays and Thursdays during the month of January 2023. Except on days when his boss has a board meeting. How do you plan ahead for John's visits to the office during January? Please note that the 30th and 31st are public holidays.

**Rust Solution using Struct DaysCalendar**:

```rust
use dayendar::types::{Month, Weekday};
use dayendar::calendar::{
    DaysCalendar,
    from_day,
    biday_to_vec_day
};

use std::collections::HashSet;

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

    let jhon_office_visit_schedule = DaysCalendar::singleton(2023, Month::January).unwrap()
      .and_weekdays(jhon_base_calendar)
      .minus(&boss_board_meeting_calendar)
      .minus(&fest_days);

    let forecast = biday_to_vec_day(jhon_office_visit_schedule.clone());
    
    println!("\nForecast: Calendar of available options\n {:?}\n", forecast);

}

```

To see more examples, explore the "examples" folder of the project or simply run:

```bash
cargo run --example <example_name>
```

## Documentation

Full documentation can be found at [https://docs.rs/dayendar](https://docs.rs/dayendar)

## Contributing

### Collaborate with us, you're welcome

PRs are welcome! Please open an issue first to discuss any major changes.

If you would like to contribute to the development of "**Dayendar**", we encourage you to do so!

Please read the contribution guidelines defined in [CONTRIBUTING](CONTRIBUTING.md) before submitting your changes, read also our governance guideline [GOBERNANCE](GOBERNANCE.md) and our code of conduct [CODE OF CONDUCT](CODE_OF_CONDUCT.md).

Please note that contributions must be signed. Sign commits by adding the `-S` option when executing `commit`:

 ```bash
 git commit -S -m "Your comment"
 ```

Important: To sign your contributions you will need to set up a GPG key and configure Git as described in the [GENERATING GPG KEY](GENERATING_GPG_KEY.md) document.

### We are looking for new maintainers

If you want to be part of Dayendar's team of maintainers then generate a PR in the document [OWNERS.md](OWNERS.md) and leave us your details. We will contact you as soon as possible.

## License

Apache-2
