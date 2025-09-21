use crate::business::affectations::create_affectations;
use crate::business::date_range::DateRange;
use crate::business::day_filter::{DayFilter, FilterDays};
use crate::business::name::Names;
use crate::gui::affectations_widget::AffectationsWidget;
use crate::gui::date_range_widget::DateRangeWidget;
use crate::gui::names_widget::NamesWidget;
use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

#[component]
pub fn App() -> Element {
    let mut range: Signal<Option<DateRange>> = use_signal(|| None);
    let mut names: Signal<Names> = use_signal(|| vec![]);
    let days_filter: Signal<DayFilter> = use_signal(|| DayFilter::default());
    let affectations = use_memo(move || {
        if let Some(range) = range() {
            create_affectations(names(), range.into_iter().filter_days(&days_filter()))
        } else {
            vec![]
        }
    });

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        h1{
            "🦋 Taking turns ! 🦋"
        }
        DateRangeWidget {
            onchange: move |new_range|{
                range.set(new_range);
            }
        }

        NamesWidget{
            onchange: move |new_names|{
                names.set(new_names)
            }
        }
        AffectationsWidget {
            affectations: affectations.read().clone()
        }
    }
}
