use uuid::Uuid;
use std::sync::Arc;
use maud::{Render, Escaper};
use std::fmt::Write;
use std::slice::Iter;
use crate::router::Page;
use crate::pages::*;


/**
 * State of your pâro application.
 * Can contain anything you need to hold as state and is
 * available in all callbacks.
 * No need to be serializable, clonable or anything. You can
 * have network connections and open file handles in here.
 */
pub struct ApplicationState {
    pub page: Page,
    pub employees: Vec<Arc<Employee>>,
    pub employee_of_the_month: Option<String>,
    pub employee_to_edit: Option<Employee>,

    pub add_state: AddState,
    pub list_state: ListState,
}

impl ApplicationState {
    pub fn default() -> ApplicationState {
        let employees = vec![
            Arc::new(Employee {
                id: Uuid::new_v4().to_string(),
                first_name: "John".to_owned(),
                last_name: "Doe".to_owned(),
                login: "john.doe".to_owned(),
                department: Department::Sales,
            }),
            Arc::new(Employee {
                id: Uuid::new_v4().to_string(),
                first_name: "Jane".to_owned(),
                last_name: "Doe".to_owned(),
                login: "Jane.doe".to_owned(),
                department: Department::Management,
            }),
            Arc::new(Employee {
                id: Uuid::new_v4().to_string(),
                first_name: "John".to_owned(),
                last_name: "Smith".to_owned(),
                login: "john.smith".to_owned(),
                department: Department::Maintenance,
            }),
            Arc::new(Employee {
                id: Uuid::new_v4().to_string(),
                first_name: "Alice".to_owned(),
                last_name: "Smith".to_owned(),
                login: "alice.smith".to_owned(),
                department: Department::Production,
            }),
            Arc::new(Employee {
                id: Uuid::new_v4().to_string(),
                first_name: "Box".to_owned(),
                last_name: "Smith".to_owned(),
                login: "bob.smith".to_owned(),
                department: Department::Production,
            }),
            Arc::new(Employee {
                id: Uuid::new_v4().to_string(),
                first_name: "Max".to_owned(),
                last_name: "Mustermann".to_owned(),
                login: "max.mustermann".to_owned(),
                department: Department::Production,
            }),
        ];

        let result = ApplicationState {
            page: Page::Home,
            employee_of_the_month: None,
            list_state: ListState::default(&employees),
            add_state: AddState::default(),
            employees: employees,
            employee_to_edit: None,
        };
        return result;
    }


}

// you could use &str and proper lifetimes here, I just want to keep it simple.
pub struct Employee {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub login: String,
    pub department: Department,
}

#[derive(Debug, PartialEq)]
pub enum EmployeeField {
    FirstName,
    LastName,
    Login,
    Department,
}

#[derive(Debug, PartialEq)]
pub enum Direction {
    Asc,
    Dsc,
}

#[derive(Debug, PartialEq)]
pub enum Department {
    Sales,
    Production,
    Management,
    Maintenance,
}

impl Department {
    pub fn into_iter() -> Iter<'static, Department> {
        static DEPARTMENTS: [Department; 4] = [
            Department::Sales,
            Department::Production,
            Department::Management,
            Department::Maintenance,
        ];
        DEPARTMENTS.iter()
    }
}

impl Render for Department {
    fn render_to(&self, output: &mut String) {
        let mut escaper = Escaper::new(output);
        let as_str = match self {
            Department::Sales => "Sales",
            Department::Production => "Production",
            Department::Management => "Management",
            Department::Maintenance => "Maintenance",
        };
        write!(escaper, "{}", as_str).unwrap();
    }
}
