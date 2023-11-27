// use futures_util::{future::ready, stream::StreamExt};
// use gloo_timers::future::IntervalStream;
use sycamore::prelude::*;
// use time::Instant;

#[derive(Prop)]
pub struct TimerProperties {
    pub stopwatch: String,
}

#[component]
pub async fn Timer<'timer, G: Html>(
    context: Scope<'timer>,
    TimerProperties { stopwatch }: TimerProperties,
) -> View<G> {
    let stopwatch: &Signal<String> = create_signal(context, stopwatch);
    if G::IS_BROWSER {
        // let rust_time = Instant::now();
        // IntervalStream::new(0)
        //     .for_each(|_| {
        //         stopwatch.set(format!(""));
        //         ready(())
        //     })
        //     .await;
    }

    view! {context,
        div() { (stopwatch.get()) }
    }
}
