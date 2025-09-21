use crate::business::weekday_filter::WeekDayFilter;
use chrono::Weekday;
use chrono::Weekday::*;
use dioxus::core_macro::component;
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;

fn to_french(day: Weekday) -> &'static str {
    match day {
        Mon => "Lundi",
        Tue => "Mardi",
        Wed => "Mercredi",
        Thu => "Jeudi",
        Fri => "Vendredi",
        Sat => "Samedi",
        Sun => "Dimanche",
    }
}
#[component]
pub fn WeekDayFilterWidget(weekday_filter: Signal<WeekDayFilter>) -> Element {
    let filters = weekday_filter.read().removed_days();
    let days = [Mon, Tue, Wed, Thu, Fri, Sat, Sun];
    rsx! {
        for day in days{
            div{
                input {
                    type: "checkbox",
                    checked: !filters.contains(&day),
                    disabled: true
                }
                {to_french(day)}
            }
        }
    }
}
