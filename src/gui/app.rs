use crate::business::date_range::{DateRange, DateRangeError};
use chrono::{NaiveDate, Utc};
use dioxus::logger::tracing;
use dioxus::prelude::*;
use std::ops::Deref;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

#[component]
pub fn App() -> Element {
    let mut range: Signal<Option<DateRange>> = use_signal(|| None);
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        h1{
            "🎉 Taking turns ! 🎉"
        }
        DateRangeWidget {
            onchange: move |new_range|{
                range.set(new_range);
            }
        }
        if range().is_some() {
            {format!("Selected range : {:?}",range().unwrap())}
        }
    }
}

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
    use_effect(move || props.onchange.call(range_option.read().clone()));
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
