
use std::sync::{Arc, RwLock};

use maud::html;

use paro_rs::*;

use crate::state::*;
use crate::pages::*;
use crate::router::*;
use crate::data_generation::generate_employees;


pub struct ListState {
    pub filtered_employees: Vec<Arc<Employee>>,
    pub search_term: String,
    pub sort_direction: Direction,
    pub sort_by: EmployeeField,
}

impl ListState {
    pub fn default(employees: &Vec<Arc<Employee>>) -> ListState {
        let mut result = ListState {
            filtered_employees: vec![],
            search_term: "".to_owned(),
            sort_direction: Direction::Asc,
            sort_by: EmployeeField::Login,
        };
        result.filter_employees(employees);
        result
    }

    pub fn sort_by(&mut self, field: EmployeeField, employees: &Vec<Arc<Employee>>) {
        if self.sort_by == field {
            if self.sort_direction == Direction::Asc {
                self.sort_direction = Direction::Dsc;
            } else {
                self.sort_direction = Direction::Asc;
            }
        } else {
            self.sort_by = field;
            self.sort_direction = Direction::Asc;
        }
        self.filter_employees(employees);
    }

    pub fn filter_employees(&mut self, employees: &Vec<Arc<Employee>>) -> () {
        let mut filtered_employees: Vec<Arc<Employee>> = employees
            .into_iter()
            .filter(|employee| {
                employee
                    .first_name
                    .contains(&self.search_term)
                    || employee.last_name.contains(&self.search_term)
                    || employee.login.contains(&self.search_term)
                    || format!("{:?}", employee.department).contains(&self.search_term)
            })
            .map(|employee_arc| employee_arc.clone())
            .collect();

        match &self.sort_by {
            EmployeeField::FirstName => filtered_employees
                .sort_by(|a, b| a.first_name.cmp(&b.first_name)),
            EmployeeField::LastName => filtered_employees
                .sort_by(|a, b| a.last_name.cmp(&b.last_name)),
            EmployeeField::Login => filtered_employees.sort_by(|a, b| a.login.cmp(&b.login)),
            EmployeeField::Department => filtered_employees
                .sort_by(|a, b| format!("{:?}", a.department).cmp(&format!("{:?}", b.department))),
        }

        if let Direction::Dsc = self.sort_direction {
            filtered_employees.reverse();
        }

        self.filtered_employees = filtered_employees;
    }
}


pub fn render_list(paro_app: &mut Arc<RwLock<ParoApp<ApplicationState>>>) -> String {
    let state = &paro_app.read().unwrap().state;
    let list_state = &state.list_state;

    let create_edit_event = |employee: Arc<Employee>| {
        event!(paro_app, (move |state: &mut ApplicationState, _value: Option<String>| {
            let mut edit_state = &mut state.edit_state;
            let employee_clone: Employee = (*employee).clone();
            edit_state.employee = Some(employee_clone);
            state.page = Page::Edit;
        }))
    };

    let content = html! {
        h1 {
            "Our Team"
        }
        table.table {
            thead {
                tr {
                    th.pointer scope="col" onclick=(
                        event!(paro_app, (move |state: &mut ApplicationState, _|
                            state.list_state.sort_by(EmployeeField::FirstName, &state.employees)))
                    ) {
                        "First"
                    }
                    th.pointer scope="col" onclick=(
                        event!(paro_app, (move |state: &mut ApplicationState, _|
                            state.list_state.sort_by(EmployeeField::LastName, &state.employees)))
                    ) {
                        "Last"
                    }
                    th.pointer scope="col" onclick=(
                        event!(paro_app, (move |state: &mut ApplicationState, _|
                            state.list_state.sort_by(EmployeeField::Login, &state.employees)))
                    ) {
                        "Login"
                    }
                    th.pointer scope="col" onclick=(
                        event!(paro_app, (move |state: &mut ApplicationState, _|
                            state.list_state.sort_by(EmployeeField::Department, &state.employees)))
                    ) {
                        "Department"
                    }
                    th scope="col" {
                        ""
                    }
                }
            }
            tbody {
                @for employee in &list_state.filtered_employees {
                    @let on_edit = create_edit_event(employee.clone());

                    tr {
                        td {
                            (employee.first_name)
                        }
                        td {
                            (employee.last_name)
                        }
                        td {
                            (employee.login)
                        }
                        td {
                            (employee.department)
                        }
                        td {
                            button.btn."btn-primary" onclick=({on_edit}) {
                                "Edit"
                            }
                        }
                    }
                }
            }
        }
    
    };
    let markup = render_layout(&mut paro_app.clone(), content);
    markup
}
