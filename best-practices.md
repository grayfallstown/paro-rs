## Do not link directly to a website.

WebViews are not full browsers. We do not have a back button. If you link directly to an external site, the user has no way of coming back to your application. Instead you can use `<a href="#" onclick="event!([....])` and then use the crate [webbrowser](https://crates.io/crates/webbrowser) in your onclick event handler to open the link in the users browser of choice.

## Render HTML right away. Do heavy lifting in another thread.

Keep blocking or expensive actions outside your rendering thread. When querying databases, doing heavy calculations or pulling data from an external source like a web API or from a file you need to parse, do it asynchronous in a separate thread and reply html back to the user right away to display a spinner or something like a message to signal to the user that the action is running. If you do not do this, you keep the `ParoApp` read locked and cannot render any html in another thread, as rendering includes `event!` calls, which require a write lock. The same is true for other callbacks, as in button clicks. The app will appear unresponsive to the user. You can always render and send new html over the websocket without a client side event like a button click triggering it.
