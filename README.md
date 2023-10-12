# Dayendar

![Version](https://img.shields.io/badge/version-0.1.1-blue)

Welcome to the "Dayendar" crate. This is version 0.1.1 and it's currently in active development.

Dayendar implements advanced operations for working with calendars in Rust. The most important parts I highlight are:

- It allows creating DaysCalendar calendars from different data sources such as date vectors, individual months, binary patterns, etc.

- It provides functions to query and extract information from calendars such as years, months and present days.

- It has ```or```, ```and```, ```not```, ```match```, ```nomatch``` and others operations to modify calendars or combine calendars, as well as filtering by days of the week or ISO weeks, adding days, replicating patterns, etc.

- Conversion functions between different calendar and date representations (DaysCalendar, Date vectors, BiDay and Day days).

In general, it allows to create and manipulate calendars in a flexible and powerful way, performing operations both at bit level (BiDay) and with more abstract representations (Date dates).

## ⚠️ Warning ⚠️

The API of "Dayendar" is unstable at this stage. It is under active development, which means there might be significant changes at any time.

### Use at Your Own Risk

By using this crate, you acknowledge that there are associated risks due to its instability. Should you choose to use it, it is at your own discretion and responsibility. It's recommended not to use it in production environments until the API becomes stable.

## Installation

To add "Dayendar" to your project, add the following line to your `Cargo.toml` file:

```toml
[dependencies]
dayendar = "0.1.0"
```

## Contributions
If you wish to contribute to the development of "Dayendar", we encourage you to do so! Please read the contribution guidelines before submitting your changes.
