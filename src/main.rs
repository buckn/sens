use iced::{button, Button, Column, Element, Sandbox, Settings, Text, Length, HorizontalAlignment, Image, Container, scrollable, Space};

pub fn main() {
    Counter::run(Settings::default())
}

#[derive(Default)]
struct Counter {
    value: i32,
    increment_button: button::State,
    decrement_button: button::State,
    container: scrollable::State,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    IncrementPressed,
    DecrementPressed,
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("A simple counter")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
            }
            Message::DecrementPressed => {
                self.value -= 1;
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
            Column::new()
            .push(Column::new()
            .padding(20)
            .push(
                Button::new(&mut self.increment_button, Text::new("Increment"))
                    .on_press(Message::IncrementPressed),
            )
            .push(Text::new(self.value.to_string()).size(50))
            .push(
                Button::new(&mut self.decrement_button, Text::new("Decrement"))
                    .on_press(Message::DecrementPressed),
            )
            .into())
            .push(Text::new(
                "Iced supports scrollable content. Try it out! Find the \
                 button further below.",
            ))
            .push(
                Text::new(
                    "Tip: You can use the scrollbar to scroll down faster!",
                )
                .size(16),
            )
            .push(Column::new().height(Length::Units(4096)))
            .push(
                Text::new("You are halfway there!")
                    .size(30)
                    .horizontal_alignment(HorizontalAlignment::Center),
            )
            .push(Column::new().height(Length::Units(4096)))
            .push(ferris(300))
            .push(
                Text::new("You made it!")
                    .size(50)
                    .horizontal_alignment(HorizontalAlignment::Center),
            )
    }
}