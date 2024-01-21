use leptos::*;

pub const LIGHT: &str = "light";
pub const DEFAULT_TOGGLE_TITLE: &str = "Switch color theme";
pub const BUTTON_CLASS: &str = "theme-toggle";

#[component]
pub fn theme_toggle() -> impl IntoView {
    view! {
        <button on:click=toggle_css_theme title=DEFAULT_TOGGLE_TITLE>
            "Theme Toggle"
        </button>
    }
}

fn toggle_css_theme(event: web_sys::MouseEvent) {
    event.prevent_default();
    let root: Option<web_sys::Element> = web_sys::window()
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
