# [pâro-rs](https://github.com/grayfallstown/paro-rs) for [tauri](https://tauri.app/)

![pâro logo](https://github.com/grayfallstown/paro-rs/blob/main/paro.png?raw=true)

An opinionated way to develop [tauri](https://tauri.app/) desktop applications powered by html and css where you do not write any client code (no javascript, no webassembly) as all html rendering and event handling is done inside your tauri application. pâro has the minimal client code that is still required already done for you. No more https calls or dealing with JSON.

This cuts down on code complexity, build process complexity, compile time, nerve-wracking and frustration.

pâro does not enforce how you generate your html. Use your favorite template engine or just use format!() on strings. pâro does not care, but [maud compile time templates](https://github.com/lambda-fairy/maud) will be used in many examples to get compile time checks.

## Example

A simple counting button example ([full example here](https://github.com/grayfallstown/paro-rs/tree/main/example/minimal-counter)):

```rust
/**
 * Pure html rendering without template engine. Has no compile time checks on
 * the generated html.
 */
fn render_with_format(paro_app: &mut Arc<RwLock<ParoApp<ApplicationState>>>) -> String {
     let html = format!(
        r#"<button onclick='{}'>
            counter: {}
        </button>"#, // we use single quotes on onclick, as event! returns a string with double quotes. maud handles that iself
            event!(paro_app, (move |state: &mut ApplicationState, _value| {
                // this is executed here in tauri and not in the gui client application
                state.current_count += 1;
                println!("first number of state.numbers updated to: {}", state.current_count);
            })),
            paro_app.read().unwrap().state.current_count
        );
    println!("format! generated html:\n{}", html);
    return html;
}

/**
 * Html rendering with a template engine. We are using maud here, as it has compile time checks
 * on the generated html, but you can use whatever you prefer.
 */
fn render_with_maud(paro_app: &mut Arc<RwLock<ParoApp<ApplicationState>>>) -> String {
    let maud_template = html! {
        button onclick=({
            event!(paro_app, (move |state: &mut ApplicationState, _value| {
                // this is executed here in tauri and not in the gui client application
                state.current_count += 1;
                println!("first number of state.numbers updated to: {}", state.current_count);
            }))
        }) { "counter:" (paro_app.read().unwrap().state.current_count) }
    };
    let html = maud_template.into_string();
    println!("maud generated html:\n{}", html);
    return html;
}
```

## All examples:

All examples are made as their own crates, rather than using cargo examples, as the imports
might overwise be confusing for rust newcommers.

- [minimal counter example](https://github.com/grayfallstown/paro-rs/tree/main/example/minimal-counter)
  The absolute minimum to get you started. Just a button that counts on click. Shows manual html generation
  as well as html generation with [maud templates](https://github.com/lambda-fairy/maud).
- [Employee CRUD example](https://github.com/grayfallstown/paro-rs/tree/main/example/complex-example)
  Shows routing, list sorting and filtering as well as form handling.
- [html2maud](https://github.com/grayfallstown/html2maud)
  A simple GUI that converts html to [maud templates](https://github.com/lambda-fairy/maud)


## Where the name comes from

> The name pâro comes from the [Dictionary of obscure sorrows](https://www.dictionaryofobscuresorrows.com/post/173924002125/p%C3%A2ro-n-the-feeling-that-no-matter-what-you-do-is) and describes the feeling that no matter what you do is always somehow wrong—that any attempt to make your way comfortably through the world will only end up crossing some invisible taboo—as if there’s some obvious way forward that everybody else can see but you, each of them leaning back in their chair and calling out helpfully, colder, colder, colder.

Pâro was what I felt writing my first tauri app and having to write an entire second application for the GUI, separated by http calls and json (de-)serialization, as well as during pâro's conceptual phase.

## Roadmap:

- [x] Get pâro working
- [x] Improve API (CallbackStore vs ApplicationState as toplevel element)
- [x] Internally re-pub uuid, and refer to it as paro::UUID so it does not have to be added to the applications dependencies
- [x] Use backticks in event! return so both single and double quotes work
- [ ] Get port from tauri and use it directly
- [ ] Use a logging framework
- [ ] Examples
  - [x] Minimal counter example
  - [ ] Implement GUI for [html2maud](https://github.com/grayfallstown/html2maud) with pâro and put it as submodule under examples
  - [ ] More complex example with
    - [x] Routing
    - [x] Conditional rendering
    - [ ] Form validation
    - [ ] Server side non gui events like api events or async database queries
- [ ] pâro starter
    - [ ] As github starter repository
    - [ ] Include basics as routing, a formular and composing
    - [ ] pure format!() pâro starter
    - [ ] maud pâro starter
- [ ] Documentation / gitbook
- [ ] Establish Best Practices
- [ ] Differential html update
- [x] Get a Logo


## License

MIT or Apache 2


## Technical Details

#### pâro itself consists of three main components:

- [ParoApp\<MyState\>](https://github.com/grayfallstown/paro-rs/blob/main/src/lib.rs#L9)
  ParoApp holds your application state `MyState` and a `HashMap<CallbackID, Callback>`.
  All server side callbacks are stored there.
- [event!](https://github.com/grayfallstown/paro-rs/blob/main/src/lib.rs#L95)
  A macro that creates a server side callback with an id and adds it to the `ParoApp`. It returns a small js call to the pâro client script as String. Example: `window.__PARO__.emitEvent("f0cbfc89-677b-481a-8746-05e2335d5cf8")` wich you can add to your html `onclick='event!([...])'`.
- [paro.js](https://github.com/grayfallstown/paro-rs/blob/main/src/paro.js)
  A quite small js script that connects to your tauri app via websocket and shows html that was send by your tauri app and it sends all client side events to the server / tauri to be handled there. Wasm would have been overkill here.

Those three components allow you to write the html gui without writing any client code, as in no javascript or webassembly.

#### Additionally pâro requires:

- You need to add the crate `uuid` with feature `v4` enabled to your dependencies
- A websocket to connect to that handles calls to the `CallbackStore` and sends html to show to the client.


## Trivia

While pâro mainly exists to be used with [tauri](https://tauri.app), outside of the readme and code comments
it does not reference tauri in any way. If you wanted, you could use pâro with tauri alternatives or even on an
actual webapp. Please be aware that handling state and event handling on the server for thousands of users
in a webapp would require quite a few resources on the server.
