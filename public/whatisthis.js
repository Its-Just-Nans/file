// https://github.com/Its-Just-Nans/whatisthis

const decodeFile = async (file) => {
    const data = await file.arrayBuffer();
    const filename = file.name;
    const source_url = URL.createObjectURL(file);
    let u8 = new Uint8Array(data);
    window.wasmBindings.process_file(u8, filename, source_url);

    document.getElementById("string-summary").onclick = async (ev) => {
        ev.currentTarget.parentElement.children[1].innerText = await file.text();
        ev.currentTarget.onclick = () => {};
    };

    document.getElementById("binary-summary").onclick = (ev) => {
        ev.currentTarget.parentElement.children[1].innerText = u8.toString();
        ev.currentTarget.onclick = () => {};
    };

    document.getElementById("hex-summary").onclick = (ev) => {
        ev.currentTarget.parentElement.children[1].innerText = u8.reduce(
            (acc, val) => acc + val.toString(16).padStart(2, "0") + " ",
            ""
        );
        ev.currentTarget.onclick = () => {};
    };
};

const startup = (event) => {
    console.log("application started - bindings:", window.wasmBindings, "WASM:", event.detail.wasm);

    const dropHandler = document.getElementById("dropHandler");
    const start = document.getElementById("start-text");
    const input = document.getElementById("file-input");

    input.addEventListener("change", (event) => {
        const file = event.target.files[0];
        decodeFile(file);
    });

    const eventHandler = (msg, callback) => (event) => {
        event.preventDefault();
        event.stopPropagation();
        start.innerHTML = msg;
        if (callback) {
            callback(event);
        }
    };

    dropHandler.addEventListener(
        "drop",
        eventHandler("Loading...", (event) => {
            if (event.dataTransfer.items) {
                [...event.dataTransfer.items].forEach((item, i) => {
                    // If dropped items aren't files, reject them
                    if (item.kind === "file") {
                        const file = item.getAsFile();
                        decodeFile(file);
                    }
                });
            } else {
                [...event.dataTransfer.files].forEach((file, i) => {
                    decodeFile(file);
                });
            }
        }),
        false
    );

    dropHandler.addEventListener("dragover", eventHandler("Dragging !"), false);

    dropHandler.addEventListener("dragenter", eventHandler("Drop now !"), false);

    dropHandler.addEventListener("dragleave", eventHandler("Drop here"), false);
};

addEventListener("TrunkApplicationStarted", startup);
