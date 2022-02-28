#![allow(warnings)]
use log::debug;
use reqwasm::http::{Request, RequestMode};
use serde::{Deserialize, Serialize};
use std::borrow::Borrow;
use sycamore::prelude::*;
use sycamore::suspense::Suspense;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement};

#[derive(Serialize, Deserialize, Default, Debug)]
struct Data {
    val: String,
}

async fn fetch_file() -> Result<Data, reqwasm::Error> {
    let url = "http://127.0.0.1:8080/".to_string();
    let resp = Request::get(&url).send().await?;

    debug!("{:?}", resp);

    let body = resp.json::<Data>().await?;

    debug!("{:?}", body);

    Ok(body)
}

#[component]
async fn GetFile<G: Html>(ctx: ScopeRef<'_>) -> View<G> {
    let cont = fetch_file().await.unwrap();

    view! { ctx,
        div {
            h1 {
                "File Content: "
                (cont.val)
            }
        }
    }
}

async fn write_file(cont: String) -> Result<Data, reqwasm::Error> {
    let url = format!("http://127.0.0.1:8080/write/{}", cont);
    let resp = Request::get(&url).send().await?;

    debug!("{:?}", resp);

    let body = resp.json::<Data>().await?;

    debug!("{:?}", body);

    Ok(body)
}

struct WriteFileProps<'a> {
    content: String,
    written: &'a Signal<bool>,
}

#[component]
async fn WriteFile<'a, G: Html>(ctx: ScopeRef<'a>, props: WriteFileProps<'a>) -> View<G> {
    write_file(props.content.to_string()).await;

    props.written.set(false);

    view! { ctx,
        div {"Written!"}
    }
}

#[component]
fn App<G: Html>(ctx: ScopeRef) -> View<G> {
    let write_file_input = ctx.create_signal(String::new());
    // ↓ This one is the write_file_input's value for WriteFile ↓
    let actual_write_file_input = ctx.create_signal(String::new());
    let btn_pressed_signal = ctx.create_signal(false);

    let handle_write_file_input = |event: Event| {
        let target: HtmlInputElement = event.target().unwrap().unchecked_into();
        write_file_input.set(target.value());
        debug!("write_file_input {}", write_file_input.get());
        debug!("actual_write_file_input {}", actual_write_file_input.get());
    };

    let handle_btn_click = |event: Event| {
        actual_write_file_input.set(write_file_input.get().to_string());
        btn_pressed_signal.set(true);
    };

    view! { ctx,
        div {
            // Wrote the same thing for both cases so that it will re-render
            (if *btn_pressed_signal.get() {
                view! {ctx,
                Suspense {
                    fallback: view! { ctx, "Loading..." },
                    GetFile {}
                }}
            } else {
                view! {ctx,
                Suspense {
                    fallback: view! { ctx, "Loading..." },
                    GetFile {}
                }}
            })

            input(type="text",on:input=handle_write_file_input)



            (if *btn_pressed_signal.get() {
                view! {ctx,
                    Suspense {
                        fallback: view! { ctx, "Writing..." },
                        WriteFile(WriteFileProps {
                            content: actual_write_file_input.get().to_string(),
                            written: btn_pressed_signal,
                        })
                    }
                }
            } else {
                view! {ctx, button(on:click=handle_btn_click) {
                    "Write to file!"
                }}
            })
        }
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();

    sycamore::render(|ctx| view! { ctx, App {} });
}
