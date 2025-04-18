use dioxus::prelude::*;
use crate::models::engineer::Engineer;

#[inline_props]
pub fn EngineerFilter(cx: Scope, engineers: Vec<Engineer>, selected: String, on_select: EventHandler<Option<String>>) -> Element {
    cx.render(rsx!{
        div { class: "engineer-filters",
            // Кнопка "Все инженеры"
            button {
                class: format_args!("engineer-button {}", if selected == "Все" { "active" } else { "" }),
                onclick: move |_| on_select.call(None),
                "Все"
            }
            
            // Кнопки для каждого инженера
            {engineers.iter().map(|eng| {
                let name = eng.name.clone();
                let is_selected = selected == name;
                
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
    })
}