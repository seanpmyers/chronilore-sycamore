use sycamore::prelude::*;
use web_sys::{window, Element, Event};

pub const LIGHT: &str = "light";
pub const DEFAULT_TOGGLE_TITLE: &str = "Switch color theme";
pub const BUTTON_CLASS: &str = "theme-toggle";

#[derive(Prop)]
pub struct ThemeToggleProperties<'toggle> {
    pub text: String,
    pub light_icon: &'toggle str,
    pub dark_icon: &'toggle str,
}

#[component]
pub fn ThemeToggle<'toggle, G: Html>(
    context: Scope<'toggle>,
    ThemeToggleProperties {
        text,
        light_icon,
        dark_icon,
    }: ThemeToggleProperties<'toggle>,
) -> View<G> {
    view! {context,
        button(
            class=BUTTON_CLASS,
            on:click=toggle_css_theme,
            title=DEFAULT_TOGGLE_TITLE) {
            span(dangerously_set_inner_html=dark_icon) {}
            span(dangerously_set_inner_html=light_icon) {}
            (if !text.is_empty() { view!{context, span() {(text)}} } else { view!{context, }})
        }
    }
}

fn toggle_css_theme(event: Event) {
    event.prevent_default();
    let root: Option<Element> = window()
        .unwrap()
        .document()
        .unwrap()
        .query_selector(":root")
        .unwrap();
    match root {
        Some(element) => element.class_list().toggle(LIGHT).unwrap(),
        None => false,
    };
}
