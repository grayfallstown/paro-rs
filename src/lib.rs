use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/**
 * The heart of pâro. A server side (as in, inside your tauri application)
 * callback store that holds all your server side callbacks / eventhandlers.
 */
 #[derive(Clone)]
pub struct CallbackStore {
    callbacks: HashMap<String, Arc<Mutex<dyn FnMut() + Send + 'static>>>
}

impl CallbackStore {
    pub fn new() -> CallbackStore {
        CallbackStore {
            callbacks: HashMap::new(),
        }
    }

    pub fn insert(&mut self, id: String, callback: Arc<Mutex<dyn FnMut() + Send + 'static>>) -> () {
        if self.callbacks.contains_key(&id) {
            panic!("[paro] callback ids must be unique, '{}' is not", &id);
        }
        self.callbacks.insert(id, callback);
    }

    pub fn clear(&mut self) -> () {
        self.callbacks.clear();
    }

    pub fn call(&mut self, id: String) -> Result<(), String> {
        match self.callbacks.get(&id) {
            Some(callback) => {
                let mut locked = { callback.lock().unwrap() };
                Ok(locked())
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
 * let counterBtnMarkup = format!(r#"<button onclick="{}">
 *         counter: {}
 *     </button>"#, event!(state, (move |mut state| {
 *               let mut callback_state = state.lock().unwrap();
 *               callback_state.numbers[0] = callback_state.numbers[0] + 1;
 *               println!("first number of state.numbers updated to: {}", callback_state.numbers[0]);
 *           })), state.lock().unwrap().numbers[0]);
 * 
 * Example usage with maud templates:
 * 
 *   let counterBtnMarkup = html! {
 *       button onclick=({
 *           event!(state, (move |mut state| {
 *               let mut callback_state = state.lock().unwrap();
 *               callback_state.numbers[0] = callback_state.numbers[0] + 1;
 *               println!("first number of state.numbers updated to: {}", callback_state.numbers[0]);
 *           }))
 *       }) { "counter:" (state.lock().unwrap().numbers[0]) }
 *   };
 */
#[macro_export]
macro_rules! event {
    ($state:expr, $closure:tt)=>{
        {
            let mut state_clone = $state.clone();
            let callback_id = uuid::Uuid::new_v4().to_string();
            let callback = move || {
                let mut closure_box: Box<dyn FnMut(&mut Arc<Mutex<ApplicationState>>) + Send + 'static> = Box::new($closure);
                closure_box(&mut state_clone);
            };
            {
                $state.lock().unwrap().callback_store.insert(
                    callback_id.clone(),
                    Arc::new(Mutex::new(callback))
                );
            }
            let javascript_call = format!{"window.__PARO__.emitEvent(\"{}\")", &callback_id};
            javascript_call
        }
    }
}
