use dioxus::prelude::*;
use crate::components::{header::Header, scheduler::Scheduler, task_form::TaskForm};
use crate::models::task::Task;
use crate::models::engineer::Engineer;
use chrono::{NaiveDate, Local};
use std::sync::Arc;

// Main application state
pub struct AppState {
    pub selected_date: NaiveDate,
    pub selected_task: Option<Task>,
    pub selected_engineer: Option<String>,
    pub engineers: Vec<Engineer>,
    pub tasks: Vec<Task>,
    pub is_editing: bool,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            selected_date: Local::now().date_naive(),
            selected_task: None,
            selected_engineer: None,
            engineers: vec![
                Engineer { name: "Попов".to_string() },
                Engineer { name: "Соломаха".to_string() },
                Engineer { name: "Ежиков".to_string() },
            ],
            tasks: Vec::new(),
            is_editing: false,
        }
    }
}

// Main App component
#[component]
pub fn App() -> Element {
    let state = use_ref(AppState::default);
    
    // Load initial tasks
    use_effect(move || {
        to_owned![state];
        async move {
            match fetch_tasks(&state.read().selected_date).await {
                Ok(tasks) => {
                    state.write().tasks = tasks;
                }
                Err(err) => {
                    eprintln!("Failed to fetch tasks: {}", err);
                }
            }
        }
    });
    
    let date_changed = move |new_date: NaiveDate| {
        let mut state = state.write();
        state.selected_date = new_date;
        state.selected_task = None;
        // We'll need to fetch tasks for the new date in a real implementation
    };

    let engineer_selected = move |engineer_name: Option<String>| {
        state.write().selected_engineer = engineer_name;
    };

    let task_selected = move |task: Option<Task>| {
        state.write().selected_task = task;
    };

    let edit_mode = move |mode: bool| {
        state.write().is_editing = mode;
    };

    let add_task = move |task: Task| {
        state.write().tasks.push(task);
        // In a real app, we'd save this to the database
    };

    let update_task = move |task: Task| {
        let mut state = state.write();
        if let Some(idx) = state.tasks.iter().position(|t| t.id == task.id) {
            state.tasks[idx] = task;
        }
        // In a real app, we'd update the database
    };

    let state_clone = state.clone();
    
    rsx! {
        div { class: "app",
            Header {
                date: state.read().selected_date,
                on_date_change: date_changed,
                engineers: state.read().engineers.clone(),
                selected_engineer: state.read().selected_engineer.clone(),
                on_engineer_select: engineer_selected,
                operator: "Лемесев Д.С.".to_string()
            }
            
            Scheduler {
                tasks: state.read().tasks.clone(),
                selected_date: state.read().selected_date,
                selected_engineer: state.read().selected_engineer.clone(),
                on_task_select: task_selected,
                on_edit_mode: edit_mode
            }
            
            if state.read().is_editing {
                TaskForm {
                    selected_task: state.read().selected_task.clone(),
                    on_submit: move |task: Task| {
                        if task.id == 0 {
                            add_task(task);
                        } else {
                            update_task(task);
                        }
                        edit_mode(false);
                    },
                    on_cancel: move || edit_mode(false)
                }
            }
        }
    }
}

async fn fetch_tasks(date: &NaiveDate) -> Result<Vec<Task>, reqwest::Error> {
    // In a real app, we'd fetch tasks from the API
    // For now, return some example tasks based on the screenshots
    
    let formatted_date = date.format("%Y-%m-%d").to_string();
    let client = reqwest::Client::new();
    let response = client.get(format!("http://localhost:8000/api/tasks?date={}", formatted_date))
        .send()
        .await?;
    
    let tasks = response.json::<Vec<Task>>().await?;
    Ok(tasks)
}
