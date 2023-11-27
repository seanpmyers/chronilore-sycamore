use futures_util::{future::ready, stream::StreamExt};
use gloo_timers::future::IntervalStream;
use sycamore::prelude::*;
use time::Instant;

#[derive(Props)]
pub struct TimerProperties {
    pub stopwatch: &'static str,
}

#[component]
pub async fn Timer<G: Html>(TimerProperties { stopwatch }: TimerProperties) -> View<G> {
    let stopwatch: Signal<&'static str> = create_signal(stopwatch);
    if G::IS_BROWSER {
        let rust_time = Instant::now();
        IntervalStream::new(0)
            .for_each(|_| {
                // stopwatch.set();
                ready(())
            })
            .await;
    }

    view! {
        div() { (stopwatch.get_clone()) }
    }
}
