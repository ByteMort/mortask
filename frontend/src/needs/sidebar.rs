use iced::{Alignment, Border, Color, Element, Length};
use iced::widget::{column, container, text};

use crate::needs::style::animated_button;
use crate::{Message, State};

pub fn view(state: &State) -> Element<'_, Message>{
    if state.sidebar_open{
        
        let mut profile_status = Message::GoToLogin;
        if let Some(_profile) = &state.profile{
            profile_status = Message::GoToProfile;
        }

        let bordered_box = container(
            column![
                animated_button(state, None, Some("assets/icons/user.svg"), profile_status),
                animated_button(state, None, Some("assets/icons/tasks.svg"), Message::GoToTasks)
            ]
            .spacing(20)
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(15)
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
        .style(|_theme| container::Style {
            border: Border {
                color: Color::from_rgb8(80, 80, 100),
                width: 1.0,
                radius: 8.0.into(),
            },
            ..Default::default()
        });

        container(bordered_box)
            .width(100)
            .height(Length::Fill)
            .padding(10)
            .into()
    }else{
        container(text(""))
        .width(0)
        .height(Length::Fill)
        .into()
    }
}