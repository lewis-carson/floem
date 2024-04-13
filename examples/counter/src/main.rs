use floem::{
    keyboard::{Key, Modifiers, NamedKey},
    peniko::Color,
    reactive::create_signal,
    unit::UnitExt,
    view::View,
    views::{label, stack, text, Decorators},
};
fn app_view() -> impl View {
    let (counter, set_counter) = create_signal(0); // Create a signal for the counter value and a setter function

    let view = stack((
        label(move || format!("Value: {}", counter.get())) // Display the current value of the counter
            .style(|s| s.padding(10.0)), // Apply padding to the label

        stack((
            text("Increment") // Button to increment the counter
                .style(|s| {
                    s.border_radius(10.0) // Apply border radius to the button
                        .padding(10.0) // Apply padding to the button
                        .background(Color::WHITE) // Set the background color of the button
                        .box_shadow_blur(5.0) // Apply box shadow to the button
                        .focus_visible(|s| s.outline(2.).outline_color(Color::BLUE)) // Show outline when button is focused
                        .hover(|s| s.background(Color::LIGHT_GREEN)) // Change background color on hover
                        .active(|s| s.color(Color::WHITE).background(Color::DARK_GREEN)) // Change text and background color when button is active
                })
                .on_click_stop({ // Handle click event for increment button
                    move |_| {
                        set_counter.update(|value| *value += 1); // Increment the counter value
                    }
                })
                .keyboard_navigatable(), // Make the button keyboard navigatable

            text("Decrement") // Button to decrement the counter
                .on_click_stop({ // Handle click event for decrement button
                    move |_| {
                        set_counter.update(|value| *value -= 1); // Decrement the counter value
                    }
                })
                .style(|s| {
                    s.box_shadow_blur(5.0) // Apply box shadow to the button
                        .background(Color::WHITE) // Set the background color of the button
                        .border_radius(10.0) // Apply border radius to the button
                        .padding(10.0) // Apply padding to the button
                        .margin_left(10.0) // Apply left margin to the button
                        .focus_visible(|s| s.outline(2.).outline_color(Color::BLUE)) // Show outline when button is focused
                        .hover(|s| s.background(Color::rgb8(244, 67, 54))) // Change background color on hover
                        .active(|s| s.color(Color::WHITE).background(Color::RED)) // Change text and background color when button is active
                })
                .keyboard_navigatable(), // Make the button keyboard navigatable

            text("Reset to 0") // Button to reset the counter to 0
                .on_click_stop(move |_| {
                    println!("Reset counter pressed"); // Print a message when the button is clicked
                    set_counter.update(|value| *value = 0); // Reset the counter value to 0
                })
                .disabled(move || counter.get() == 0) // Disable the button if the counter value is 0
                .style(|s| {
                    s.box_shadow_blur(5.0) // Apply box shadow to the button
                        .border_radius(10.0) // Apply border radius to the button
                        .padding(10.0) // Apply padding to the button
                        .margin_left(10.0) // Apply left margin to the button
                        .background(Color::LIGHT_BLUE) // Set the background color of the button
                        .focus_visible(|s| s.outline(2.).outline_color(Color::BLUE)) // Show outline when button is focused
                        .disabled(|s| s.background(Color::LIGHT_GRAY)) // Set background color when button is disabled
                        .hover(|s| s.background(Color::LIGHT_YELLOW)) // Change background color on hover
                        .active(|s| s.color(Color::WHITE).background(Color::YELLOW_GREEN)) // Change text and background color when button is active
                })
                .keyboard_navigatable(), // Make the button keyboard navigatable
        )),
    ))
    .style(|s| {
        s.size(100.pct(), 100.pct()) // Set the size of the view to 100% of the parent container
            .flex_col() // Arrange child elements in a column
            .items_center() // Center align child elements horizontally
            .justify_center() // Center align child elements vertically
    });

    let id = view.id(); // Get the ID of the view
    view.on_key_up(Key::Named(NamedKey::F11), Modifiers::empty(), move |_| {
        id.inspect() // Inspect the ID of the view when F11 key is pressed
    })
}

fn main() {
    floem::launch(app_view); // Launch the app with the app_view as the root view
}
