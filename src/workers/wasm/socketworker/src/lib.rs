#[path = "./client_to_server_generated.rs"]
mod client_to_server;
#[allow(dead_code, unused_imports)]
#[path = "./server_to_client_generated.rs"]
mod server_to_client;
#[allow(dead_code, unused_imports)]
mod shared_generated;
mod utils;

use std::convert::TryInto;

use wasm_bindgen::prelude::*;
use web_sys::{ErrorEvent, MessageEvent, WebSocket};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub struct SocketWorker {}

#[wasm_bindgen]
impl SocketWorker {
    pub fn new() -> Result<SocketWorker, JsValue> {
        // let ws = WebSocket::new("wss://api.pog.casino/")?;
        let ws = WebSocket::new("ws://localhost:4000/")?;
        ws.set_binary_type(web_sys::BinaryType::Arraybuffer);
        let cloned_ws = ws.clone();
        let onmessage_callback = Closure::<dyn FnMut(_)>::new(move |e: MessageEvent| {
            if let Ok(abuf) = e.data().dyn_into::<js_sys::ArrayBuffer>() {
                console_log!("message event, received arraybuffer: {:?}", abuf);
                let array = js_sys::Uint8Array::new(&abuf);
                let vec = array.to_vec();
                let message = server_to_client::root_as_root_message(&vec);
                if let Err(e) = message {
                    log(&format!("Invalid Flatbuffer: {e:?}"));
                    return;
                }

                let message = message.unwrap();

                if let Some(ping) = message.message_as_ping() {
                    let timestamp = std::time::UNIX_EPOCH
                        .checked_add(std::time::Duration::from_millis(
                            ping.timestamp().since_epoch(),
                        ))
                        .unwrap();
                    let data = ping.data();

                    log(&format!("Received Ping {timestamp:?} {data}"));

                    let mut builder = flatbuffers::FlatBufferBuilder::new();
                    let b_ref = &mut builder;

                    let root_message = {
                        let args = {
                            let args = {
                                let args = {
                                    shared_generated::UnixTimestampArgs {
                                        since_epoch: std::time::SystemTime::now()
                                            .duration_since(std::time::UNIX_EPOCH)
                                            .unwrap()
                                            .as_millis()
                                            .try_into()
                                            .unwrap(),
                                    }
                                };
                                client_to_server::PongArgs {
                                    data: data,
                                    timestamp: Some(shared_generated::UnixTimestamp::create(
                                        b_ref, &args,
                                    )),
                                }
                            };
                            client_to_server::RootMessageArgs {
                                message_type: client_to_server::Message::Pong,
                                message: Some(
                                    client_to_server::Pong::create(b_ref, &args).as_union_value(),
                                ),
                            }
                        };
                        client_to_server::RootMessage::create(b_ref, &args)
                    };
                    client_to_server::finish_root_message_buffer(b_ref, root_message);

                    cloned_ws
                        .send_with_u8_array(builder.finished_data())
                        .unwrap();
                } else if let Some(chat) = message.message_as_chat_message() {
                    let author = chat.author();
                    let content = chat.content();

                    log(&format!(
                        "Received Message from {}, {content}",
                        author.username()
                    ));
                } else {
                    log("Received unknown message type!!");
                }
            } else {
                console_log!("message event, received Unknown: {:?}", e.data());
            }
        });
        // set message event handler on WebSocket
        ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
        // forget the callback to keep it alive
        onmessage_callback.forget();

        let onerror_callback = Closure::<dyn FnMut(_)>::new(move |e: ErrorEvent| {
            console_log!("error event: {:?}", e);
        });
        ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
        onerror_callback.forget();

        let cloned_ws = ws.clone();
        let onopen_callback = Closure::<dyn FnMut()>::new(move || {
            console_log!("socket opened");
        });
        ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
        onopen_callback.forget();

        Ok(SocketWorker {})
    }
}

impl Drop for SocketWorker {
    fn drop(&mut self) {}
}
