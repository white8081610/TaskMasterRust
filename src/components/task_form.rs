use dioxus::prelude::*;
use crate::models::task::Task;

#[component]
pub fn TaskForm(
    selected_task: Option<Task>,
    on_submit: EventHandler<Task>,
    on_cancel: EventHandler<()>,
) -> Element {
    let task = use_state(|| selected_task.unwrap_or_default());
    
    let update_time = move |evt: FormEvent| {
        task.update(|t| t.time = evt.value.clone());
    };
    
    let update_contract = move |evt: FormEvent| {
        task.update(|t| t.contract = evt.value.clone());
    };
    
    let update_area = move |evt: FormEvent| {
        task.update(|t| t.area = evt.value.clone());
    };
    
    let update_address = move |evt: FormEvent| {
        task.update(|t| t.address = evt.value.clone());
    };
    
    let update_phone = move |evt: FormEvent| {
        task.update(|t| t.phone = evt.value.clone());
    };
    
    let update_description = move |evt: FormEvent| {
        task.update(|t| t.description = evt.value.clone());
    };
    
    let submit = move |_| {
        on_submit(task.get().clone());
    };
    
    let cancel = move |_| {
        on_cancel(());
    };

    rsx! {
        div { class: "task-form-overlay",
            div { class: "task-form",
                h2 { "Детали заявки" }
                
                div { class: "form-row",
                    label { "Время:" }
                    input {
                        value: "{task.time}",
                        oninput: update_time,
                    }
                }
                
                div { class: "form-row",
                    label { "Договор:" }
                    input {
                        value: "{task.contract}",
                        oninput: update_contract,
                    }
                }
                
                div { class: "form-row",
                    label { "Район:" }
                    input {
                        value: "{task.area}",
                        oninput: update_area,
                    }
                }
                
                div { class: "form-row",
                    label { "Адрес:" }
                    input {
                        value: "{task.address}",
                        oninput: update_address,
                    }
                }
                
                div { class: "form-row",
                    label { "Телефон абонента:" }
                    input {
                        value: "{task.phone}",
                        oninput: update_phone,
                    }
                }
                
                div { class: "form-row",
                    label { "Описание проблемы:" }
                    textarea {
                        value: "{task.description}",
                        oninput: update_description,
                    }
                }
                
                div { class: "form-actions",
                    button { onclick: submit, "Сохранить" }
                    button { onclick: cancel, "Отмена" }
                }
            }
        }
    }
}
