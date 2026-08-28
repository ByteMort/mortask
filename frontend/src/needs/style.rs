use iced::{
    Background, Border, Color, Element, Theme, widget::{self, button, mouse_area, svg, text}
};
use iced_anim::animation::animation;

use crate::{Message, State};

pub fn button_style(
    _theme: &Theme,
    status: iced::widget::button::Status
) -> iced::widget::button::Style{
    
    match status {
        button::Status::Active => button::Style {
            background: None,
            text_color: Color::from_rgb8(180, 180, 185),
            ..Default::default()
        },

        button::Status::Hovered => button::Style {
            background: Some(
                Color::from_rgb8(55, 38, 70).into()
            ),
            text_color: Color::WHITE,
            ..Default::default()
        },

        button::Status::Pressed => button::Style {
            background: Some(
                Color::from_rgb8(45, 30, 58).into()
            ),
            text_color: Color::WHITE,
            ..Default::default()
        },

        button::Status::Disabled => button::Style {
            background: None,
            text_color: Color::from_rgb8(100, 100, 100),
            ..Default::default()
        },
    }
}

pub fn button_transparant_style(status: iced::widget::button::Status)
-> widget::button::Style {
    let background = match status{
        button::Status::Hovered => Some(Background::Color(Color::from_rgb8(55, 38, 70))),
        button::Status::Active => Some(Background::Color(Color::from_rgb8(80, 80, 100))),
        _ => Some(Background::Color(Color::TRANSPARENT)),
    };

    button::Style {
        background,
        text_color: Color::WHITE,
        border: Border{
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 20.0.into()
        },
        shadow: Default::default(),
        snap: Default::default()
    }
}

pub fn animated_button<'a>(
    state: &'a State,
    content: Option<&'a str>,
    icon_path: Option<&'a str>,
    msg: Message
) -> Element<'a, Message> {
    let icon_size = *state.sidebar_button_size.value();

    let button_content: Element<'a, Message> = if let Some(path) = icon_path {
        svg(svg::Handle::from_path(path))
            .width(icon_size)   
            .height(icon_size)
            .into()
    } else if let Some(label) = content {
        text(label).into()
    } else {
        text("T").into()
    };


    mouse_area(
        animation(
            &state.sidebar_button_size,
            button(button_content)
                .padding(10)
                .style(|_theme, status| button_transparant_style(status))
                .on_press(msg)
        )
        .on_update(Message::SidebarButtonAnimation)
        
    )
    .on_enter(Message::HoverSidebarButton(true))
    .on_exit(Message::HoverSidebarButton(false))
    .into()
}

pub fn priority_color(priority: u8) -> Color {
    let t = (priority.saturating_sub(1)) as f32 / 9.0;  

    let start = (255.0, 220.0, 80.0);  
    let end = (255.0, 60.0, 60.0);      

    let r = start.0 + (end.0 - start.0) * t;
    let g = start.1 + (end.1 - start.1) * t;
    let b = start.2 + (end.2 - start.2) * t;

    Color::from_rgb8(r as u8, g as u8, b as u8)
}