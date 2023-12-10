use futures_util::{future::ready, stream::StreamExt};
use gloo_timers::future::IntervalStream;
use js_sys::Date;
use sycamore::{futures::spawn_local_scoped, prelude::*};

const SECONDS_IN_MILLISECOND: f64 = 1000_f64;
const TIMER_UPDATE_FREQUENCY_MILLISECONDS: u32 = 2;
const ZERO_SECONDS: f64 = 0_f64;

#[derive(Props)]
pub struct TimerProperties {
    pub reset: Signal<bool>,
    pub paused: Signal<bool>,
    pub elapsed: Signal<f64>,
}

#[component]
pub async fn Timer<G: Html>(
    TimerProperties {
        reset,
        paused,
        elapsed,
    }: TimerProperties,
) -> View<G> {
    let browser_time: Signal<f64> = create_signal(Date::new_0().get_time());
    let stopwatch: ReadSignal<String> = create_memo(move || format!("{:.2}", elapsed.get()));
    let time_at_pause: Signal<f64> = create_signal(ZERO_SECONDS);
    if G::IS_BROWSER {
        create_effect(move || {
            paused.track();
            if paused.get() && !reset.get() {
                time_at_pause.set(
                    time_at_pause.get()
                        + ((Date::new_0().get_time() - browser_time.get())
                            / SECONDS_IN_MILLISECOND),
                );
            } else {
                browser_time.set(Date::new_0().get_time());
            }
        });
        create_effect(move || {
            reset.track();
            reset.get();
            paused.set(true);
            elapsed.set(ZERO_SECONDS);
            time_at_pause.set(ZERO_SECONDS);
            reset.set(false);
        });
        stopwatch.track();
        reset.track();
        paused.track();
        browser_time.track();
        time_at_pause.track();
        elapsed.track();
        spawn_local_scoped(async move {
            IntervalStream::new(TIMER_UPDATE_FREQUENCY_MILLISECONDS)
                .for_each(|_| {
                    if paused.get() {
                        return ready(());
                    }
                    elapsed.set(
                        time_at_pause.get()
                            + ((Date::new_0().get_time() - browser_time.get())
                                / SECONDS_IN_MILLISECOND),
                    );
                    ready(())
                })
                .await;
        });
    }

    view! {
        div() { "Timer" }
        div() { (stopwatch.get_clone()) }
    }
}
