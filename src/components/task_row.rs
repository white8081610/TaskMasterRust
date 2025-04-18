use dioxus::prelude::*;
use crate::models::task::Task;
use crate::models::task::TaskStatus;

#[component]
pub fn TaskRow(time: String, tasks: Vec<Task>, selected_task_id: u64, on_task_select: EventHandler<Task>) -> Element {
    rsx!{
        div { class: "time-row",
            // Ячейка с временем
            div { class: "time-cell", "{time}" }
            
            // Ячейка с задачами
            div { class: "tasks-cell",
                {tasks.iter().map(|task| {
                    let is_selected = task.id == *selected_task_id;
                    let status_class = match task.status {
                        TaskStatus::Pending => "status-pending",
                        TaskStatus::InProgress => "status-in-progress",
                        TaskStatus::Completed => "status-completed",
                        TaskStatus::Cancelled => "status-cancelled",
                    };
                    
                    let task_clone = task.clone();
                    
                    rsx!{
                        div {
                            key: "{task.id}",
                            class: format_args!("task-card {} {}", status_class, if is_selected { "selected" } else { "" }),
                            onclick: move |_| on_task_select.call(task_clone.clone()),
                            
                            div { class: "task-header",
                                div { class: "task-engineer", "{task.engineer}" }
                                div { class: "task-address", "{task.address}" }
                            }
                            div { class: "task-description", "{task.description}" }
                        }
                    }
                })}
            }
        }
    }
}