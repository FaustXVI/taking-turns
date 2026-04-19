use crate::business::date_range::DateRange;
use crate::business::excluded_period_filter::ExcludedPeriodsFilter;
use crate::business::name::{Name, Names};
use crate::gui::date_range_widget::DateRangeWidget;
use dioxus::core_macro::{component, Props};
use dioxus::dioxus_core::Element;
use dioxus::html::completions::CompleteWithBraces::form;
use dioxus::prelude::*;

#[component]
pub fn ExcludedPeriodsWidget(excluded_period_filter: Signal<ExcludedPeriodsFilter>) -> Element {
    let mut current_exclusion: Signal<Option<DateRange>> = use_signal(|| None);
    let mut excluded: Signal<Vec<DateRange>> = use_signal(Vec::default);
    use_effect(move || {
        excluded_period_filter.set(ExcludedPeriodsFilter::from(excluded.read().clone()))
    });
    rsx! {
        for (i,range) in excluded.iter().enumerate() {
            div {
                {format!("sauf du {} au {}", range.starting_date().format("%d/%m/%Y").to_string(), range.ending_date().format("%d/%m/%Y").to_string())}
                " "
                button {
                onclick:  move |_| {
                    excluded.with_mut(move |v| v.remove(i));
                },
                "Supprimer"
            }
            }
        }
        div{

        div{
            "Exclure :"
        }
        DateRangeWidget {
            onchange: move |new_range|{
                current_exclusion.set(new_range)
            }
        }
            button {
                onclick: move |_| {
                    if let Some(range) = current_exclusion.read().clone() {
                    excluded.with_mut(move |v| v.push(range))
                    }
                },
                "Ajouter"
            }

        }
    }
}
