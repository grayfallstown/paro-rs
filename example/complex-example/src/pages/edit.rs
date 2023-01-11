
use std::sync::{Arc, RwLock};

use maud::html;

use paro_rs::*;

use crate::state::*;
use crate::pages::*;
use crate::router::*;

pub struct EditState {
    pub employee: Option<Employee>,
    validation: EmployeeValidation,
}

impl EditState {
    pub fn default() -> EditState {
        EditState {
            employee: None,
            validation: EmployeeValidation::default(),
        }
    }
}


pub fn render_edit(paro_app: &mut Arc<RwLock<ParoApp<ApplicationState>>>) -> String {
    let mut write_lock = paro_app.write().unwrap();
    let state = &mut write_lock.state;
    println!("xcxxxxxxxxxxxxxxxxxxxxx");
    let edit_state = &mut state.edit_state;

    if let Some(employee) = edit_state.employee.as_ref() {
        edit_state.validation.validate(employee, &state.employees, false);
    }

    let on_first_name_input = event!(paro_app, (move |state: &mut ApplicationState, value: Option<String>| {
        let edit_state = &mut state.edit_state;
        if let Some(employee) = edit_state.employee.as_mut() {
            employee.first_name = if value.is_some() { value.unwrap().trim().to_owned() } else { "".to_owned() };
            employee.login = format!("{}.{}", &employee.first_name, &employee.last_name);
            edit_state.validation.validate(&employee, &state.employees, false);    
        }
    }));

    let on_last_name_input = event!(paro_app, (move |state: &mut ApplicationState, value: Option<String>| {
        let edit_state = &mut state.edit_state;
        if let Some(employee) = edit_state.employee.as_mut() {
            employee.last_name = if value.is_some() { value.unwrap().trim().to_owned() } else { "".to_owned() };
            employee.login = format!("{}.{}", &employee.first_name, &employee.last_name);
            edit_state.validation.validate(&employee, &state.employees, false);
        }
    }));

    let on_login_input = event!(paro_app, (move |state: &mut ApplicationState, value: Option<String>| {
        let edit_state = &mut state.edit_state;
        if let Some(employee) = edit_state.employee.as_mut() {
            employee.login = if value.is_some() { value.unwrap().trim().to_owned() } else { "".to_owned() };
            edit_state.validation.validate(&employee, &state.employees, false);
        }
    }));

    let on_submit = event!(paro_app, (move |state: &mut ApplicationState, _value: Option<String>| {
        let edit_state = &mut state.edit_state;
        if edit_state.validation.is_valid() {
            if let Some(employee) = edit_state.employee.take() {
                state.employees.push(Arc::new(employee.clone()));
                state.list_state.filter_employees(&state.employees);
                state.page = Page::List;
            }
        }
    }));

    let content = html! {
        h1 {
            "Edit member"
        }
        
        form.row."g-3"."was-validated" {

            @if let Some(employee) = edit_state.employee.as_ref() {
                (render_form_fields(
                    employee,
                    &edit_state.validation,
                    on_first_name_input,
                    on_last_name_input,
                    on_login_input,
                ))
            }

            div."col-12" {
                button.btn."btn-primary" type="submit" onclick=({on_submit}) {
                    "Save Changes"
                }
            }
        }
    };
    // we need to release the write_lock here, as render_layout tries to get a write_lock as well.
    drop(write_lock);
    let markup = render_layout(&mut paro_app.clone(), content);
    markup
}

