use dioxus::prelude::*;
use chrono::{Local, NaiveDate};
use crate::components::header::Header;
use crate::components::engineer_filter::EngineerFilter;
use crate::components::scheduler::Scheduler;
use crate::components::task_form::TaskForm;
use crate::models::task::Task;
use crate::models::engineer::Engineer;
use web_sys::{window, Storage};
use gloo_console::log;

fn get_local_storage() -> Option<Storage> {
    window()
        .and_then(|window| window.local_storage().ok())
        .flatten()
}

pub fn app(cx: Scope) -> Element {
    let today = Local::now().date_naive();
    let current_date = use_state(cx, || today);
    let selected_engineer = use_state(cx, || None::<String>);
    let show_form = use_state(cx, || false);
    let selected_task = use_state(cx, || None::<Task>);
    
    // Имитируем инженеров и задачи из локального хранилища (в реальности должны быть из API)
    let engineers = use_state(cx, || vec![
        Engineer::new("Попов".to_string()),
        Engineer::new("Соломаха".to_string()),
        Engineer::new("Ежиков".to_string()),
    ]);
    
    let tasks = use_state(cx, Vec::<Task>::new);
    
    // При изменении даты загружаем задачи
    let load_tasks = move || {
        let date = *current_date.get();
        let engineer = selected_engineer.get().clone();
        
        spawn(async move {
            match fetch_tasks(date, engineer).await {
                Ok(loaded_tasks) => {
                    tasks.set(loaded_tasks);
                }
                Err(e) => {
                    log!("Error loading tasks:", e.to_string());
                }
            }
        });
    };
    
    // Загружаем задачи при первом рендере
    let is_first_render = use_state(cx, || true);
    if *is_first_render.get() {
        is_first_render.set(false);
        load_tasks();
    }
    
    let handle_task_select = move |task: Task| {
        selected_task.set(Some(task));
        show_form.set(true);
    };
    
    let handle_new_task = move |_| {
        selected_task.set(None);
        show_form.set(true);
    };
    
    let handle_edit_task = move |_| {
        if selected_task.get().is_some() {
            show_form.set(true);
        }
    };
    
    let handle_form_save = move |task: Task| {
        let task_clone = task.clone();
        spawn(async move {
            match save_task(task_clone).await {
                Ok(_) => {
                    load_tasks();
                    show_form.set(false);
                }
                Err(e) => {
                    log!("Error saving task:", e.to_string());
                }
            }
        });
    };
    
    let handle_form_cancel = move |_| {
        show_form.set(false);
    };
    
    let handle_date_change = move |date: NaiveDate| {
        current_date.set(date);
        // Загружаем задачи на новую дату
        load_tasks();
    };
    
    let handle_engineer_select = move |eng: Option<String>| {
        selected_engineer.set(eng);
        // Загружаем задачи для выбранного инженера
        load_tasks();
    };
    
    cx.render(rsx!{
        div { class: "app",
            header {
                date: *current_date.get(),
                on_date_change: handle_date_change,
                selected_engineer: selected_engineer.get().clone().unwrap_or_else(|| "Все".to_string()),
                operator_name: "Лемесев Д.С.".to_string()
            }
            
            engineer_filter {
                engineers: engineers.get().clone(),
                selected: selected_engineer.get().clone().unwrap_or_else(|| "Все".to_string()),
                on_select: handle_engineer_select
            }
            
            scheduler {
                date: *current_date.get(),
                tasks: tasks.get().clone(),
                on_task_select: handle_task_select
            }
            
            div { class: "action-buttons",
                button {
                    class: "action-button new-task",
                    onclick: handle_new_task,
                    "новая заявка"
                }
                
                button {
                    class: "action-button edit-task",
                    onclick: handle_edit_task,
                    disabled: selected_task.get().is_none(),
                    "изменение заявки"
                }
                
                button {
                    class: "action-button move-task",
                    disabled: selected_task.get().is_none(),
                    "перенос заявки"
                }
                
                button {
                    class: "action-button print",
                    "печать"
                }
                
                button {
                    class: "action-button history",
                    "история операций"
                }
                
                button {
                    class: "action-button edit-time",
                    "редактировать рабочее время"
                }
            }
            
            // Форма для создания/редактирования задачи
            {show_form.get().then(|| {
                let task = selected_task.get().clone().unwrap_or_else(|| {
                    Task::new(
                        *current_date.get(),
                        "09:00".to_string(),
                        engineers.get().first().map(|e| e.name.clone()).unwrap_or_default(),
                        "".to_string(),
                        "".to_string(),
                        "".to_string(),
                        "".to_string(),
                        "".to_string()
                    )
                });
                
                rsx!{
                    task_form {
                        selected_task: task,
                        on_save: handle_form_save,
                        on_cancel: handle_form_cancel,
                        engineers: engineers.get().iter().map(|e| e.name.clone()).collect(),
                        date: *current_date.get()
                    }
                }
            })}
        }
    })
}

// Загрузка задач (имитация запроса к API)
async fn fetch_tasks(date: NaiveDate, engineer: Option<String>) -> Result<Vec<Task>, String> {
    // В реальности, это был бы запрос к API, но пока используем локальное хранилище
    if let Some(storage) = get_local_storage() {
        let key = format!("tasks_{}", date.format("%Y%m%d"));
        if let Ok(Some(data)) = storage.get_item(&key) {
            if let Ok(mut tasks) = serde_json::from_str::<Vec<Task>>(&data) {
                // Фильтруем по инженеру, если указан
                if let Some(eng) = engineer {
                    tasks.retain(|t| t.engineer == eng);
                }
                return Ok(tasks);
            }
        }
    }
    
    // Если ничего не найдено, возвращаем пустой список
    Ok(Vec::new())
}

// Сохранение задачи (имитация запроса к API)
async fn save_task(task: Task) -> Result<Task, String> {
    // В реальности, это был бы запрос к API, но пока используем локальное хранилище
    if let Some(storage) = get_local_storage() {
        let key = format!("tasks_{}", task.date.format("%Y%m%d"));
        
        // Загружаем существующие задачи
        let mut tasks = match storage.get_item(&key) {
            Ok(Some(data)) => {
                match serde_json::from_str::<Vec<Task>>(&data) {
                    Ok(t) => t,
                    Err(_) => Vec::new(),
                }
            },
            _ => Vec::new(),
        };
        
        // Определяем, новая ли это задача или обновление существующей
        if task.id == 0 {
            // Новая задача: генерируем ID и добавляем
            let new_id = (tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1).max(1);
            let mut new_task = task;
            new_task.id = new_id;
            tasks.push(new_task.clone());
            
            // Сохраняем обновленный список
            if let Ok(data) = serde_json::to_string(&tasks) {
                if storage.set_item(&key, &data).is_ok() {
                    return Ok(new_task);
                }
            }
        } else {
            // Обновление существующей: находим по ID и заменяем
            if let Some(pos) = tasks.iter().position(|t| t.id == task.id) {
                tasks[pos] = task.clone();
                
                // Сохраняем обновленный список
                if let Ok(data) = serde_json::to_string(&tasks) {
                    if storage.set_item(&key, &data).is_ok() {
                        return Ok(task);
                    }
                }
            }
        }
    }
    
    Err("Не удалось сохранить задачу".to_string())
}