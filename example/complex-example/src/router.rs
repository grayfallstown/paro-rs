use std::sync::{Arc, RwLock};

use paro_rs::ParoApp;

use crate::state::ApplicationState;
use crate::pages::{render_add, render_edit, render_home, render_list};

#[derive(Debug, Clone, PartialEq)]
pub enum Page {
    Home,
    List,
    Edit,
    Add,
}


pub fn render_page(paro_app: &mut Arc<RwLock<ParoApp<ApplicationState>>>) -> String {
    let page = {paro_app.read().unwrap().state.page.clone()};
    let html = match page {
        Page::Add => render_add(paro_app),
        Page::Edit => render_edit(paro_app),
        Page::Home => render_home(paro_app),
        Page::List => render_list(paro_app),
    };
    html
}
