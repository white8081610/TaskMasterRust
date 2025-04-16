use dioxus::prelude::*;
use chrono::{NaiveDate, Duration};
use crate::models::engineer::Engineer;

#[component]
pub fn Header(
    date: NaiveDate,
    on_date_change: EventHandler<NaiveDate>,
    engineers: Vec<Engineer>,
    selected_engineer: Option<String>,
    on_engineer_select: EventHandler<Option<String>>,
    operator: String,
) -> Element {
    let formatted_date = date.format("ВС.%d.%m.%Y").to_string();
    
    let go_prev_day = move |_| {
        on_date_change(date - Duration::days(1));
    };
    
    let go_next_day = move |_| {
        on_date_change(date + Duration::days(1));
    };
    
    let go_prev_week = move |_| {
        on_date_change(date - Duration::days(7));
    };
    
    let go_next_week = move |_| {
        on_date_change(date + Duration::days(7));
    };
    
    rsx! {
        div { class: "header",
            div { class: "menu-bar",
                div { class: "menu-item", "Сервис" }
                div { class: "menu-item", "Операции" }
                div { class: "menu-item", "Справка" }
            }
            
            div { class: "header-content",
                div { class: "engineer-info",
                    "Инженер: ",
                    if let Some(eng) = &selected_engineer {
                        span { class: "engineer-name", "{eng}" }
                    } else {
                        span { class: "engineer-name", "Все" }
                    }
                }
                
                div { class: "date-navigation",
                    button { class: "nav-button", onclick: go_prev_week, "<<" }
                    button { class: "nav-button", onclick: go_prev_day, "<" }
                    div { class: "current-date", "{formatted_date}" }
                    button { class: "nav-button", onclick: go_next_day, ">" }
                    button { class: "nav-button", onclick: go_next_week, ">>" }
                }
                
                div { class: "operator-info",
                    "оператор: {operator}",
                    button { class: "exit-button", "выйти" }
                }
            }
            
            div { class: "engineer-filters",
                button {
                    class: format_args!("engineer-button {}", if selected_engineer.is_none() { "active" } else { "" }),
                    onclick: move |_| on_engineer_select(None),
                    "Все"
                }
                
                {engineers.iter().map(|eng| {
                    let eng_name = eng.name.clone();
                    let is_selected = selected_engineer.as_ref().map_or(false, |s| s == &eng.name);
                    rsx! {
                        button {
                            class: format_args!("engineer-button {}", if is_selected { "active" } else { "" }),
                            onclick: move |_| on_engineer_select(Some(eng_name.clone())),
                            "{eng.name}"
                        }
                    }
                })}
            }
        }
    }
}
