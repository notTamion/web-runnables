let port = chrome.runtime.connectNative('de.tamion.web_runnables');
port.onMessage.addListener(function (msg) {
    console.log('Received: ' + msg);
});
port.onDisconnect.addListener(function () {
    console.log('Disconnected');
});

chrome.tabs.onUpdated.addListener(
    function (tabId, changeInfo, tab) {
        if (changeInfo.url) {
        }
    }
);

chrome.runtime.onMessage.addListener(
    function(request, sender, sendResponse) {
        console.log(request.key);
        port.postMessage("run hello");
    }
);