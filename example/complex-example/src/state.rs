use uuid::Uuid;
use crate::router::Page;
use maud::{Render, Escaper};
use std::fmt::Write;


/**
 * State of your pâro application.
 * Can contain anything you need to hold as state and is
 * available in all callbacks.
 * No need to be serializable, clonable or anything. You can
 * have network connections and open file handles in here.
 */
pub struct ApplicationState {
    pub page: Page,
    pub employees: Vec<Employee>,
    pub employee_of_the_month: Option<String>,
    pub employee_to_edit: Option<Employee>,
    pub employee_to_add: Employee,
}

impl ApplicationState {
    pub fn default() -> ApplicationState {
        ApplicationState {
            page: Page::Home,
            employee_of_the_month: None,
            employees: vec![
                Employee {
                    id: Uuid::new_v4().to_string(),
                    first_name: "John".to_owned(),
                    last_name: "Doe".to_owned(),
                    login: "john.doe".to_owned(),
                    department: Department::Sales,
                },
                Employee {
                    id: Uuid::new_v4().to_string(),
                    first_name: "Jane".to_owned(),
                    last_name: "Doe".to_owned(),
                    login: "Jane.doe".to_owned(),
                    department: Department::Management,
                },
                Employee {
                    id: Uuid::new_v4().to_string(),
                    first_name: "John".to_owned(),
                    last_name: "Smith".to_owned(),
                    login: "john.smith".to_owned(),
                    department: Department::Maintenance,
                },
                Employee {
                    id: Uuid::new_v4().to_string(),
                    first_name: "Alice".to_owned(),
                    last_name: "Smith".to_owned(),
                    login: "alice.smith".to_owned(),
                    department: Department::Production,
                },
                Employee {
                    id: Uuid::new_v4().to_string(),
                    first_name: "Box".to_owned(),
                    last_name: "Smith".to_owned(),
                    login: "bob.smith".to_owned(),
                    department: Department::Production,
                },
                Employee {
                    id: Uuid::new_v4().to_string(),
                    first_name: "Max".to_owned(),
                    last_name: "Mustermann".to_owned(),
                    login: "max.mustermann".to_owned(),
                    department: Department::Production,
                },
            ],
            employee_to_edit: None,
            employee_to_add: Employee {
                id: Uuid::new_v4().to_string(),
                first_name: "".to_owned(),
                last_name: "".to_owned(),
                login: "".to_owned(),
                department: Department::Production,
            },
        }
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

pub enum Department {
    Sales,
    Production,
    Management,
    Maintenance,
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
