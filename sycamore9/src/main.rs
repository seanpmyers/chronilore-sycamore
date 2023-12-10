use sycamore::{prelude::*, suspense::Suspense};
use web_sys::MouseEvent;

use crate::components::{
    icon::{MOON_SVG_HTML, SUN_SVG_HTML},
    theme_toggle::ThemeToggle,
    timer::Timer,
};

pub mod components;

#[component]
fn App<G: Html>() -> View<G> {
    let mut state: Signal<i32> = create_signal(0);

    let increment = move |_| state += 1;
    let hide_display: Signal<bool> = create_signal(false);
    let paused: Signal<bool> = create_signal(true);
    let reset: Signal<bool> = create_signal(true);
    let elapsed: Signal<f64> = create_signal(0_f64);
    let toggle_pause = move |_: MouseEvent| paused.set(!paused.get());
    let toggle_reset = move |_: MouseEvent| reset.set(true);
    let toggle_hide_display = move |_: MouseEvent| hide_display.set(!hide_display.get());

    view! {
        div {
            "Component demo"

            button(on:click=increment) { "+" }
            span() {(state.get())}
            ThemeToggle(text="", light_icon=SUN_SVG_HTML, dark_icon=MOON_SVG_HTML)
            button(on:click=toggle_pause) { "Start/Stop" }
            button(on:click=toggle_reset) { "Reset" }
            button(on:click=toggle_hide_display) { "Hide/Show" }
            Suspense(fallback=view!{ "Loading..." }) {
                Timer(hide_display=hide_display, reset=reset, paused=paused, elapsed=elapsed)
            }
        }
    }
}

fn main() {
    sycamore::render(App);
}
