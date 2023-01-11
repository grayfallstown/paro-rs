# [pâro-rs](https://github.com/grayfallstown/paro-rs) for [tauri](https://tauri.app/)

![pâro logo](https://github.com/grayfallstown/paro-rs/blob/main/paro.png?raw=true)

An opinionated way to develop [tauri](https://tauri.app/) desktop applications powered by html and css where you do not write any client code (no javascript, no webassembly) as all html rendering and event handling is done inside your tauri application. pâro has the minimal client code that is still required already done for you. No more https calls or dealing with JSON serialization.

This cuts down on code complexity, build process complexity, compile time, nerve-wracking and frustration.

pâro does not enforce how you generate your html. Use your favorite template engine or just use format!() on strings. pâro does not care, but [maud compile time templates](https://github.com/lambda-fairy/maud) will be used in many examples to get compile time checks.

## To Clearify. This is all pâro does:

- pâro allows you to render your html inside tauri with whatever template engine you like
- pâro pulls client events like onclick and oninput into your tauri app, so you can handle them there, with full access to all objects and functions in your application
- pâro will show all html you send from your tauri app in the webview

Thats it. pâro is not a framework or something large and fancy.


## What you don't have to learn

- **How to render HTML**: pâro does not bring a new rendering mechanism or template engine. Just use whatever
  template engine you already know, as long as it can render into a String, you are set.
- **URL based Routing**: WebView applications don't come with a full browser and normally do not have bookmarking.
  Because of that there is no point in having URL based Routing and pâro does not bring a router.
- **How pâro does MVC, MVVC, or another pattern**: pâro does not follow any MVC pattern or alike. pâro knows a
  struct `State` which you provide and that contains all data you require to to render HTML, create callbacks
  and if necessary, to pull more data in from external sources. How you fill the state is up to you.



## Philosophy

- **Why write two applications that communicate like web applications when we write a native gui**, just utilizing html and css because of how powerfull and flexible they are?
- **No gap between application and GUI.** No datastructures need to be send via JSON to be rendered, no http handling or the logic being split, or worse, copied across two applications.
- **All control, rendering and event handling inside the developers tauri application.** No application specific client code has to be written, other than a single variable holding a port and a call to Paro.initialize(). (the port specification might disappear in the future as well)
- **Give all power to the developer.** Show him how things can be realized, but don't enforce certain project structures, development cycles or ways on how to solve certain problems in his application. We don't know every developers applications and their requirements.
- **Provide the minial API for the developer to be able to do everything he will ever need and not one thing more.**
- **Make no assumptions over the developers datastructures**, as in do not require things to be serializable to JSON, or to implement any traits.
- We are showing a GUI inside a tauri WebView, which is a lighter version of a normal browser. We use the WebView
  because we want to save on resources and be performant, but there is **no need to squeeze the last kilobyte or the last nanosecond out of the rendering cylce, as we create a GUI and not a multiplayer video game.**


## What you do need to learn

- **How to do event callbacks**: pâro uses a single macro to simplify creation of eventhandler (for onclick, oninput, etc.). You can use the direct API if you do not like macros. The events are executed inside
  your tauri application. The example should explain how to use it.

## Example

A simple counting button example ([full example here](https://github.com/grayfallstown/paro-rs/tree/main/example/minimal-counter)):

```rust
/**
 * Pure html rendering without template engine. Has no compile time checks on
 * the generated html.
 * _value is an Option<String> which contains the input fields value if you, for example, 
 * use onchange or oninput on an input field or textarea if there is any.
 */
fn render_with_format(paro_app: &mut Arc<RwLock<ParoApp<ApplicationState>>>) -> String {
    let increase_counter = event!(paro_app, (move |state: &mut ApplicationState, _value| {
        // this is executed here in tauri and not in the gui client application
        state.current_count += 1;
        println!("first number of state.numbers updated to: {}", state.current_count);
    }));
    let html = format!(
        r#"<button onclick="{}">
            counter: {}
        </button>"#,
            increase_counter,
            paro_app.read().unwrap().state.current_count
        );
    println!("format! generated html:\n{}", html);
    return html;
}

/**
 * Html rendering with a template engine. We are using maud here, as it is easy, has compile time checks
 * on the generated html and is very performant, but you can use whatever you prefer.
 */
fn render_with_maud(paro_app: &mut Arc<RwLock<ParoApp<ApplicationState>>>) -> String {
    let increase_counter = event!(paro_app, (move |state: &mut ApplicationState, _value| {
        // this is executed here in tauri and not in the gui client application
        state.current_count += 1;
        println!("first number of state.numbers updated to: {}", state.current_count);
    }));
    let maud_template = html! {
        button onclick=(increase_counter) {
          "counter: " (paro_app.read().unwrap().state.current_count)
        }
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
  Shows how routing, list sorting and filtering as well as form handling *can* be done.
- [html2maud](https://github.com/grayfallstown/html2maud)
  A simple GUI that converts html to [maud templates](https://github.com/lambda-fairy/maud)


## Where the name comes from

> The name pâro comes from the [Dictionary of obscure sorrows](https://www.dictionaryofobscuresorrows.com/post/173924002125/p%C3%A2ro-n-the-feeling-that-no-matter-what-you-do-is) and describes the feeling that no matter what you do is always somehow wrong—that any attempt to make your way comfortably through the world will only end up crossing some invisible taboo—as if there’s some obvious way forward that everybody else can see but you, each of them leaning back in their chair and calling out helpfully, colder, colder, colder.

Pâro was what I felt writing my first tauri app and having to write an entire second application for the GUI, separated by http calls and json (de-)serialization and possibly another programming language, as well as during pâro's conceptual phase.

# pâro will enter alpha status shortly

## Roadmap:

- [x] Get pâro working
- [ ] Get port from tauri and use it directly or use a non-port connection
- [ ] Use a logging framework
- [ ] Examples
  - [x] Minimal counter example
  - [x] Implement GUI for [html2maud](https://github.com/grayfallstown/html2maud) with pâro and put it as submodule under examples
  - [ ] More complex example with
    - [x] Routing
    - [x] Conditional rendering
    - [x] Form handling like validation
    - [ ] Server side non gui events like api events or async database queries
- [ ] pâro starter
    - [ ] As github starter repository
    - [ ] Include basics as routing, a formular and composing
    - [ ] pure format!() pâro starter
    - [ ] maud pâro starter
- [ ] Documentation / gitbook
- [ ] Establish Best Practices
- [ ] Differential html updates
- [x] Get a Logo


## License

MIT or Apache 2


## Technical Details

#### pâro itself consists of three main components:

- [ParoApp\<MyState\>](https://github.com/grayfallstown/paro-rs/blob/main/src/lib.rs#L9)
  ParoApp holds your application state `MyState` and a `HashMap<CallbackID, Callback>`.
  All server side callbacks are stored there.
- [event!](https://github.com/grayfallstown/paro-rs/blob/main/src/lib.rs#L95)
  A macro that creates a server side callback with an id and adds it to the `ParoApp`. It returns a small js call to the pâro client script as String. Example: `window.__PARO__.emitEvent("f0cbfc89-677b-481a-8746-05e2335d5cf8")` wich you can add to your html `onclick='event!([...])'`. Within the event callback you have `ParoApp` and therefore your application state available, as well as a an Option `value`. The value is the value of the input field / textarea once the user interacted with it, if the event was triggered on such an element.
- [paro.js](https://github.com/grayfallstown/paro-rs/blob/main/src/paro.js)
  A quite small js script that connects to your tauri app via websocket and shows html that was send by your tauri app and sends all client side events to your tauri app to be handled there. Wasm would have been overkill here.

Those three components allow you to write the html GUI without writing any client code, as in no javascript or webassembly.


#### Additionally pâro requires:

- A websocket to connect to that handles calls to the `ParoApp` and sends html to show to the client.


## Trivia

While pâro mainly exists to be used with [tauri](https://tauri.app), outside of the readme and code comments,
it does not reference tauri in any way. If you wanted, you could use pâro with tauri alternatives or even on an
actual webapp. Please be aware that handling state and event handling on the server for thousands of users
in a webapp would make resource consumption on the server add up.
