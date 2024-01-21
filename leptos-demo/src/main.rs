use components::{theme_toggle::ThemeToggle, timer::Timer};
use leptos::*;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::console::log_1;

use crate::components::{
    icon::FERRIS_SVG_HTML,
    modal::{Modal, ModalType},
};

pub mod components;

pub const SEARCH_DIALOG_ID: &str = "search-dialog";

fn main() {
    let click_out_to_close: (ReadSignal<bool>, WriteSignal<bool>) = create_signal(true);
    let modal_type: (ReadSignal<ModalType>, WriteSignal<ModalType>) =
        create_signal(ModalType::SidePanelRight);

    let keypress_handle: gloo_events::EventListener =
        gloo_events::EventListener::new(&window(), "keydown", move |event| {
            let event: &web_sys::KeyboardEvent =
                event.dyn_ref::<web_sys::KeyboardEvent>().unwrap_throw();
            open_search(event.to_owned());
        });
    on_cleanup(|| drop(keypress_handle));

    mount_to_body(move || {
        view! {
            <div class="container" >
                <div class="top-nav">
                    <div class="top-nav-heading">
                        <span inner_html={FERRIS_SVG_HTML} class="logo"> </span>
                        <a href="/"> Rust UI </a>
                    </div>
                    <div>
                        <ThemeToggle />
                    </div>
                </div>
                <div class="side-nav"> </div>
                <main class="glass">
                    <div class=""> </div>
                    <p>"Hello, world!"</p>
                    <Timer initial_value=3 />
                    <Modal click_out_to_close=click_out_to_close.0 modal_type=modal_type.0/>
                </main>
             </div>
            <dialog id=SEARCH_DIALOG_ID class="modal">
                <div class="modal-container">
                    <div>
                        "Test"
                    </div>
                </div>
            </dialog>
        }
    })
}

pub fn open_search(event: web_sys::KeyboardEvent) {
    log_1(
        &(format!(
            "key: {} key_code:{} ctrl: {} guess:{}",
            event.key(),
            event.key_code(),
            event.ctrl_key(),
            ('k' as u32)
        ))
        .into(),
    );
    if !(event.ctrl_key() && event.key().to_ascii_lowercase() == "k") {
        log_1(&"Not ctrl+k".into());
        return;
    }
    if (event.ctrl_key() && event.code() == "KeyK") {
        event.prevent_default();
    }
    log_1(&"ctrl+k".into());

    let dialog_node: Option<web_sys::Element> = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id(SEARCH_DIALOG_ID);
    if let Some(element) = dialog_node {
        element
            .unchecked_into::<web_sys::HtmlDialogElement>()
            .show_modal()
            .unwrap();
    };
}
