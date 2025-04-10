document.addEventListener('keydown', function (e) {
    if (!e.isTrusted) {
        return;
    }
    (async () => {
        const response = await chrome.runtime.sendMessage({key: e.key, ctrlKey: e.ctrlKey, shiftKey: e.shiftKey, altKey: e.altKey, metaKey: e.metaKey});
        console.log(e)
    })();
});