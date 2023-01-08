use std::sync::{Arc, RwLock};
use maud::{html, Markup};
use paro_rs::*;
use uuid;

use crate::state::*;
use crate::pages::render_layout;


pub struct AddState {
    employee: Employee,
    first_name_validation: Option<String>,
    last_name_validation: Option<String>,
    login_validation: Option<String>,
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
            first_name_validation: Some("Required.".to_owned()),
            last_name_validation: Some("Required.".to_owned()),
            login_validation: Some("Required.".to_owned()),
        }
    }
}

pub fn render_add(paro_app: &mut Arc<RwLock<ParoApp<ApplicationState>>>) -> String {
    fn validate_login(add_state: &mut AddState, employees: &Vec<Arc<Employee>>) {
        let login = add_state.employee.login.trim();
        if login.is_empty() || login == "." {
            add_state.login_validation = Some("Please provide a valid username".to_owned());
        } else {
            if employees.into_iter().any(|employee| employee.login == login) {
                add_state.login_validation = Some("This username is taken".to_owned());
            } else {
                add_state.login_validation = None;
            }
        }
    }

    // nothing stops us from defining events outside of the template to keep
    // out html generation cleaner.
    let on_first_name_input = event!(paro_app, (move |state: &mut ApplicationState, value: Option<String>| {
        let mut add_state = &mut state.add_state;
        add_state.employee.first_name = if value.is_some() { value.unwrap().trim().to_owned() } else { "".to_owned() };
        if add_state.employee.first_name.is_empty() {
            add_state.first_name_validation = None;
        } else {
            add_state.last_name_validation = Some("Required".to_owned());
        }
        add_state.employee.login = format!("{}.{}", &add_state.employee.first_name, &add_state.employee.last_name);
        validate_login(add_state, &state.employees);
    }));

    let on_last_name_input = event!(paro_app, (move |state: &mut ApplicationState, value: Option<String>| {
        let mut add_state = &mut state.add_state;
        add_state.employee.last_name = if value.is_some() { value.unwrap().trim().to_owned() } else { "".to_owned() };
        if add_state.employee.last_name.is_empty() {
            add_state.last_name_validation = None;
        } else {
            add_state.last_name_validation = Some("Required".to_owned());
        }
        add_state.employee.login = format!("{}.{}", &add_state.employee.first_name, &add_state.employee.last_name);
        validate_login(add_state, &state.employees);
    }));

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

    let add_state = &paro_app.read().unwrap().state.add_state;

    let content = html! {
        h1 {
            "Add a new team member"
        }
        
        form.row."g-3"."was-validated" {
            div."col-md-6" {
                label."form-label" for="validationCustom01" {
                    "First Name"
                }
                input #validationCustom01."form-control" value=(add_state.employee.first_name) required type="text"
                    oninput=({on_first_name_input}) {
                }
                ({render_validation(&add_state.first_name_validation)})
            }
            div."col-md-6" {
                label."form-label" for="validationCustom02" {
                    "Last name"
                }
                input #validationCustom02."form-control" value=(add_state.employee.last_name) required type="text"
                    oninput=({on_last_name_input}) {
                }
                (render_validation(&add_state.last_name_validation))
            }
            div."col-md-6" {
                label."form-label" for="validationCustom03" {
                    "Username"
                }
                input #validationCustom03."form-control" value=(add_state.employee.login) required type="text" {
                }
                (render_validation(&add_state.login_validation))
            }
            div."col-md-6" {
                label."form-label" for="validationCustom04" {
                    "Department"
                }
                select #validationCustom04."form-select" required {
                    @for department in Department::into_iter() {
                        option selected[add_state.employee.department == *department] value=(department) {
                            (format!("{:?}", department))
                        }
                    }
                }
            }
            div."col-12" {
                button.btn."btn-primary" type="submit" disabled {
                    "Submit form (not yet implemented)"
                }
            }
        }
    };
    let markup = render_layout(&mut paro_app.clone(), content);
    markup
}
