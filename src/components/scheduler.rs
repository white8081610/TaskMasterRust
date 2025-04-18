use dioxus::prelude::*;
use chrono::NaiveDate;
use crate::models::task::Task;
use crate::components::task_row::TaskRow;

#[component]
pub fn Scheduler(date: NaiveDate, tasks: Vec<Task>, on_task_select: EventHandler<Task>) -> Element {
    // Создаем массив с временными слотами для расписания (с 8:00 до 21:00 с шагом 30 минут)
    let time_slots = vec![
        "08:00", "08:30", "09:00", "09:30", "10:00", "10:30", "11:00", "11:30",
        "12:00", "12:30", "13:00", "13:30", "14:00", "14:30", "15:00", "15:30",
        "16:00", "16:30", "17:00", "17:30", "18:00", "18:30", "19:00", "19:30",
        "20:00", "20:30", "21:00"
    ];
    
    // Состояние для отслеживания выбранной задачи
    let selected_task_id = use_state(|| 0);
    
    // Обработчик выбора задачи
    let handle_task_select = move |task: Task| {
        selected_task_id.set(task.id);
        on_task_select.call(task);
    };
    
    rsx!{
        div { class: "scheduler",
            div { class: "scheduler-header",
                div { class: "time-header", "Время" }
                div { class: "tasks-header", "Заявки" }
            }
            
            div { class: "scheduler-content",
                // Строки расписания для каждого временного слота
                {time_slots.iter().map(|&time| {
                    // Находим задачи для этого временного слота
                    let slot_tasks: Vec<&Task> = tasks.iter()
                        .filter(|task| task.time == time)
                        .collect();
                    
                    rsx!{
                        TaskRow {
                            key: "{time}",
                            time: time.to_string(),
                            tasks: slot_tasks.iter().map(|&&task| task.clone()).collect(),
                            selected_task_id: *selected_task_id.get(),
                            on_task_select: handle_task_select
                        }
                    }
                })}
            }
        }
    }
}