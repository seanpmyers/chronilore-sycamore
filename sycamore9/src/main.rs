use sycamore::prelude::*;

use crate::components::{
    icon::{MOON_SVG_HTML, SUN_SVG_HTML},
    theme_toggle::ThemeToggle,
};

pub mod components;

#[component]
fn App<G: Html>() -> View<G> {
    let mut state: Signal<i32> = create_signal(0);

    let increment = move |_| state += 1;

    view! {
        div {
            "Component demo"

            button(on:click=increment) { "+" }
            span() {(state.get())}
            ThemeToggle(text="", light_icon=SUN_SVG_HTML, dark_icon=MOON_SVG_HTML)
        }
    }
}

fn main() {
    sycamore::render(App);
}
