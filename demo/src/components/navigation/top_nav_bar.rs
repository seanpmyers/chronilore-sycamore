use chronilore_sycamore::ThemeToggle;
use sycamore::prelude::*;

#[component]
pub fn TopNavBar<G: Html>(context: Scope) -> View<G> {
    let nav_classes: &str = "top-nav";

    view! {context,
        nav(class=nav_classes) {
            div(class="", style="flex: 1;") {
                a(href="/", id="", class="") { "Chronilore-Sycamore" }
            }
            ThemeToggle()
        }
    }
}
