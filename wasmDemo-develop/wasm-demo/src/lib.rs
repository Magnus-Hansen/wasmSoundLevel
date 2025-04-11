use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use std::rc::Rc;
use std::cell::RefCell;
use web_sys::{FileReader, HtmlElement, Event, DragEvent};
use async_std::net::UdpSocket;


#[wasm_bindgen]
pub fn getTemp() {
    #[async_std::main]
    async fn main() -> std::io::Result<()> {
        {
            let socket = UdpSocket::bind("127.0.0.1:8080").await?;
            println!("Listening on {}", socket.local_addr()?);

            let mut buf = vec![0u8; 1024];

            loop
            {
                let (recv, peer) = socket.recv_from(&mut buf).await?;
                let data = i32::from_be_bytes(buf[0..4].try_into().unwrap());

                println!("Received {} from {:?}", data, peer);
            }
        }
    }
}

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[wasm_bindgen]
pub fn setup_drag_and_drop(drop_zone_id: &str, output_id: &str) {
    let window = web_sys::window().expect("No window found");
    let document = window.document().expect("No document found");

    let drop_zone = document.get_element_by_id(drop_zone_id)
        .expect("Drop zone not found")
        .dyn_into::<HtmlElement>()
        .expect("Could not cast to HtmlElement");

    let output = document.get_element_by_id(output_id)
        .expect("Output not found");
    let output = Rc::new(RefCell::new(output));

    let closure = {
        let output = Rc::clone(&output);

        Closure::wrap(Box::new(move |event: DragEvent| {
            event.prevent_default();
            if let Some(data_transfer) = event.data_transfer() {
                if let Some(files) = data_transfer.files() {
                    if let Some(file) = files.item(0) {
                        let file_name = file.name();
                        let file_size = file.size();

                        let reader = FileReader::new().expect("Failed to create FileReader");
                        let reader_clone = reader.clone();

                        let output_clone = Rc::clone(&output);

                        let onload_callback = Closure::wrap(Box::new(move |_event: Event| {
                            let result = reader_clone.result().unwrap();
                            let url = result.as_string().unwrap();
                            
                            output_clone.borrow().set_inner_html(&format!(
                                "<p>File: {} ({} bytes)</p><img src='{}' style='max-width: 200px;'/>",
                                file_name, file_size, url
                            ));
                        }) as Box<dyn FnMut(_)>);

                        reader.set_onload(Some(onload_callback.as_ref().unchecked_ref()));
                        reader.read_as_data_url(&file).expect("Failed to read file");

                        onload_callback.forget();
                    }
                }
            }
        }) as Box<dyn FnMut(_)>)
    };

    drop_zone.set_ondrop(Some(closure.as_ref().unchecked_ref()));
    drop_zone.set_ondragover(Some(closure.as_ref().unchecked_ref()));

    closure.forget();
}
