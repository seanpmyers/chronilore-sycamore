use leptos::*;
use uuid::Uuid;
use wasm_bindgen::JsCast;
use web_sys::{HtmlDialogElement, MouseEvent};

use crate::components::icon::CLOSE_X_SVG_HTML;

#[derive(PartialEq, Clone)]
pub enum ModalType {
    Default,
    SidePanelRight,
    SidePanelLeft,
}

#[component]
pub fn modal(
    click_out_to_close: ReadSignal<bool>,
    modal_type: ReadSignal<ModalType>,
) -> impl IntoView {
    let dialog_id: Uuid = Uuid::new_v4();

    let open_click_handler = move |_: MouseEvent| {
        open_dialog(&dialog_id.to_string());
    };

    let close_click_handler = move |_: MouseEvent| {
        close_dialog(&dialog_id.to_string());
    };

    let on_modal_click_handler = move |event: MouseEvent| {
        if !click_out_to_close.get() {
            return;
        }
        if let Some(html_element) = event.target() {
            if html_element.dyn_ref::<HtmlDialogElement>().is_none() {
                return;
            }
            close_dialog(&dialog_id.to_string())
        }
    };

    view! {
        <button on:click=open_click_handler> "Open modal" </button>
        <dialog id=dialog_id.to_string() on:click=on_modal_click_handler class=(move || match modal_type.get() {
            ModalType::Default => "modal",
            ModalType::SidePanelRight => "modal-side-panel-right",
            ModalType::SidePanelLeft => "modal-side-panel-left",
        })>
            <div class="modal-container">
                <button title="close" class="modal-close" on:click=close_click_handler>
                    <span inner_html={CLOSE_X_SVG_HTML} class="icon"></span>
                </button>
                <div></div>
            </div>
        </dialog>
    }
}

//TODO: fix error handling
pub fn close_dialog(dialog_html_id: &str) {
    let dialog_node = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id(dialog_html_id);
    if let Some(element) = dialog_node {
        element.unchecked_into::<HtmlDialogElement>().close();
    };
}

//TODO: fix error handling
pub fn open_dialog(dialog_html_id: &str) {
    let dialog_node = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id(dialog_html_id);
    if let Some(element) = dialog_node {
        element
            .unchecked_into::<HtmlDialogElement>()
            .show_modal()
            .unwrap();
    };
}
