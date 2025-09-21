use crate::business::affectations::Affectation;
use dioxus::prelude::*;

#[component]
pub fn AffectationsWidget(affectations: Vec<Affectation>) -> Element {
    rsx! {
        for affectation in affectations {
            div {
                    {affectation.date.format("%d/%m/%Y").to_string()} " : {affectation.name}"
            }
        }
    }
}
