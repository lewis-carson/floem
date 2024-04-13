use floem::{
    reactive::{create_rw_signal, RwSignal},
    view::View,
    views::{dyn_container, h_stack, label, v_stack, Decorators},
    widgets::button,
};

#[derive(Clone, PartialEq)]
enum ViewSwitcher {
    One,
    Two,
}

fn view_one() -> impl View {
    label(|| "A view")
}

// Define a function `view_two` that takes a `RwSignal<ViewSwitcher>` as input and returns an implementation of the `View` trait.
fn view_two(view: RwSignal<ViewSwitcher>) -> impl View {
    // Create a vertical stack of views.
    v_stack((
        // Create a label with the text "Another view".
        label(|| "Another view"),
        // Create a button with the text "Switch back" and attach an event handler to it.
        button(|| "Switch back").on_click_stop(move |_| {
            // When the button is clicked, set the value of the `view` signal to `ViewSwitcher::One`.
            view.set(ViewSwitcher::One);
        }),
    ))
    // Apply a style to the vertical stack, setting the gap between views to 0.0 vertically and 10.0 horizontally.
    .style(|s| s.gap(0.0, 10.0))
}

fn app_view() -> impl View {
    let view = create_rw_signal(ViewSwitcher::One);

    v_stack((
        h_stack((
            label(|| "Swap views:").style(|s| s.padding(5)),
            button(|| "Switch views")
                .on_click_stop(move |_| {
                    if view.get() == ViewSwitcher::One {
                        view.set(ViewSwitcher::Two);
                    } else {
                        view.set(ViewSwitcher::One);
                    }
                })
                .style(|s| s.margin_bottom(20)),
        )),
        dyn_container(
            move || view.get(),
            move |value| match value {
                ViewSwitcher::One => view_one().any(),
                ViewSwitcher::Two => view_two(view).any(),
            },
        )
        .style(|s| s.padding(10).border(1)),
    ))
    .style(|s| {
        s.width_full()
            .height_full()
            .items_center()
            .justify_center()
            .gap(10, 0)
    })
}

fn main() {
    floem::launch(app_view);
}
