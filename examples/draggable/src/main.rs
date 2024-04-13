use floem::{
    peniko::Color,
    view::View,
    views::{label, Decorators},
};

fn app_view() -> impl View {
    label(|| "Drag me!") // Create a label with the text "Drag me!"
        .style(|s| { // Apply styling to the label
            s.border(1.0) // Set a border with a width of 1.0
                .border_radius(2.0) // Set the border radius to 2.0
                .padding(10.0) // Add padding of 10.0
                .margin_left(10.0) // Add a left margin of 10.0
                .focus_visible(|s| s.border(2.).border_color(Color::BLUE)) // Set the border and border color when the label is focused
                .hover(|s| { // Apply styling when the label is hovered
                    s.background(Color::rgb8(244, 67, 54)) // Set the background color to RGB(244, 67, 54)
                        .border_radius(0.) // Set the border radius to 0.0
                        .border(2.) // Set a border with a width of 2.0
                        .border_color(Color::BLUE) // Set the border color to blue
                        .outline(2.) // Set an outline with a width of 2.0
                        .outline_color(Color::PALE_GREEN) // Set the outline color to pale green
                })
                .active(|s| s.color(Color::WHITE).background(Color::RED)) // Apply styling when the label is active
        })
        .keyboard_navigatable() // Make the label keyboard navigatable
        .draggable() // Make the label draggable
        .dragging_style(|s| { // Apply styling when the label is being dragged
            s.border(2.) // Set a border with a width of 2.0
                .border_color(Color::BLACK) // Set the border color to black
                .outline(20.) // Set an outline with a width of 20.0
                .outline_color(Color::RED) // Set the outline color to red
        })
}

fn main() {
    floem::launch(app_view);
}
