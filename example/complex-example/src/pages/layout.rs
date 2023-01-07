
use std::sync::{Arc, RwLock};

use maud::{html, Markup};
use paro_rs::ParoApp;

use crate::state::ApplicationState;
use crate::pages::render_navigation;


pub fn render_layout(paro_app: &mut Arc<RwLock<ParoApp<ApplicationState>>>, content: Markup) -> String {
    let rendered_navbar = html! {
        (render_navigation(&mut paro_app.clone()))

        (content)
     }.into_string();
     rendered_navbar
}
