use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebSocket, MessageEvent, window};

// This is Client side
// Establishes a WebSocket connection to a server and listens for messages

#[wasm_bindgen(start)]
pub fn main_js() {
    wasm_bindgen_futures::spawn_local(start_websocket());
}

pub async fn start_websocket() {


    let ws = WebSocket::new("ws://localhost:9000").unwrap();

    // data refers to the data received as Rust code
    let onmessage_callback = Closure::wrap(Box::new(move |event: MessageEvent| {
        if let Ok(data) = event.data().dyn_into::<js_sys::JsString>() {
            if let Some(value) = data.as_string() {
                if let Ok(num) = value.parse::<i32>() {
                    display_sound_level_in_browser(num);
                }
            }

            //let data = data.as_string().unwrap_or_default();
            //web_sys::console::log_1(&format!("Received: {}", data).into());
        }
    }) as Box<dyn FnMut(_)>);

    // Registers the onmessage callback. forget() is used to prevent the closure from being garbage collected.
    ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
    onmessage_callback.forget();
}
#[wasm_bindgen]
pub fn display_string_in_browser(message: &str) {
    let window = window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    // Create a new div element
    let div = document.create_element("div").expect("should create a div");

    // Set the div inner HTML to the message
    div.set_inner_html(message);

    document.body().expect("document should have a body").append_child(&div).expect("Should append div to body");
}

pub fn display_sound_level_in_browser(sound_level: i32) {
    let window = window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    // Create a new div element
    let div = document.create_element("div").expect("should create a div");

    // Set the div inner HTML to the sound level
    div.set_inner_html(&format!("Sound Level: {}", sound_level));

    document.body().expect("document should have a body").append_child(&div).expect("Should append div to body");
}


