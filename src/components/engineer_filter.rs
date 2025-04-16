use dioxus::prelude::*;
use crate::models::engineer::Engineer;

#[component]
pub fn EngineerFilter(
    engineers: Vec<Engineer>,
    selected: Option<String>,
    on_select: EventHandler<Option<String>>,
) -> Element {
    rsx! {
        div { class: "engineer-filter",
            button {
                class: format_args!("engineer-button {}", if selected.is_none() { "active" } else { "" }),
                onclick: move |_| on_select(None),
                "Все"
            }
            
            {engineers.iter().map(|eng| {
                let eng_name = eng.name.clone();
                let is_selected = selected.as_ref().map_or(false, |s| s == &eng.name);
                rsx! {
                    button {
                        class: format_args!("engineer-button {}", if is_selected { "active" } else { "" }),
                        onclick: move |_| on_select(Some(eng_name.clone())),
                        background_color: if is_selected { "#FFFF00" } else { "#F0F0F0" },
                        "{eng.name}"
                    }
                }
            })}
        }
    }
}
