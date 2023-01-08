use std::sync::{ Arc, RwLock };

use maud::{ html, Markup };
use paro_rs::{ ParoApp, event };

use crate::state::ApplicationState;
use crate::router::Page;

pub fn render_navigation(paro_app: &mut Arc<RwLock<ParoApp<ApplicationState>>>) -> Markup {
    let state = &paro_app.read().unwrap().state;
    let rendered_navbar =
        html! {
        nav.navbar."navbar-expand"."navbar-dark"."bg-dark" {
     
           div #navbarSupportedContent.collapse."navbar-collapse" {
              ul."navbar-nav"."mr-auto" {
                 li."nav-item".active[state.page == Page::Home] {
                    a."nav-link" href="#" onclick=({
                        event!(paro_app, (move |state: &mut ApplicationState, _| state.page = Page::Home))
                    }){
                        "Home"
                        @if state.page == Page::Home {
                            span."sr-only" {
                                " (current)"
                            }  
                        }
                    }
                 }
     
                 li."nav-item".active[state.page == Page::List] {
                    a."nav-link" href="#" onclick=({
                        event!(paro_app, (move |state: &mut ApplicationState, _| state.page = Page::List))
                    }){
                        "Team"
                        @if state.page == Page::List {
                            span."sr-only" {
                                " (current)"
                            }  
                        }
                    }
                 }
     
                 li."nav-item".active[state.page == Page::List] {
                    a."nav-link" href="#" onclick=({
                        event!(paro_app, (move |state: &mut ApplicationState, _| state.page = Page::Add))
                    }){
                        "Add"
                        @if state.page == Page::Add {
                            span."sr-only" {
                                " (current)"
                            }
                        }
                    }
                 }
     
                 li."nav-item".active[state.page == Page::Edit] {
                    a."nav-link".disabled[state.employee_to_edit.is_none()] href="#" onclick=({
                        event!(paro_app, (move |state: &mut ApplicationState, _| state.page = Page::Edit))
                    }){
                        @if state.employee_to_edit.is_some() {
                            "Edit '" (state.employee_to_edit.as_ref().unwrap().login) "'"
                        } else {
                            "Edit"
                        }
                        @if state.page == Page::Edit {
                            span."sr-only" {
                                " (current)"
                            }
                        }
                    }
                 }
              }
     
                form."form-inline"."my-2"."my-lg-0" onsubmit=({
                    event!(paro_app, (move |state: &mut ApplicationState, _| state.page = Page::List))
                }) {
                    input."form-control"."mr-sm-2" type="search" placeholder="Search" aria-label="Search" oninput=({
                        event!(paro_app, (move |state: &mut ApplicationState, value: String| {
                            state.page = Page::List;
                            state.list_state.search_term = if value == "undefined" || value == "null" { "".to_owned() } else { value };
                            state.list_state.filter_employees(&state.employees);
                        }))
                    }) value=(paro_app.read().unwrap().state.list_state.search_term) {
                }
                button.btn."btn-outline-success"."my-2"."my-sm-0" type="submit" {
                    "Search"
                }
            }
           }
        }
     };
    rendered_navbar
}
