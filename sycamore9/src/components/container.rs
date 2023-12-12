use sycamore::prelude::*;

#[derive(Props)]
pub struct ContainerProperties<G: Html> {
    pub title: &'static str,
    pub children: Children<G>,
}

#[component]
pub fn Container<G: Html>(
    ContainerProperties { title, children }: ContainerProperties<G>,
) -> View<G> {
    let children: View<G> = children.call();
    view! {
        div(class=" container") {
            // TopNavBar()
            // SideNavBar()
            div(class="top-nav") {
                div(style="display:flex; flex: 1;") { "Sycamore-9 Demo" }
            }
            div() { "Side" }
            div(class="glass", id="main", data-title=title) {
                (children)
            }
        }
    }
}
