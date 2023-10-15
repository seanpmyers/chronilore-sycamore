use sycamore::prelude::*;
use web_sys::{window, Event};

#[component]
pub fn ThemeToggle<G: Html>(context: Scope) -> View<G> {
    let theme_switch_handler = move |event: Event| {
        event.prevent_default();
        let root = window()
            .unwrap()
            .document()
            .unwrap()
            .query_selector(":root")
            .unwrap();
        match root {
            Some(element) => element.class_list().toggle("light").unwrap(),
            None => false,
        };
    };
    view! {context,
        button(
            on:click=theme_switch_handler,
            title="Switch color theme") {
            span(dangerously_set_inner_html="") {}
            span(dangerously_set_inner_html="") {}
            span() {"Theme Toggle"}
        }
    }
}
