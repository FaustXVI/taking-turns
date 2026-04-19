use crate::business::date_range::DateRange;
use chrono::NaiveDate;
use std::collections::{BTreeSet, HashSet};

#[derive(Debug, Clone)]
pub struct ExcludedPeriodsFilter {
    excluded_periods: BTreeSet<DateRange>,
}

impl Default for ExcludedPeriodsFilter {
    fn default() -> Self {
        ExcludedPeriodsFilter {
            excluded_periods: BTreeSet::from([]),
        }
    }
}

impl From<Vec<DateRange>> for ExcludedPeriodsFilter {
    fn from(value: Vec<DateRange>) -> Self {
        ExcludedPeriodsFilter {
            excluded_periods: BTreeSet::from_iter(value.into_iter()),
        }
    }
}

impl ExcludedPeriodsFilter {
    pub fn add(self, period: DateRange) -> Self {
        let mut excluded_periods = self.excluded_periods;
        excluded_periods.insert(period);
        Self { excluded_periods }
    }

    pub fn remove(self, period: &DateRange) -> Self {
        let mut excluded_periods = self.excluded_periods;
        excluded_periods.remove(period);
        Self { excluded_periods }
    }
}

#[derive(Clone, Debug)]
pub struct ExcludedPeriodsFilterIterator<I> {
    iterator: I,
    filter: ExcludedPeriodsFilter,
}

impl<I> Iterator for ExcludedPeriodsFilterIterator<I>
where
    I: Iterator<Item = NaiveDate>,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator.find(|day| {
            !self
                .filter
                .excluded_periods
                .iter()
                .any(|excluded| excluded.contains(&day))
        })
    }
}

pub trait ExcludePeriods
where
    Self: Sized,
{
    fn exclude_period(
        self,
        day_filter: &ExcludedPeriodsFilter,
    ) -> ExcludedPeriodsFilterIterator<Self>;
}

impl<T> ExcludePeriods for T
where
    T: Iterator<Item = NaiveDate>,
{
    fn exclude_period(
        self,
        day_filter: &ExcludedPeriodsFilter,
    ) -> ExcludedPeriodsFilterIterator<Self> {
        ExcludedPeriodsFilterIterator {
            iterator: self,
            filter: day_filter.clone(),
        }
    }
}

#[cfg(test)]
mod day_filter_should {
    use super::*;
    use crate::business::date_range::test_helpers::date_from;
    use googletest::assert_that;
    use googletest::matchers::*;
    use rstest::rstest;

    #[rstest]
    fn can_add_a_period() {
        let filter = ExcludedPeriodsFilter::default();
        let filter = filter.add(DateRange::try_from(("2025-01-01", "2025-01-01")).unwrap());
        assert_that!(filter.excluded_periods.len(), eq(1))
    }

    #[rstest]
    fn can_remove_a_period() {
        let filter = ExcludedPeriodsFilter::default();
        let filter = filter.add(DateRange::try_from(("2025-01-01", "2025-01-01")).unwrap());
        let filter = filter.remove(&DateRange::try_from(("2025-01-01", "2025-01-01")).unwrap());
        assert_that!(filter.excluded_periods.len(), eq(0))
    }

    #[rstest]
    fn can_be_used_to_filter_an_iterator() {
        let filter = ExcludedPeriodsFilter::default();
        let filter = filter.add(DateRange::try_from(("2025-01-09", "2025-01-11")).unwrap());
        let dates = vec![date_from("2025-01-06"), date_from("2025-01-10")];

        assert_that!(
            dates
                .into_iter()
                .exclude_period(&filter)
                .collect::<Vec<_>>(),
            container_eq([date_from("2025-01-06")])
        )
    }
}
