"use strict";
importScripts("./wasm/socketworker/socketworker.js");
wasm_bindgen("/workers/wasm/socketworker/socketworker_bg.wasm")
    .then((_) => {
    var selfTyped = self;
    selfTyped.addEventListener("connect", (ev) => {
        ev.ports.forEach((port) => {
            port.addEventListener("message", (ev) => {
                port.postMessage(ev.data);
            });
            port.start();
        });
    });
    var socketWorker = undefined;
    selfTyped.addEventListener("online", (_ev) => {
        if (socketWorker) {
            console.warn("Online event but socket worker exists?!");
            socketWorker.free();
        }
        console.log("creating socket worker");
        socketWorker = wasm_bindgen.SocketWorker.new();
    });
    selfTyped.addEventListener("offline", (_ev) => {
        socketWorker?.free();
        socketWorker = undefined;
    });
    if (navigator.onLine) {
        selfTyped.dispatchEvent(new Event("online"));
    }
});
