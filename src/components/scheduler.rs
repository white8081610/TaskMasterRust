use dioxus::prelude::*;
use chrono::NaiveDate;
use crate::models::task::Task;
use crate::components::task_row::TaskRow;

#[inline_props]
pub fn Scheduler(cx: Scope, date: NaiveDate, tasks: Vec<Task>, on_task_select: EventHandler<Task>) -> Element {
    // Создаем временные слоты с 8:00 до 21:00 с интервалом 30 минут
    let time_slots = vec![
        "08:00", "08:30", "09:00", "09:30", "10:00", "10:30", "11:00", "11:30",
        "12:00", "12:30", "13:00", "13:30", "14:00", "14:30", "15:00", "15:30",
        "16:00", "16:30", "17:00", "17:30", "18:00", "18:30", "19:00", "19:30",
        "20:00", "20:30", "21:00"
    ];
    
    cx.render(rsx!{
        div { class: "scheduler",
            // Заголовок таблицы
            div { class: "scheduler-header",
                div { "время" }
                div { "договор" }
                div { "район" }
                div { "адрес" }
                div { "тел. абонента" }
                div { "описание проблемы" }
            }
            
            // Содержимое таблицы
            div { class: "scheduler-content",
                // Для каждого временного слота
                {time_slots.iter().map(|slot| {
                    let slot_tasks: Vec<&Task> = tasks.iter()
                        .filter(|task| task.time == *slot)
                        .collect();
                    
                    // Если нет задач в этом слоте, показываем пустую строку
                    if slot_tasks.is_empty() {
                        rsx!{
                            div { 
                                class: "task-row",
                                key: "{slot}",
                                div { class: "task-time", "{slot}" }
                                div { class: "task-contract" }
                                div { class: "task-area" }
                                div { class: "task-address" }
                                div { class: "task-phone" }
                                div { class: "task-description" }
                            }
                        }
                    } else {
                        // Иначе показываем все задачи на этот временной слот
                        rsx!{
                            {slot_tasks.iter().map(|&task| {
                                rsx!{
                                    TaskRow {
                                        key: "{task.id}",
                                        task: task.clone(),
                                        on_select: on_task_select.clone()
                                    }
                                }
                            })}
                        }
                    }
                })}
            }
        }
    })
}