use std::collections::HashMap;
use std::sync::{Arc, Mutex};


/**
 * The heart of pâro. A server side (as in, inside your tauri application)
 * callback store that holds all your server side callbacks / eventhandlers.
 */
 pub struct ParoApp<State> {
    callbacks: HashMap<String, Arc<Mutex<dyn FnMut(&mut State) + Send + 'static>>>,
    pub state: State,
}

impl <State> ParoApp<State> {
    pub fn new(state: State) -> ParoApp<State> {
        ParoApp::<State> {
            callbacks: HashMap::new(),
            state: state,
        }
    }

    /**
     * Register a callback with pâro so it can be called by it
     */
    pub fn insert(&mut self, id: String, callback: Arc<Mutex<dyn FnMut(&mut State) + Send + 'static>>) -> () {
        if self.callbacks.contains_key(&id) {
            panic!("[paro] callback ids must be unique, '{}' is not", &id);
        }
        self.callbacks.insert(id, callback);
    }

    /**
     * Clear callback registry. It is adviced to clear the callback registry before
     * re-rendering and therfore refilling the registry to free up memory.
     */
    pub fn clear(&mut self) -> () {
        self.callbacks.clear();
    }

    /**
     * Call a callback by its id
     */
    pub fn call(&mut self, id: String) -> Result<(), String> {
        match self.callbacks.get(&id) {
            Some(callback) => {
                println!("before lock");
                let mut locked = { callback.lock().unwrap() };
                println!("after lock");
                let result = Ok(locked(&mut self.state));
                
                println!("after call");
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
        r#"<button onclick='{}'>
            counter: {}
        </button>"#, // we use single quotes on onclick, as event! returns a string with double quotes. maud handles that iself
            event!(paro_app, (move |state: &mut ApplicationState| { // ApplicationState beeing whatever struct you use, here ParoApp<ApplicationState>
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
            event!(paro_app, (move |state: &mut ApplicationState| {
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
            let callback_id = uuid::Uuid::new_v4().to_string();
            {
                $paroApp.lock().unwrap().insert(
                    callback_id.clone(),
                    Arc::new(Mutex::new(($closure)))
                );
            }
            let javascript_call = format!{"window.__PARO__.emitEvent(\"{}\")", &callback_id};
            javascript_call
        }
    }
}
