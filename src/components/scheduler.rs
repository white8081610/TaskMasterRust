use dioxus::prelude::*;
use chrono::NaiveDate;
use crate::models::task::Task;
use crate::components::task_row::TaskRow;

#[component]
pub fn Scheduler(
    tasks: Vec<Task>,
    selected_date: NaiveDate,
    selected_engineer: Option<String>,
    on_task_select: EventHandler<Option<Task>>,
    on_edit_mode: EventHandler<bool>,
) -> Element {
    // Filter tasks by selected engineer if needed
    let filtered_tasks = if let Some(eng_name) = selected_engineer {
        tasks.iter()
            .filter(|task| task.engineer == eng_name)
            .cloned()
            .collect::<Vec<Task>>()
    } else {
        tasks.clone()
    };
    
    // Create time slots for the scheduler
    let time_slots = (9..21).flat_map(|hour| {
        vec![
            format!("{:02}:00", hour),
            format!("{:02}:30", hour)
        ]
    }).collect::<Vec<String>>();

    let create_new_task = move |_| {
        on_task_select(Some(Task::default()));
        on_edit_mode(true);
    };

    let edit_task = move |task: Task| {
        on_task_select(Some(task));
        on_edit_mode(true);
    };

    rsx! {
        div { class: "scheduler",
            // Table header
            div { class: "scheduler-header",
                div { class: "time-header", "время" }
                div { class: "contract-header", "договор" }
                div { class: "area-header", "район" }
                div { class: "address-header", "адрес" }
                div { class: "phone-header", "тел. абонента" }
                div { class: "description-header", "описание проблемы" }
            }
            
            // Scheduler rows
            div { class: "scheduler-content",
                {time_slots.iter().map(|time_slot| {
                    let time = time_slot.clone();
                    let time_task = filtered_tasks.iter()
                        .find(|task| task.time == time)
                        .cloned();
                    
                    rsx! {
                        TaskRow {
                            time: time.clone(),
                            task: time_task,
                            on_select: move |t| edit_task(t),
                        }
                    }
                })}
            }
            
            // Action buttons
            div { class: "action-buttons",
                button { 
                    class: "action-button new-task", 
                    onclick: create_new_task,
                    "новая заявка" 
                }
                button { class: "action-button edit-task", "изменение заявки" }
                button { class: "action-button move-task", "перенос заявки" }
                button { class: "action-button print", "печать" }
                button { class: "action-button history", "история операций" }
                button { class: "action-button edit-time", "редактировать рабочее время" }
            }
        }
    }
}
