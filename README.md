# Dayendar = Days + Calendar

## Description

Rust Library for Advanced and Efficient Calendar Operations.

![Version](https://img.shields.io/badge/version-0.1.2-blue)
[![Rust Report Card](https://rust-reportcard.xuri.me/badge/github.com/racherb/dayendar)](https://rust-reportcard.xuri.me/report/github.com/racherb/dayendar)
[![dependency status](https://deps.rs/repo/github/racherb/dayendar/status.svg)](https://deps.rs/repo/github/racherb/dayendar)
[![codecov](https://codecov.io/gh/racherb/dayendar/graph/badge.svg?token=B5lReInEZW)](https://codecov.io/gh/racherb/dayendar)

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

**Case**: Determine which days John should come to the office in January 2023. John usually comes on Mondays and Thursdays. He should avoid days when his boss has board meetings. Also, consider the hackathon days on 30th and 31st January 2023.

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
      .and_weekdays(john_work_days)
      .minus(&boss_meetings_calendar)
      .minus(&hackathon_days);

    // Convert the final calendar to a vector of days.
    let office_days_list = biday_to_vec_day(john_office_days.clone());
    
    println!("\nJohn's Office Days in January 2023:\n {:?}\n", office_days_list);
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

This project is licensed under the **Apache 2.0 License**. It is a permissive license that is widely used in open-source software projects. It allows users to use, modify, and distribute the software in any way they wish, as long as they comply with the conditions set by the license. For more information, please refer to the Apache 2.0 License at [LICENSE](LICENSE).
