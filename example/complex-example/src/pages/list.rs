
use std::sync::{Arc, RwLock};

use maud::html;

use paro_rs::*;

use crate::state::*;
use crate::pages::*;
use crate::router::*;


pub struct ListState {
    pub filtered_employees: Vec<Arc<Employee>>,
    pub employees_to_show: Vec<Arc<Employee>>,
    pub search_term: String,
    pub sort_direction: Direction,
    pub sort_by: EmployeeField,

    pub current_page: usize,
    pub page_count: usize,
}

impl ListState {
    pub fn default(employees: &Vec<Arc<Employee>>) -> ListState {
        let mut result = ListState {
            filtered_employees: vec![],
            employees_to_show: vec![],
            search_term: "".to_owned(),
            sort_direction: Direction::Asc,
            sort_by: EmployeeField::Login,

            current_page: 0,
            page_count: 0,

        };
        result.filter_employees(employees);
        result
    }

    pub fn next_page(&mut self, employees: &Vec<Arc<Employee>>) -> () {
        self.current_page += 1;
        self.filter_employees(employees);
    }

    pub fn prev_page(&mut self, employees: &Vec<Arc<Employee>>) -> () {
        self.current_page -= 1;
        self.filter_employees(employees);
    }

    pub fn sort_by(&mut self, field: EmployeeField, employees: &Vec<Arc<Employee>>) {
        self.current_page = 0;
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
                    .first_name.to_lowercase().contains(&self.search_term)
                    || employee.last_name.to_lowercase().contains(&self.search_term)
                    || employee.login.to_lowercase().contains(&self.search_term)
                    || format!("{:?}", employee.department).to_lowercase().contains(&self.search_term)
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

        self.page_count = ((filtered_employees.len() as f32) / (25 as f32)).ceil() as usize;
        let employees_to_show: Vec<Arc<Employee>> = filtered_employees.iter()
            .skip(self.current_page * 25)
            .take(25)
            .map(|employee| employee.clone())
            .collect();
        self.filtered_employees = employees_to_show;
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
        div {
            "Page: " ((list_state.current_page + 1))
            "/" (list_state.page_count)
            

            @if list_state.current_page != 0 {
                button.btn."btn-primary" type="button" onclick=({
                    event!(paro_app, (move |state: &mut ApplicationState, _value: Option<String>| {
                        state.list_state.prev_page(&state.employees);
                    }))
                }) {
                    "prev page"
                }
            }

            @if list_state.page_count > 0 && list_state.current_page != (list_state.page_count - 1) {
                button.btn."btn-primary" type="button" onclick=({
                    event!(paro_app, (move |state: &mut ApplicationState, _value: Option<String>| {
                        state.list_state.next_page(&state.employees);
                    }))
                }) {
                    "next page"
                }
            }
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
