use dioxus::prelude::*;
use chrono::NaiveDate;
use crate::models::task::Task;

#[component]
pub fn TaskForm(selected_task: Task, on_save: EventHandler<Task>, on_cancel: EventHandler<()>, engineers: Vec<String>, date: NaiveDate) -> Element {
    let task = use_state(|| selected_task.clone());
    
    let handle_time_change = move |evt: Event<FormData>| {
        task.set(Task {
            time: evt.value.clone(),
            ..task.get().clone()
        });
    };
    
    let handle_engineer_change = move |evt: Event<FormData>| {
        task.set(Task {
            engineer: evt.value.clone(),
            ..task.get().clone()
        });
    };
    
    let handle_contract_change = move |evt: Event<FormData>| {
        task.set(Task {
            contract: evt.value.clone(),
            ..task.get().clone()
        });
    };
    
    let handle_area_change = move |evt: Event<FormData>| {
        task.set(Task {
            area: evt.value.clone(),
            ..task.get().clone()
        });
    };
    
    let handle_address_change = move |evt: Event<FormData>| {
        task.set(Task {
            address: evt.value.clone(),
            ..task.get().clone()
        });
    };
    
    let handle_phone_change = move |evt: Event<FormData>| {
        task.set(Task {
            phone: evt.value.clone(),
            ..task.get().clone()
        });
    };
    
    let handle_description_change = move |evt: Event<FormData>| {
        task.set(Task {
            description: evt.value.clone(),
            ..task.get().clone()
        });
    };
    
    let handle_save = move |_| {
        on_save.call(task.get().clone());
    };
    
    let handle_cancel = move |_| {
        on_cancel.call(());
    };
    
    rsx!{
        div { class: "task-form-overlay",
            div { class: "task-form",
                h2 { "Детали заявки" }
                
                div { class: "form-row",
                    label { "Время:" }
                    select { 
                        value: "{task.time}",
                        onchange: handle_time_change,
                        
                        option { value: "08:00", "08:00" }
                        option { value: "08:30", "08:30" }
                        option { value: "09:00", "09:00" }
                        option { value: "09:30", "09:30" }
                        option { value: "10:00", "10:00" }
                        option { value: "10:30", "10:30" }
                        option { value: "11:00", "11:00" }
                        option { value: "11:30", "11:30" }
                        option { value: "12:00", "12:00" }
                        option { value: "12:30", "12:30" }
                        option { value: "13:00", "13:00" }
                        option { value: "13:30", "13:30" }
                        option { value: "14:00", "14:00" }
                        option { value: "14:30", "14:30" }
                        option { value: "15:00", "15:00" }
                        option { value: "15:30", "15:30" }
                        option { value: "16:00", "16:00" }
                        option { value: "16:30", "16:30" }
                        option { value: "17:00", "17:00" }
                        option { value: "17:30", "17:30" }
                        option { value: "18:00", "18:00" }
                        option { value: "18:30", "18:30" }
                        option { value: "19:00", "19:00" }
                        option { value: "19:30", "19:30" }
                        option { value: "20:00", "20:00" }
                        option { value: "20:30", "20:30" }
                        option { value: "21:00", "21:00" }
                    }
                }
                
                div { class: "form-row",
                    label { "Инженер:" }
                    select { 
                        value: "{task.engineer}",
                        onchange: handle_engineer_change,
                        
                        {engineers.iter().map(|eng| {
                            rsx!{
                                option { value: "{eng}", "{eng}" }
                            }
                        })}
                    }
                }
                
                div { class: "form-row",
                    label { "Договор:" }
                    input { 
                        r#type: "text",
                        value: "{task.contract}",
                        onchange: handle_contract_change
                    }
                }
                
                div { class: "form-row",
                    label { "Район:" }
                    input { 
                        r#type: "text",
                        value: "{task.area}",
                        onchange: handle_area_change
                    }
                }
                
                div { class: "form-row",
                    label { "Адрес:" }
                    input { 
                        r#type: "text",
                        value: "{task.address}",
                        onchange: handle_address_change
                    }
                }
                
                div { class: "form-row",
                    label { "Телефон:" }
                    input { 
                        r#type: "text",
                        value: "{task.phone}",
                        onchange: handle_phone_change
                    }
                }
                
                div { class: "form-row",
                    label { "Описание проблемы:" }
                    textarea { 
                        value: "{task.description}",
                        onchange: handle_description_change
                    }
                }
                
                div { class: "form-actions",
                    button { 
                        onclick: handle_save,
                        "Сохранить"
                    }
                    button { 
                        onclick: handle_cancel,
                        "Отмена"
                    }
                }
            }
        }
    }
}