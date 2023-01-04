# pâro-rs

An opinionated way to develop [tauri](https://tauri.app/) desktop applications powered by html and csss where you do not write any client code (no javascript, no webassembly) as all html rendering and event handling is done inside your tauri application. pâro has the minimal client code that is still required already done for you. No more https calls or dealing with JSON.

This cuts down on code complexity, build process complexity, compile time, nerve-wracking and frustration.

pâro does not enforce how you generate your html. Use your favorite template engine or just use format!() on strings. pâro does not care, but [maud compile time templates](https://github.com/lambda-fairy/maud) will be used in many examples to get compile time checks.

# the pâro library is currently in pre-alpha state and will have small api changes soon, with a more stable version in the coming days.

## Where the name comes from

> The name pâro comes from the [Dictionary of obscure sorrows](https://www.dictionaryofobscuresorrows.com/post/173924002125/p%C3%A2ro-n-the-feeling-that-no-matter-what-you-do-is) and describes the feeling that no matter what you do is always somehow wrong—that any attempt to make your way comfortably through the world will only end up crossing some invisible taboo—as if there’s some obvious way forward that everybody else can see but you, each of them leaning back in their chair and calling out helpfully, colder, colder, colder.

Pâro was what I felt writing my first tauri app and having to write an entire second application for the GUI, separated by http calls and json (de-)serialization, as well as during pâro's conceptual phase.

## Roadmap:

- [x] Get pâro working
- [ ] Improve API (CallbackStore vs ApplicationState as toplevel element)
- [ ] Examples
  - [x] Minimal counter example
  - [ ] Implement GUI for [html2maud](https://github.com/grayfallstown/html2maud) with pâro and put it as submodule under examples
  - [ ] More complex example with
    - [ ] Routing
    - [ ] Conditional rendering
    - [ ] Server side non gui events like api events or async database queries
- [ ] pâro starter
    - [ ] As github starter repository
    - [ ] Include basics as routing, a formular and composing
    - [ ] pure format!() pâro starter
    - [ ] maud pâro starter
- [ ] Dokumentation
- [ ] Establish Best Practices
- [ ] Differential html update
- [ ] Get a Logo


## License

MIT or Apache 2


## Maintainablility

- I plan to use pâro on multiple projects myself
- If the [bus factor](https://en.wikipedia.org/wiki/Bus_factor) of 1 worries you, with around 200 lines of easy code I got to say: If you can use pâro, you are already overqualified to maintain it yourself.

## Technical Details

#### pâro itself consists of three main components:

- [CallbackStore](https://github.com/grayfallstown/paro-rs/blob/main/src/lib.rs#L9)
  The CallbackStore is a struct that contains a `HashMap<CallbackID, Callback>`.
  All server side callbacks are stored here.
- [event!](https://github.com/grayfallstown/paro-rs/blob/main/src/lib.rs)
  A macro that creates a callback with an id and adds it to the `CallbackStore`. It returns a small js call to the pâro client script as String. Example: `window.__PARO__.emitEvent("f0cbfc89-677b-481a-8746-05e2335d5cf8")`
- [paro.js](https://github.com/grayfallstown/paro-rs/blob/main/src/paro.js)
  A quite small js script that connects to your tauri app via websocket and shows html that was send by your tauri app. Wasm would have been overkill here.

#### Additionally pâro requires:

- You need to add the crate `uuid` with feature `v4` enabled to your dependencies
- A websocket to connect to that handles calls to the `CallbackStore` and sends html to show to the client.

## Known issues

- Usage of `event!` causes `unnecessary parentheses around function argument` warning, but parentheses cannot be removed.


## Trivia

While pâro mainly exists to be used with [tauri](https://tauri.app), outside of the readme and code comments
it does not reference tauri in any way. If you wanted, you could use pâro with tauri alternatives or even on an
actual webapp. Please be aware that handling state and event handling on the server for thousands of users
would require quite a few resources on the server.
