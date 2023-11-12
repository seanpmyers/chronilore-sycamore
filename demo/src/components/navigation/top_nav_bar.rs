use chronilore_sycamore::components::icon::{self, MOON_SVG_HTML, SUN_SVG_HTML};
use chronilore_sycamore::components::theme_toggle::ThemeToggle;
use sycamore::prelude::*;

#[component]
pub fn TopNavBar<G: Html>(context: Scope) -> View<G> {
    let nav_classes: &str = "top-nav";

    view! {context,
        nav(class=nav_classes) {
            div(class="", style="flex: 1; display: flex;") {
                a(href="/", id="", class="button hover top-nav-heading") {
                    span(dangerously_set_inner_html=icon::DNA_SVG_HTML) {}
                    "Chronilore-Sycamore"
                }
            }
            ThemeToggle(text=String::new(), light_icon=SUN_SVG_HTML, dark_icon=MOON_SVG_HTML )
        }
    }
}
