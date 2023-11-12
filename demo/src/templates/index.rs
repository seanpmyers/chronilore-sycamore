use perseus::prelude::*;
use sycamore::prelude::*;

use crate::components::container::Container;

fn index_page<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        Container(title="Chronilore-Sycamore") {
            // Don't worry, there are much better ways of styling in Perseus!
            div(class="") {
                h1 { "Demo" }
            }
        }
    }
}

#[engine_only_fn]
fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "Demo" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("index").view(index_page).head(head).build()
}
