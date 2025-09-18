use chrono::NaiveDate;

#[derive(Debug, Eq, PartialEq)]
pub struct DateRange {
    starting_date: NaiveDate,
    ending_date: NaiveDate,
}

#[derive(thiserror::Error, Debug)]
pub enum DateRangeError {
    #[error("Wrong format for starting date {0}")]
    StartDateFormatWrong(String),
    #[error("Wrong format for ending date {0}")]
    EndDateFormatWrong(String),
    #[error("Starting date {0} is after ending date {1}")]
    StartDateAfterEndDate(NaiveDate, NaiveDate),
}
impl TryFrom<(&str, &str)> for DateRange {
    type Error = DateRangeError;

    fn try_from((starting_date, ending_date): (&str, &str)) -> Result<Self, Self::Error> {
        let starting_date = NaiveDate::parse_from_str(starting_date, "%Y-%m-%d")
            .map_err(|_| DateRangeError::StartDateFormatWrong(starting_date.to_string()))?;
        let ending_date = NaiveDate::parse_from_str(ending_date, "%Y-%m-%d")
            .map_err(|_| DateRangeError::EndDateFormatWrong(ending_date.to_string()))?;
        if starting_date > ending_date {
            Err(DateRangeError::StartDateAfterEndDate(
                starting_date,
                ending_date,
            ))
        } else {
            Ok(DateRange {
                starting_date,
                ending_date,
            })
        }
    }
}

#[cfg(test)]
mod date_range_should {
    use crate::business::date_range::{DateRange, DateRangeError};
    use chrono::NaiveDate;
    use core::str::FromStr;
    use googletest::assert_that;
    use googletest::matchers::*;
    use rstest::*;

    #[rstest]
    #[case::big_difference("0001-01-01", "3000-12-30")]
    #[case::one_day_difference("2025-01-01", "2025-01-02")]
    #[case::same_day("2025-01-01", "2025-01-01")]
    fn build_from_two_date_as_string(#[case] starting_date: &str, #[case] ending_date: &str) {
        let result: Result<DateRange, _> = (starting_date, ending_date).try_into();
        assert_that!(
            result,
            ok(eq(&DateRange {
                starting_date: NaiveDate::from_str(starting_date).unwrap(),
                ending_date: NaiveDate::from_str(ending_date).unwrap()
            }))
        );
    }

    #[rstest]
    #[case::wrong_separation("2025/01/01")]
    #[case::swap_day_and_month("2025-13-01")]
    fn fails_to_build_when_starting_date_is_badly_formatted(#[case] starting_date: &str) {
        let result: Result<DateRange, _> = (starting_date, "3000-12-30").try_into();
        assert_that!(
            result,
            err(pat!(DateRangeError::StartDateFormatWrong(eq(
                starting_date
            ))))
        );
    }

    #[rstest]
    #[case::wrong_separation("2025/01/01")]
    #[case::swap_day_and_month("2025-13-01")]
    fn fails_to_build_when_ending_date_is_badly_formatted(#[case] ending_date: &str) {
        let result: Result<DateRange, _> = ("0001-01-01", ending_date).try_into();
        assert_that!(
            result,
            err(pat!(DateRangeError::EndDateFormatWrong(eq(ending_date))))
        );
    }

    #[rstest]
    #[case::big_difference("3000-12-30", "0001-01-01")]
    #[case::one_day_difference("2025-01-02", "2025-01-01")]
    fn fails_to_build_when_starting_date_is_after_ending_date(
        #[case] starting_date: &str,
        #[case] ending_date: &str,
    ) {
        let result: Result<DateRange, _> = (starting_date, ending_date).try_into();

        let starting_date = NaiveDate::from_str(starting_date).unwrap();
        let ending_date = NaiveDate::from_str(ending_date).unwrap();
        assert_that!(
            result,
            err(pat!(DateRangeError::StartDateAfterEndDate(
                eq(&starting_date),
                eq(&ending_date)
            )))
        );
    }
}
