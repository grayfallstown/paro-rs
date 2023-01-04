/**
 * Entire pâro client code.
 * All you need to do is to overwrite window.__PARO__.websocketUrl
 * to use the same port as your tauri app, have an element with id
 * 'paro-content' (can be configured and can be on body)
 * and then call * `window.__PARO__.initialize();`
 * 
 * After importing paro.js:
 *     <script>
 *         window.__PARO__.websocketUrl = ws://127.0.0.1:1234
 *         window.__PARO__.initialize();
 *     </script>
 * </head>
 * <body id="paro-application">
 * </body>
 */
(function() {
    PARO = {
        websocketUrl: "ws://127.0.0.1:8080",
        initialize: null,
        onMessageHandler: undefined, // (event) => {}
        onOpenHandler: undefined, // (event) => {}
        onCloseHandler: undefined, // (event) => {}
        onErrorHandler: undefined, // (event) => {}
        onEmitEventHandler: undefined, // (event_id) => {}
        emitEvent: undefined, // (event_id) => {}
        baseElementId: "paro-application",
        pingInterval: 60000,
        logging: true,
        logger: {
            info: console.info.bind(console),
            error: console.info.bind(console)
        },
        websocket: null
    };
    window.__PARO__ = PARO;

    // give the paro user a chance to change pâro settings

    PARO.initialize = () => {
        PARO = window.__PARO__;
        if (PARO.logging) PARO.logger.info("[paro init] connecting to websocket via " + PARO.websocketUrl);
        let socket = new WebSocket(PARO.websocketUrl);

        socket.onopen = function(event) {
            if (PARO.logging) PARO.logger.info("[paro open] Connection established", event);
            PARO.websocket = socket;
            if (PARO.onOpenHandler) PARO.onOpenHandler(event);
            socket.send("ping");
        };

        // keep connection alive
        setInterval(() => socket.send("ping"), PARO.pingInterval);

        socket.onmessage = function(event) {
            if (PARO.logging)
                PARO.logger.info("[paro websocket message] Data received from server:", event);
            if (PARO.onMessageHandler)
                PARO.onMessageHandler(event);
            if (event.data == "pong")
                return;
            var paroElement = document.getElementById(PARO.baseElementId);
            if (paroElement)
                paroElement.innerHTML = event.data;
            else if (PARO.logging)
                PARO.logger.error("[paro websocket message] could not find paro element '#" + PARO.baseElementId +
                "'. Html will not be rendered!", event);
        };
        

        socket.onclose = function(event) {
        if (event.wasClean) {
            if (PARO.logging)
            PARO.logger.info("[paro websocket close] Connection closed cleanly code=" +
                event.code + " reason=" + event.reason, event);
        } else {
            // e.g. server process killed or network down
            // event.code is usually 1006 in this case
            if (PARO.logging)
            PARO.logger.info("[paro websocket close] Connection died", event);
        }
        if (PARO.onCloseHandler)
            PARO.onCloseHandler(event);
        };

        socket.onerror = function(error) {
        if (PARO.logging)
            PARO.logger.error("[paro websocket error]", event);
        if (PARO.onErrorHandler)
            PARO.onErrorHandler(event);
        };
    }

    PARO.emitEvent = (event_id) => {
        PARO = window.__PARO__;
        if (PARO.websocket) {
            if (PARO.logging)
                PARO.logger.info("[paro emit event] emitting event '" + event_id + "'");
            if (PARO.onEmitEventHandler)
                PARO.onEmitEventHandler(event);
            PARO.websocket.send(event_id);
        } else if (PARO.logging) {
            PARO.logger.error("[paro emit event] trying to emit an event while websocket is not yet connected", event_id);
        }
    };

})();
