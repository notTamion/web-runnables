let config;
let port = chrome.runtime.connectNative('de.tamion.web_runnables');
port.onMessage.addListener(function (msg) {
    switch (msg.type) {
        case "config": {
            config = msg.value;
            console.log(config)
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
            let match = sender.tab.url.match(runnable.regex);
            if (match === null) {
                continue;
            }
            if (runnable.hotkey.toUpperCase().replace(/\s+/g, "") !== keysPressed.toUpperCase()) {
                continue;
            }

            console.log("result = " + JSON.stringify(match) + ";" + runnable.arg_parser)
            chrome.userScripts.execute({
                    js : [{ code : "result = " + JSON.stringify(match) + ";" + runnable.arg_parser }],
                    target: { tabId: sender.tab.id }
                },
                (result) => {
                    console.log(result)
                    port.postMessage({
                        "type": "run",
                        "id": i,
                        "args": result[0].result
                    });
                });
        }
    }
);

function evalArgParser(runnable) {
    eval(runnable.arg_parser);
}