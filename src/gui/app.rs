use crate::business::date_range::DateRange;
use crate::gui::date_range_widget::DateRangeWidget;
use dioxus::prelude::*;

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
