use sycamore::prelude::*;

const NAV_CLASSES: &str = "side-nav";
const NAV_UL_CLASSES: &str = "side-nav-container";
const NAV_LI_CLASSES: &str = "side-nav-item";
const A_CLASS: &str = "";
const NAV_BUTTON_ICON_CLASS: &str = "";
const NAV_BUTTON_TEXT_CLASS: &str = "big-nav-button-text";

#[component]
pub fn SideNavBar<G: Html>(context: Scope) -> View<G> {
    view! {context,
        nav(class=NAV_CLASSES) {
            ul(class=NAV_UL_CLASSES) {
                li(class=NAV_LI_CLASSES) {
                    a(class=A_CLASS, id="", href="/") {
                        span(class=NAV_BUTTON_ICON_CLASS,dangerously_set_inner_html="") {}
                        span(class=NAV_BUTTON_TEXT_CLASS) {"Home"}

                    }
                }
            }
        }
    }
}

#[component]
pub fn DesignSystemLink<G: Html>(context: Scope) -> View<G> {
    view! { context,
       li(class=NAV_LI_CLASSES) {
           a(
               class=A_CLASS,
               id="",
               href=""
           ) {
               span(class=NAV_BUTTON_ICON_CLASS,dangerously_set_inner_html="") {}
               span(class=NAV_BUTTON_TEXT_CLASS) { "" }

           }
       }
    }
}
