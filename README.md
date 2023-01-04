# pâro-rs

An opinionated way to develop [tauri](https://tauri.app/) desktop applications powered by html and csss where you do not write any client code (no javascript, no webassembly) as all html rendering and event handling is done inside your tauri application. pâro has the minimal client code that is still required already done for you. No more https calls or dealing with JSON.

This cuts down on code complexity, build process complexity, compile time, nerve-wracking and frustration.

pâro does not enforce how you generate your html. Use your favorite template engine or just use format!() on strings. pâro does not care, but [maud compile time templates](https://github.com/lambda-fairy/maud) will be used in many examples to get compile time checks.

# the pâro library is currently in pre-alpha state and will have small api changes soon.

## Where the name comes from

> The name pâro comes from the [Dictionary of obscure sorrows](https://www.dictionaryofobscuresorrows.com/post/173924002125/p%C3%A2ro-n-the-feeling-that-no-matter-what-you-do-is) and describes the feeling that no matter what you do is always somehow wrong—that any attempt to make your way comfortably through the world will only end up crossing some invisible taboo—as if there’s some obvious way forward that everybody else can see but you, each of them leaning back in their chair and calling out helpfully, colder, colder, colder.

That was what I felt writing my first tauri app and having to write an entire second application for the GUI, separated by http calls and json (de-)serialization, as well as during pâro's conceptual phase.



## Roadmap:

- [x] Get pâro working
- [ ] Improve API (CallbackStore vs ApplicationState as toplevel element)
- [ ] Examples
  - [ ] Minimal counter example
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
