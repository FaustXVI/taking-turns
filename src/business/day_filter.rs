use chrono::{Datelike, NaiveDate, Weekday};

#[derive(Debug, Copy, Clone)]
pub struct DayFilter {}

impl Default for DayFilter {
    fn default() -> Self {
        DayFilter {}
    }
}

impl DayFilter {
    pub fn removed_days(&self) -> Vec<Weekday> {
        vec![Weekday::Sat, Weekday::Sun]
    }
}

#[derive(Clone, Copy, Debug)]
pub struct FilterDaysIterator<I> {
    iterator: I,
    filter: DayFilter,
}

impl<I> Iterator for FilterDaysIterator<I>
where
    I: Iterator<Item = NaiveDate>,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator
            .find(|date| !self.filter.removed_days().contains(&date.weekday()))
    }
}

pub trait FilterDays
where
    Self: Sized,
{
    fn filter_days(self, day_filter: &DayFilter) -> FilterDaysIterator<Self> {
        FilterDaysIterator {
            iterator: self,
            filter: *day_filter,
        }
    }
}

impl<T> FilterDays for T where T: Iterator<Item = NaiveDate> {}

#[cfg(test)]
mod day_filter_should {
    use super::*;
    use chrono::{NaiveDate, Weekday};
    use googletest::assert_that;
    use googletest::prelude::container_eq;
    use rstest::rstest;

    #[rstest]
    fn filter_out_weekend_days_by_default() {
        let filter = DayFilter::default();
        assert_that!(
            filter.removed_days(),
            container_eq([Weekday::Sat, Weekday::Sun])
        )
    }

    #[rstest]
    fn can_be_used_to_filter_an_iterator() {
        let filter = DayFilter::default();
        let dates = vec![
            NaiveDate::from_isoywd_opt(2025, 2, Weekday::Fri).unwrap(),
            NaiveDate::from_isoywd_opt(2025, 2, Weekday::Sat).unwrap(),
            NaiveDate::from_isoywd_opt(2025, 2, Weekday::Sun).unwrap(),
        ];

        assert_that!(
            dates.into_iter().filter_days(&filter).collect::<Vec<_>>(),
            container_eq([NaiveDate::from_isoywd_opt(2025, 2, Weekday::Fri).unwrap()])
        )
    }
}
