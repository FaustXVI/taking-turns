use chrono::Weekday::*;
use chrono::{Datelike, NaiveDate, Weekday};
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct WeekDayFilter {
    accepted_days: HashSet<Weekday>,
}

impl Default for WeekDayFilter {
    fn default() -> Self {
        WeekDayFilter {
            accepted_days: HashSet::from([Mon, Tue, Wed, Thu, Fri]),
        }
    }
}

impl WeekDayFilter {
    pub fn accepted_days(&self) -> Vec<Weekday> {
        self.accepted_days.iter().copied().collect()
    }

    pub fn toggle(self, day: Weekday) -> WeekDayFilter {
        let mut accepted_days = self.accepted_days;
        if !accepted_days.remove(&day) {
            accepted_days.insert(day);
        }
        WeekDayFilter { accepted_days }
    }
}

#[derive(Clone, Debug)]
pub struct FilterByWeekDayIterator<I> {
    iterator: I,
    filter: WeekDayFilter,
}

impl<I> Iterator for FilterByWeekDayIterator<I>
where
    I: Iterator<Item = NaiveDate>,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator
            .find(|date| self.filter.accepted_days().contains(&date.weekday()))
    }
}

pub trait FilterByWeekDays
where
    Self: Sized,
{
    fn filter_by_weekday(self, day_filter: &WeekDayFilter) -> FilterByWeekDayIterator<Self>;
}

impl<T> FilterByWeekDays for T
where
    T: Iterator<Item = NaiveDate>,
{
    fn filter_by_weekday(self, day_filter: &WeekDayFilter) -> FilterByWeekDayIterator<Self> {
        FilterByWeekDayIterator {
            iterator: self,
            filter: day_filter.clone(),
        }
    }
}

#[cfg(test)]
mod day_filter_should {
    use super::*;
    use chrono::NaiveDate;
    use googletest::assert_that;
    use googletest::matchers::*;
    use rstest::rstest;

    #[rstest]
    fn filter_out_weekend_days_by_default() {
        let filter = WeekDayFilter::default();
        assert_that!(
            filter.accepted_days(),
            unordered_elements_are![eq(&Mon), eq(&Tue), eq(&Wed), eq(&Thu), eq(&Fri)]
        )
    }

    #[rstest]
    fn can_toggle_out_a_day() {
        let filter = WeekDayFilter::default();
        let filter = filter.toggle(Mon);
        assert_that!(
            filter.accepted_days(),
            unordered_elements_are![eq(&Tue), eq(&Wed), eq(&Thu), eq(&Fri)]
        )
    }

    #[rstest]
    fn can_toggle_in_a_day() {
        let filter = WeekDayFilter::default();
        let filter = filter.toggle(Sun);
        assert_that!(
            filter.accepted_days(),
            unordered_elements_are![eq(&Mon), eq(&Tue), eq(&Wed), eq(&Thu), eq(&Fri), eq(&Sun)]
        )
    }

    #[rstest]
    fn can_be_used_to_filter_an_iterator() {
        let filter = WeekDayFilter::default();
        let dates = vec![
            NaiveDate::from_isoywd_opt(2025, 2, Fri).unwrap(),
            NaiveDate::from_isoywd_opt(2025, 2, Sat).unwrap(),
            NaiveDate::from_isoywd_opt(2025, 2, Sun).unwrap(),
        ];

        assert_that!(
            dates
                .into_iter()
                .filter_by_weekday(&filter)
                .collect::<Vec<_>>(),
            container_eq([NaiveDate::from_isoywd_opt(2025, 2, Fri).unwrap()])
        )
    }
}
