
use std::sync::{Arc, RwLock};

use maud::html;

use paro_rs::ParoApp;

use crate::state::*;
use crate::pages::render_layout;


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
    let list_state = &paro_app.read().unwrap().state.list_state;
    let content = html! {
        h1 {
            "List"
        }
        table.table {
            thead {
                tr {
                    th scope="col" {
                        "First"
                    }
                    th scope="col" {
                        "Last"
                    }
                    th scope="col" {
                        "Login"
                    }
                    th scope="col" {
                        "Department"
                    }
                }
            }
            tbody {
                @for employee in &list_state.filtered_employees {
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
                    }
                }
            }
        }
    
    };
    let markup = render_layout(&mut paro_app.clone(), content);
    markup
}
