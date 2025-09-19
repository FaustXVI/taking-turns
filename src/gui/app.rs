use crate::business::date_range::DateRange;
use crate::business::name::Name;
use crate::gui::date_range_widget::DateRangeWidget;
use crate::gui::names_widget::NamesWidget;
use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

#[component]
pub fn App() -> Element {
    let mut range: Signal<Option<DateRange>> = use_signal(|| None);
    let mut names: Signal<Vec<Name>> = use_signal(|| vec![]);
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

        if range().is_some() {
            div {{format!("Selected range : {:?}",range().unwrap())}}
        }
        for name in names(){
            div{ "{name}"}
        }
    }
}
