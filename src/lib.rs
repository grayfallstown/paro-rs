use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub use uuid::Uuid;


/**
 * The heart of pâro. A server side (as in, inside your tauri application)
 * callback store that holds all your server side callbacks / eventhandlers.
 */
 pub struct ParoApp<State> {
    callbacks: HashMap<String, (u128, Arc<Mutex<dyn FnMut(&mut State, String) + Send + 'static>>)>,
    iteration: u128,
    pub state: State,
}

impl <State> ParoApp<State> {
    pub fn new(state: State) -> ParoApp<State> {
        ParoApp::<State> {
            callbacks: HashMap::new(),
            iteration: 0,
            state: state,
        }
    }

    /**
     * Register a callback with pâro so it can be called by it
     */
    pub fn insert(&mut self, id: String, callback: Arc<Mutex<dyn FnMut(&mut State, String) + Send + 'static>>) -> () {
        if self.callbacks.contains_key(&id) {
            panic!("[paro] callback ids must be unique, '{}' is not", &id);
        }
        self.callbacks.insert(id, (self.iteration, callback));
    }

    /**
     * Clears old callbacks from the registry. It is adviced to call iterate before
     * each re-rendering.
     */
    pub fn iterate(&mut self) -> () {
        self.iteration += 1;
        let keys_to_drop: Vec<String> = {self.callbacks.iter()
            .filter(|(_key, (callback_iteration, _callback))| self.iteration - callback_iteration > 100)
            .map(|(key, _value)| key.to_owned())
            .collect()};
        for key in &keys_to_drop {
            self.callbacks.remove(key);
        }
        // println!("paro iterate dropped {} old callbacks and now contains {}", keys_to_drop.len(), self.callbacks.len());
    }

    /**
     * Call a callback by its id
     */
    pub fn call(&mut self, id: String) -> Result<(), String> {
        let split = id.split_once("__PARO__")
            .expect("expected __PARO__ as part of the message");
        let id = split.0;
        let value = split.1.to_owned();
        match self.callbacks.get(id) {
            Some((_, callback)) => {
                let mut locked = { callback.lock().unwrap() };
                let result = Ok(locked(&mut self.state, value));
                return result;
            },
            None => Err(format!("[paro] callback '{}' not found", &id))
        }
    }
}



/***
 * Creates a mew event / callback that you can reference in your html.
 * The event / callback is executed inside your tauri app / the server
 * and event! returns a minimal javascript call to tell pâro which
 * event / callback should be triggered on the server side. What you
 * write is pure rust, pâro handles javscript.
 * 
 * Example usage without maud templates
     let html = format!(
        r#"<button onclick="{}">
            counter: {}
        </button>"#,
            event!(paro_app, (move |state: &mut ApplicationState, _| { // ApplicationState beeing whatever struct you use, here ParoApp<ApplicationState>
                // this is executed here in tauri and not in the gui client application
                state.current_count += 1;
                println!("first number of state.numbers updated to: {}", state.current_count);
            })),
            paro_app.lock().unwrap().state.current_count
        );
 * 
 * Example usage with maud templates:
 * 
    let maud_template = html! {
        button onclick=({
            event!(paro_app, (move |state: &mut ApplicationState, _| {
                // this is executed here in tauri and not in the gui client application
                state.current_count += 1;
                println!("first number of state.numbers updated to: {}", state.current_count);
            }))
        }) { "counter:" (paro_app.lock().unwrap().state.current_count) }
    };
    let html = maud_template.into_string();
 */
#[macro_export]
macro_rules! event {
    ($paroApp:expr, $closure:tt)=>{
        {
            let callback_id = paro_rs::Uuid::new_v4().to_string();
            {
                $paroApp.lock().unwrap().insert(
                    callback_id.clone(),
                    Arc::new(Mutex::new(($closure)))
                );
            }
            let javascript_call = format!{"window.__PARO__.emitEvent(`{}`, event)", &callback_id};
            javascript_call
        }
    }
}
