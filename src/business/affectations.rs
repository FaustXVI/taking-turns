use crate::business::date_range::DateRange;
use crate::business::name::{Name, Names};
use chrono::NaiveDate;

#[derive(Debug, Eq, PartialEq, derive_more::Display)]
#[display("{date}: {name}")]
pub struct Affectation {
    pub date: NaiveDate,
    pub name: Name,
}

pub fn create_affectations(names: Names, date_range: DateRange) -> Vec<Affectation> {
    date_range
        .into_iter()
        .zip(names.into_iter().cycle())
        .map(|(date, name)| Affectation { date, name })
        .collect()
}

#[cfg(test)]
mod affectation_system_should {
    use super::*;
    use crate::business::date_range::test_helpers::date_from;
    use googletest::assert_that;
    use googletest::prelude::container_eq;
    use rstest::rstest;

    #[rstest]
    fn affect_first_name_to_first_date() {
        let names = vec!["Xavier".into()];
        let date_range = ("2025-01-01", "2025-01-01").try_into().unwrap();
        let affectations = create_affectations(names, date_range);
        assert_that!(
            affectations,
            container_eq([Affectation {
                date: date_from("2025-01-01"),
                name: "Xavier".into()
            }])
        )
    }

    #[rstest]
    fn affect_a_name_to_each_dates_by_cycling_over_names() {
        let names = vec!["Xavier".into(), "Merve".into()];
        let date_range = ("2025-01-01", "2025-01-03").try_into().unwrap();
        let affectations = create_affectations(names, date_range);
        assert_that!(
            affectations,
            container_eq([
                Affectation {
                    date: date_from("2025-01-01"),
                    name: "Xavier".into()
                },
                Affectation {
                    date: date_from("2025-01-02"),
                    name: "Merve".into()
                },
                Affectation {
                    date: date_from("2025-01-03"),
                    name: "Xavier".into()
                }
            ])
        )
    }
}
