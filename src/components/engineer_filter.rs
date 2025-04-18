use dioxus::prelude::*;
use crate::models::engineer::Engineer;

#[component]
pub fn EngineerFilter(engineers: Vec<Engineer>, selected: String, on_select: EventHandler<Option<String>>) -> Element {
    rsx!{
        div { class: "engineer-filters",
            // Кнопка "Все инженеры"
            button {
                class: format_args!("engineer-button {}", if selected.as_str() == "Все" { "active" } else { "" }),
                onclick: move |_| on_select.call(None),
                "Все"
            }
            
            // Кнопки для каждого инженера
            {engineers.iter().map(|eng| {
                let name = eng.name.clone();
                let name_str = name.as_str();
                let selected_str = selected.as_str();
                let is_selected = selected_str == name_str;
                
                rsx!{
                    button {
                        key: "{name}",
                        class: format_args!("engineer-button {}", if is_selected { "active" } else { "" }),
                        onclick: move |_| on_select.call(Some(name.clone())),
                        "{name}"
                    }
                }
            })}
        }
    }
}