use perseus::prelude::*;
use sycamore::prelude::*;
use sycamore::suspense::Suspense;

use crate::components::container::Container;
use chronilore_sycamore::components::timer::Timer;

fn index_page<G: Html>(context: Scope) -> View<G> {
    view! { context,
        Container(title="Chronilore-Sycamore") {
            // Don't worry, there are much better ways of styling in Perseus!
            div(class="") {
                h1 { "Demo" }
                Timer(stopwatch=String::new()).delayed_widget()
            }
        }
    }
}

#[engine_only_fn]
fn head(context: Scope) -> View<SsrNode> {
    view! { context,
        title { "Demo" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("index").view(index_page).head(head).build()
}
