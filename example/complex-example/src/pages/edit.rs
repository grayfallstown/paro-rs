
use std::sync::{Arc, RwLock};

use maud::html;

use paro_rs::ParoApp;

use crate::state::ApplicationState;
use crate::pages::render_layout;


pub fn render_edit(paro_app: &mut Arc<RwLock<ParoApp<ApplicationState>>>) -> String {
    let content = html! {
        h1 {
            "Edit"
        }
        
    };
    let markup = render_layout(&mut paro_app.clone(), content);
    markup
}