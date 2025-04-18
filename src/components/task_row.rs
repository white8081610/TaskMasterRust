use dioxus::prelude::*;
use crate::models::task::Task;

#[inline_props]
pub fn TaskRow(cx: Scope, task: Task, on_select: EventHandler<Task>) -> Element {
    let handle_click = move |_| {
        on_select.call(task.clone());
    };
    
    cx.render(rsx!{
        div { 
            class: "task-row",
            onclick: handle_click,
            
            div { class: "task-time", "{task.time}" }
            div { class: "task-contract", "{task.contract}" }
            div { class: "task-area", "{task.area}" }
            div { class: "task-address", "{task.address}" }
            div { class: "task-phone", "{task.phone}" }
            div { class: "task-description", "{task.description}" }
        }
    })
}