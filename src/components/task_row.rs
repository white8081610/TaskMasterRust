use dioxus::prelude::*;
use crate::models::task::Task;

#[component]
pub fn TaskRow(
    time: String,
    task: Option<Task>,
    on_select: EventHandler<Task>,
) -> Element {
    let background_color = match &task {
        Some(t) => {
            if t.time.starts_with("13:") {
                "#FFAAAA" // Red for 13:00-13:59 (lunch break)
            } else {
                "#D0F0D0" // Light green for normal tasks
            }
        },
        None => "white", // No task
    };
    
    let on_row_click = move |_| {
        if let Some(task) = task.clone() {
            on_select(task);
        }
    };

    rsx! {
        div { 
            class: "task-row",
            style: format_args!("background-color: {background_color}"),
            onclick: on_row_click,
            
            div { class: "task-time", "{time}" }
            
            if let Some(t) = &task {
                div { class: "task-contract", "{t.contract}" }
                div { class: "task-area", "{t.area}" }
                div { class: "task-address", "{t.address}" }
                div { class: "task-phone", "{t.phone}" }
                div { class: "task-description", "{t.description}" }
            } else {
                div { class: "task-contract" }
                div { class: "task-area" }
                div { class: "task-address" }
                div { class: "task-phone" }
                div { class: "task-description" }
            }
        }
    }
}
