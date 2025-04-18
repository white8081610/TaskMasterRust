use dioxus::prelude::*;
use chrono::{Duration, NaiveDate};
use crate::utils::date_utils::format_date_russian;

#[inline_props]
pub fn Header(cx: Scope, date: NaiveDate, on_date_change: EventHandler<NaiveDate>, selected_engineer: String, operator_name: String) -> Element {
    let handle_prev_day = move |_| {
        let new_date = *date - Duration::days(1);
        on_date_change.call(new_date);
    };
    
    let handle_next_day = move |_| {
        let new_date = *date + Duration::days(1);
        on_date_change.call(new_date);
    };
    
    let handle_prev_week = move |_| {
        let new_date = *date - Duration::days(7);
        on_date_change.call(new_date);
    };
    
    let handle_next_week = move |_| {
        let new_date = *date + Duration::days(7);
        on_date_change.call(new_date);
    };
    
    let formatted_date = format_date_russian(&date);
    let engineer_display = selected_engineer.clone();
    
    cx.render(rsx!{
        div { class: "header",
            div { class: "menu-bar",
                div { class: "menu-item", "Сервис" }
                div { class: "menu-item", "Операции" }
                div { class: "menu-item", "Справка" }
            }
            div { class: "header-content",
                div { class: "engineer-info",
                    span { "Инженер: " }
                    span { class: "engineer-name", "{engineer_display}" }
                }
                div { class: "date-navigation",
                    button { 
                        class: "nav-button",
                        onclick: handle_prev_week,
                        "<<"
                    }
                    button { 
                        class: "nav-button",
                        onclick: handle_prev_day,
                        "<"
                    }
                    div { class: "current-date", "{formatted_date}" }
                    button { 
                        class: "nav-button",
                        onclick: handle_next_day,
                        ">"
                    }
                    button { 
                        class: "nav-button",
                        onclick: handle_next_week,
                        ">>"
                    }
                }
                div { class: "operator-info",
                    span { "оператор: {operator_name}" }
                    button { class: "exit-button", "выйти" }
                }
            }
        }
    })
}