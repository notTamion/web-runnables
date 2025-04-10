let config;
let port = chrome.runtime.connectNative('de.tamion.web_runnables');
port.onMessage.addListener(function (msg) {
    console.log(msg)
    switch (msg.type) {
        case "config": {
            config = msg.value;
            break;
        }
        case "log": {
            console.log("native: " + msg.value);
            break;
        }
    }
});
port.onDisconnect.addListener(function () {
    console.log('Disconnected');
});

chrome.runtime.onMessage.addListener(
    function(event, sender, sendResponse) {
        if (event.key.length !== 1) {
            return
        }

        const keys = [];

        if (event.ctrlKey) keys.push("Ctrl");
        if (event.shiftKey) keys.push("Shift");
        if (event.altKey) keys.push("Alt");
        if (event.metaKey) keys.push("Meta"); // For Mac Command key

        if (keys.length === 0 && config.require_special) {
            return;
        }

        keys.push(event.key);

        const keysPressed = keys.join("+");

        for (let i = 0; i < config.runnables.length; i++) {
            let runnable = config.runnables[i];
            if (runnable.hotkey.toUpperCase().replace(/\s+/g, "") === keysPressed.toUpperCase()) {
                console.log("Sent")
                port.postMessage({
                    "type": "run",
                    "id": i,
                });
            }
        }
    }
);