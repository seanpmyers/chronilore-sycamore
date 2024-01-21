use futures_util::{future::ready, stream::StreamExt};
use gloo_timers::future::IntervalStream;
use js_sys::Date;

use leptos::*;

const SECONDS_IN_MILLISECOND: f64 = 1000_f64;
const TIMER_UPDATE_FREQUENCY_MILLISECONDS: u32 = 2;
const ZERO_SECONDS: f64 = 0_f64;

#[component]
pub fn timer(initial_value: i32) -> impl IntoView {
    // create a reactive signal with the initial value
    let (value, set_value) = create_signal(initial_value);

    // create event handlers for our buttons
    // note that `value` and `set_value` are `Copy`, so it's super easy to move them into closures
    let clear = move |_| set_value.set(0);
    let decrement = move |_| set_value.update(|value| *value -= 1);
    let increment = move |_| set_value.update(|value| *value += 1);

    // create user interfaces with the declarative `view!` macro
    view! {
        <div>
            <button on:click=clear>Clear</button>
            <button on:click=decrement>-1</button>
            // text nodes can be quoted or unquoted
            <span>"Value: " {value} "!"</span>
            <button on:click=increment>+1</button>
        </div>
    }
}
