use crate::business::date_range::{DateRange, DateRangeError};
use chrono::Utc;
use dioxus::core_macro::Props;
use dioxus::dioxus_core::Element;
use dioxus::hooks::{use_effect, use_memo, use_signal};
use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct DateRangeWidgetProps {
    onchange: EventHandler<Option<DateRange>>,
}

#[component]
pub fn DateRangeWidget(props: DateRangeWidgetProps) -> Element {
    let mut starting_date: Signal<String> = use_signal(|| Utc::now().date_naive().to_string());
    let mut ending_date: Signal<String> = use_signal(|| Utc::now().date_naive().to_string());
    let range = use_memo(move || (starting_date().as_str(), ending_date().as_str()).try_into());
    let range_option = use_memo(move || range.read().clone().ok());
    use_effect(move || props.onchange.call(*range_option.read()));
    rsx! {
        div{
            label { for:"starting_date", "Du"}
            input {
                type: "date",
                id:"starting_date",
                value : "{starting_date}",
                onchange: move |e| {
                    starting_date.set(e.value());
                }
            }
            if let Err(DateRangeError::StartDateFormatWrong(_)) = *range.read(){
                div{"Date de début invalide"}
            }
        }
        div{
            label { for:"ending_date", "Au"}
            input {
                type: "date",
                id:"ending_date",
                value : "{ending_date}",
                onchange: move |e| {
                    ending_date.set(e.value());
                }
            }
            if let Err(DateRangeError::EndDateFormatWrong(_)) = *range.read(){
                div{"Date de fin invalide"}
            }
        }
        if let Err(DateRangeError::StartDateAfterEndDate(_,_)) = *range.read(){
                div{"Date de début après la date de fin"}
        }
    }
}
