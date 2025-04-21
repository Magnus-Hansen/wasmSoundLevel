Work in progress

Remember to run serialreader in microbit folder

build
wasm-pack  build --target web

Start the server
cargo run --release

Serve files
npx serve -l 8000


[Microbit]
└─▶ Sends mic value via UART (every 3s)
└─▶ [Rust Relay App on PC] → sends UDP to 127.0.0.1:8080

[Rust Server with Tokio]
└─▶ Listens on UDP (port 8080)
└─▶ Forwards values via WebSocket (port 9000)

[Browser Frontend (WASM/Rust)]
└─▶ Connects to WebSocket on ws://localhost:9000
└─▶ Displays live sound level using DOM API (via wasm-bindgen)
