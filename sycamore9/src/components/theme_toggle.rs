use sycamore::prelude::*;
use web_sys::{window, Element, MouseEvent};

pub const LIGHT: &str = "light";
pub const DEFAULT_TOGGLE_TITLE: &str = "Switch color theme";
pub const BUTTON_CLASS: &str = "theme-toggle";

#[derive(Props)]
pub struct ThemeToggleProperties {
    pub text: &'static str,
    pub light_icon: &'static str,
    pub dark_icon: &'static str,
}

#[component]
pub fn ThemeToggle<G: Html>(
    ThemeToggleProperties {
        text,
        light_icon,
        dark_icon,
    }: ThemeToggleProperties,
) -> View<G> {
    view! {
        button(
            class=BUTTON_CLASS,
            on:click=toggle_css_theme,
            title=DEFAULT_TOGGLE_TITLE) {
            span(dangerously_set_inner_html=dark_icon) {}
            span(dangerously_set_inner_html=light_icon) {}
            (if !text.is_empty() {
                view!{ span() {(text)}}
            }
            else {
                view!{ }
            })

        }
    }
}

fn toggle_css_theme(event: MouseEvent) {
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
