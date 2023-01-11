use std::sync::{Arc, RwLock};
use maud::{html, Markup};
use paro_rs::*;
use uuid;

use crate::state::*;
use crate::router::*;
use crate::pages::render_layout;


pub struct AddState {
    employee: Employee,
    validation: EmployeeValidation,
}

impl AddState {
    pub fn default() -> AddState {
        AddState {
            employee: Employee {
                id: uuid::Uuid::new_v4().to_string(),
                first_name: "".to_owned(),
                last_name: "".to_owned(),
                login: "".to_owned(),
                department: Department::Production,
            },
            validation: EmployeeValidation::default(),
        }
    }
}

fn render_validation(validation: &Option<String>) -> Markup {
    if validation.is_none() {
        html! {
            div."valid-feedback" {
                "Looks good!"
            }    
        }
    } else {
        html! {
            div."invalid-feedback" {
                (validation.as_ref().unwrap())
            }
        }
    }
}


pub fn render_form_fields(
    employee: &Employee,
    validation: &EmployeeValidation,
    on_first_name_input: String,
    on_last_name_input: String,
    on_login_input: String,
) -> Markup {
    html! {
        div."col-md-6" {
            label."form-label" for="validationCustom01" {
                "First Name"
            }
            input #validationCustom01."form-control" value=(employee.first_name) required type="text"
                oninput=({on_first_name_input}) {
            }
            ({render_validation(&validation.first_name)})
        }
        div."col-md-6" {
            label."form-label" for="validationCustom02" {
                "Last name"
            }
            input #validationCustom02."form-control" value=(employee.last_name) required type="text"
                oninput=({on_last_name_input}) {
            }
            (render_validation(&validation.last_name))
        }
        div."col-md-6" {
            label."form-label" for="validationCustom03" {
                "Username"
            }
            input #validationCustom03."form-control" value=(employee.login) required type="text"
                oninput=({on_login_input}) {
            }
            (render_validation(&validation.login))
        }
        div."col-md-6" {
            label."form-label" for="validationCustom04" {
                "Department"
            }
            select #validationCustom04."form-select" required {
                @for department in Department::into_iter() {
                    option selected[employee.department == *department] value=(department) {
                        (format!("{:?}", department))
                    }
                }
            }
        }
    }

}


pub fn render_add(paro_app: &mut Arc<RwLock<ParoApp<ApplicationState>>>) -> String {

    let on_first_name_input = event!(paro_app, (move |state: &mut ApplicationState, value: Option<String>| {
        let mut add_state = &mut state.add_state;
        add_state.employee.first_name = if value.is_some() { value.unwrap().trim().to_owned() } else { "".to_owned() };
        add_state.employee.login = format!("{}.{}", &add_state.employee.first_name, &add_state.employee.last_name);
        add_state.validation.validate(&add_state.employee, &state.employees, false);
    }));

    let on_last_name_input = event!(paro_app, (move |state: &mut ApplicationState, value: Option<String>| {
        let mut add_state = &mut state.add_state;
        add_state.employee.last_name = if value.is_some() { value.unwrap().trim().to_owned() } else { "".to_owned() };
        add_state.employee.login = format!("{}.{}", &add_state.employee.first_name, &add_state.employee.last_name);
        add_state.validation.validate(&add_state.employee, &state.employees, false);
    }));

    let on_login_input = event!(paro_app, (move |state: &mut ApplicationState, value: Option<String>| {
        let mut add_state = &mut state.add_state;
        add_state.employee.login = if value.is_some() { value.unwrap().trim().to_owned() } else { "".to_owned() };
        add_state.validation.validate(&add_state.employee, &state.employees, false);
    }));

    let on_submit = event!(paro_app, (move |state: &mut ApplicationState, _value: Option<String>| {
        let mut add_state = &mut state.add_state;
        if add_state.validation.is_valid() {
            state.employees.push(Arc::new(add_state.employee.clone()));
            add_state.employee = Employee::default();
            add_state.validation.validate(&add_state.employee, &state.employees, false);
            state.list_state.filter_employees(&state.employees);
            state.page = Page::List;
        }
    }));

    let add_state = &paro_app.read().unwrap().state.add_state;

    let content = html! {
        h1 {
            "Add a new team member"
        }
        
        form.row."g-3"."was-validated" {

            (render_form_fields(
                &add_state.employee,
                &add_state.validation,
                on_first_name_input,
                on_last_name_input,
                on_login_input,
            ))

            div."col-12" {
                button.btn."btn-primary" type="submit" onclick=({on_submit}) {
                    "Add to list"
                }
            }
        }
    };
    let markup = render_layout(&mut paro_app.clone(), content);
    markup
}
