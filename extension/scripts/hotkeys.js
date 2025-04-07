document.addEventListener('keydown', function (e) {
    (async () => {
        const response = await chrome.runtime.sendMessage({key: e.key});
    })();
});