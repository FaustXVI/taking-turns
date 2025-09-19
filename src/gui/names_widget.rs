use crate::business::name::Name;
use dioxus::core_macro::{component, Props};
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct NamesWidgetProps {
    onchange: EventHandler<Vec<Name>>,
}

#[component]
pub fn NamesWidget(props: NamesWidgetProps) -> Element {
    let mut names: Signal<Vec<Name>> = use_signal(|| vec![]);
    let new_name: Signal<String> = use_signal(|| "".to_string());
    use_effect(move || props.onchange.call(names.read().clone()));
    rsx! {
        div{
            "Noms :"
        }
        for (i,name) in names().iter().enumerate() {
            div {
                input {
                    value: "{name}",
                    onchange: move |e|{
                        let name: Result<Name,_> = e.value().try_into();
                        if let Ok(name) = name{
                            names.with_mut(|v|v[i] = name);
                        }else{
                            names.with_mut(|v|v.remove(i));
                        }
                    }
                }
            }
        }
        div{
            input {
                    placeholder: "Ajouter quelqu'un",
                    value:"{new_name}",
                    onchange: move |e|{
                        let name: Result<Name,_> = e.value().try_into();
                        if let Ok(name) = name {
                            names.push(name);
                        }
                    }
            }
        }
    }
}
