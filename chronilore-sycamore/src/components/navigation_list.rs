use sycamore::prelude::*;

pub enum Direction {
    Vertical,
    Horizontal,
}

#[derive(Prop)]
pub struct NavigationListProperties<'list, G: Html> {
    pub direction: Direction,
    pub children: Children<'list, G>,
}

#[component]
pub fn NavigationList<'list, G: Html>(
    context: Scope<'list>,
    NavigationListProperties {
        direction,
        children,
    }: NavigationListProperties<'list, G>,
) -> View<G> {
    let children: View<G> = children.call(context);
    let list_class: &Signal<String> = match direction {
        Direction::Horizontal => create_signal(context, String::from("horizontal nav-list")),
        Direction::Vertical => create_signal(context, String::from("vertical nav-list")),
    };
    view! { context,
        ul(class=list_class) {
            (children)
        }
    }
}
