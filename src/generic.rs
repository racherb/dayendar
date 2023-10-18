/// The `types` module provides the general data types used in the system
pub mod types {

    /// Represents the months of the year and comes directly from `time::Month`.
    pub use time::Month as TimeMonth;
    pub use time::{Date as Date, Duration as Duration, Weekday as Weekday};
    /// The macro `date`. Comes directly from `time::macros::date`.
    pub use time::macros::date;

    /// Represents the year
    pub type Year = u16;
    /// Represents the day of the month
    pub type Day = u8;

    /// Is the value of the minimum date allowed. It comes from the data type `time::Date`.
    pub const MIN_DATE: Date = date!(0001 - 01 - 01);
    /// Is the value of the maximun date allowed. It comes from the data type `time::Date`.
    pub const MAX_DATE: Date = date!(9999 - 12 - 31);

    /// Enumerator `Month` representing the months of the year
    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[repr(u8)]
    pub enum Month {
        January,
        February,
        March,
        April,
        May,
        June,
        July,
        August,
        September,
        October,
        November,
        December,
    }

    impl Month {
        /// `to_index` function takes the name of a given month and returns the number of the month
        pub fn to_index(&self) -> u8 {
            *self as u8 + 1
        }
        /// `from_index` returns the name of the month from its numerical representation
        pub fn from_index(index: u8) -> Option<Self> {
            match index {
                1 => Some(Month::January),
                2 => Some(Month::February),
                3 => Some(Month::March),
                4 => Some(Month::April),
                5 => Some(Month::May),
                6 => Some(Month::June),
                7 => Some(Month::July),
                8 => Some(Month::August),
                9 => Some(Month::September),
                10 => Some(Month::October),
                11 => Some(Month::November),
                12 => Some(Month::December),
                _ => None,
            }
        }
        /// Converts `Month` type to `TimeMonth` type
        pub fn to_time_month(&self) -> TimeMonth {
            TimeMonth::try_from(self.to_index()).expect("Invalid month index")
        }
    }

    /// Enumerator representing the binary value of the day
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum BiDay {
        /// The value `Zero` means that the item does not included in the calendar
        Zero,
        /// The value `One` means that the item is included in the calendar.
        One,
    }

    impl BiDay {
        /// Convert types from BiDay to u8
        pub fn to_u8(&self) -> u8 {
            match self {
                BiDay::Zero => 0,
                BiDay::One => 1,
            }
        }
        /// Convert types from u8 to BiDay
        pub fn from_u8(value: u8) -> Option<BiDay> {
            match value {
                0 => Some(BiDay::Zero),
                1 => Some(BiDay::One),
                _ => None,
            }
        }
    }


    #[cfg(test)]
    mod tests_types {
        use super::*;

        #[test]
        fn month_to_index_test() {
            assert_eq!(Month::January.to_index(), 1);
            assert_eq!(Month::December.to_index(), 12);
        }

        #[test]
        fn month_from_index_test() {
            assert_eq!(Month::from_index(1), Some(Month::January));
            assert_eq!(Month::from_index(12), Some(Month::December));
            assert_eq!(Month::from_index(13), None);
        }

        #[test]
        fn month_from_index_outbound_no_panic_test() {
            assert_eq!(Month::from_index(13), None);
        }

        #[test]
        fn biday_to_u8_test() {
            assert_eq!(BiDay::Zero.to_u8(), 0);
            assert_eq!(BiDay::One.to_u8(), 1);
        }

        #[test]
        fn biday_from_u8_test() {
            assert_eq!(BiDay::from_u8(0), Some(BiDay::Zero));
            assert_eq!(BiDay::from_u8(1), Some(BiDay::One));
            assert_eq!(BiDay::from_u8(2), None);
        }
    }

}

