
use std::sync::{Arc, RwLock};

use maud::html;

use paro_rs::ParoApp;

use crate::state::ApplicationState;
use crate::pages::render_layout;


pub fn render_list(paro_app: &mut Arc<RwLock<ParoApp<ApplicationState>>>) -> String {
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
                @for employee in &paro_app.read().unwrap().state.employees {
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
