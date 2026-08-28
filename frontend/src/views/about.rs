use iced::{ Alignment, Element, Length };
use iced::widget::{button, column, container, text};

use crate::Message;
use crate::needs::style::button_transparant_style;

pub fn view() -> Element<'static, Message> {
    let content = column![
        text("MorTask is a simple and modern To-Do List application\ndesigned to help you organize your tasks and stay productive."
        ).align_x(Alignment::Center).width(Length::Fill),
        
        button("Developed by ByteMort")
        .style(|_theme, status| button_transparant_style(status))
        .on_press(Message::OpenDeveloper),
    ]
    .align_x(Alignment::Center)
    .spacing(20)
    .width(Length::Fill);

    container(content)
    .center_x(Length::Fill)
    .center_y(Length::Fill)
    .into()
}
