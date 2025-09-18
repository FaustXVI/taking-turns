use chrono::{NaiveDate, Utc};
use dioxus::prelude::*;
const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

#[component]
pub fn App() -> Element {
    let mut date_str: Signal<String> = use_signal(|| Utc::now().date_naive().to_string());
    let date: Memo<Option<NaiveDate>> =
        use_memo(move || NaiveDate::parse_from_str(&date_str(), "%Y-%m-%d").ok());
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        h1{
            "🎉 Taking turns ! 🎉"
        }
        input {
            type: "date",
            value : "{date_str}",
            onchange: move |e| {
                date_str.set(e.value());
            }
        }
        if let Some(d) = date() {
            div {
                "Selected date : {d}"
            }
        }
    }
}
