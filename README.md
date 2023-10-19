# Dayendar = Days + Calendar

## Description

Rust Library for Advanced and Efficient Calendar Operations.

![Version](https://img.shields.io/badge/version-0.1.2-blue)
[![Rust Report Card](https://rust-reportcard.xuri.me/badge/github.com/racherb/dayendar)](https://rust-reportcard.xuri.me/report/github.com/racherb/dayendar)
[![dependency status](https://deps.rs/repo/github/racherb/dayendar/status.svg)](https://deps.rs/repo/github/racherb/dayendar)
[![coverage](https://shields.io/endpoint?url=https://racherb.github.io/dayendar/coverage.json)](https://racherb.github.io/dayendar/index.html)

## Features

- Flexible calendar representation as DaysCalendar<T> allows modeling days in different forms (BiDay, Day, etc).
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
use calendar::{DaysCalendar, BiDay};

let weekdays = DaysCalendar::singleton(2022, 1)
    .and_weekdays(vec![Weekday::Mon, Weekday::Tue])
    .biday_to_vec_day();

println!("Weekdays in January 2022: {:?}", weekdays);

```

## Documentation

Full documentation can be found at [https://docs.rs/dayendar](https://docs.rs/dayendar)

## Contributing

PRs are welcome! Please open an issue first to discuss any major changes.

If you would like to contribute to the development of "**Dayendar**", we encourage you to do so!

Please read the contribution guidelines defined in [CONTRIBUTING](CONTRIBUTING.md) before submitting your changes, read also our governance guideline [GOBERNANCE](GOBERNANCE.md) and our code of conduct [CODE OF CONDUCT](CODE_OF_CONDUCT.md).

Please note that contributions must be signed. Sign commits by adding the ```-S`` option when executing:

 ```bash
 git commit -S -m "Your comment"
 ```

Important: To sign your contributions you will need to set up a GPG key and configure Git as described in the [GENERATING GPG KEY](GENERATING_GPG_KEY.md) document.

## License

Apache-2
