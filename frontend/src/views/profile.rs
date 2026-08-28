use iced::{Alignment, Border, Color, Element, Length, Padding, Task, widget::{Space, button, column, container, row, svg, text}};
use reqwest::header::COOKIE;
use serde::{Deserialize, Serialize};

use crate::{API_URL, Message, State, needs::help_methods::format_date_time, views::login::{LogoutMessage, Token}};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ProfileRole{
    User,
    Admin
}

#[derive(Debug, Clone, Deserialize)]
pub struct Profile{
    pub username: String,
    pub email: String,
    pub created_at: String,
    pub role: ProfileRole,
    pub success_msg: Option<String>,
    pub error_msg: Option<String>
}

pub fn view(state: &State) -> Element<'_, Message> {
    let username = text(format!("{}", state.profile.as_ref().unwrap().username));

    let user_icon = container(
        column![
            svg(svg::Handle::from_path("assets/icons/user.svg"))
                .width(50)
                .height(50),
            username
        ]
        .spacing(12)
        .align_x(Alignment::Center)
    )
    .style(|_theme| container::Style {
        border: Border {
            color: Color::from_rgb8(200, 170, 255),
            width: 1.0,
            radius: 30.0.into(),
        },
        ..Default::default()
    })
    .padding(15)
    .width(Length::Shrink)
    .height(Length::Shrink)
    .align_x(Alignment::Center)
    .align_y(Alignment::Center);

    let email_row = row![
        row![
            svg(svg::Handle::from_path("assets/icons/mail.svg"))
                .width(25)
                .height(25),
            text("Email")
        ]
        .spacing(10) 
        .align_y(Alignment::Center),
        
        Space::new().width(Length::Fill),

        text(format!("{}", state.profile.as_ref().unwrap().email))
    ]
    .width(550)
    .align_y(Alignment::Center);

    let role_row = row![
        row![
            svg(svg::Handle::from_path("assets/icons/shield.svg"))
                .width(25)
                .height(25),
            text("Role")
        ]
        .spacing(10) 
        .align_y(Alignment::Center),
        
        Space::new().width(Length::Fill),

        text(format!("{:?}", state.profile.as_ref().unwrap().role))
    ]
    .width(550)
    .align_y(Alignment::Center);

    let created_at_row = row![
        row![
            svg(svg::Handle::from_path("assets/icons/clock.svg"))
                .width(25)
                .height(25),
            text("Member since")
        ]
        .spacing(10) 
        .align_y(Alignment::Center),
        
        Space::new().width(Length::Fill),

        text(format!("{}", format_date_time(&(state.profile.as_ref().unwrap().created_at))))
    ]
    .width(550)
    .align_y(Alignment::Center);

    let logout_button = button(
        row![
            svg(svg::Handle::from_path("assets/icons/log-out.svg")),
            text("Logout!")
        ].spacing(25)
    )
    .style(|_theme, _status| button::Style { 
        background: Some(iced::Background::Color(Color::from_rgb8(89, 41, 46))),
        text_color: Color::from_rgb8(245, 225, 226),
        border: Border {
            color: Color::from_rgb8(191, 89, 96),
            width: 1.0,
            radius: 8.0.into(),
        },
        ..Default::default()
    })
    .width(Length::Shrink)
    .height(Length::Shrink)
    .on_press(Message::Logout(LogoutMessage::LogoutPressed))
    .padding(10);

    let mut content = column![
        user_icon,
        
    ].spacing(30)
    .align_x(Alignment::Center);

    if let Some(err) = state.profile.clone().unwrap().error_msg{
        content = content.push(
            container(
                text(err)
            )
            .padding(8)
            .style(|_theme| container::Style{
                background: Some(iced::Background::Color(Color::from_rgb8(140, 51, 51))),
                ..Default::default()
            })
        )
        .push(Space::new().height(10));
    }else if let Some(scss) = state.profile.clone().unwrap().success_msg {
        content = content.push(
            container(
                text(format!("{} - Redirecting...", scss))
            )
            .padding(8)
            .style(|_theme| container::Style{
                background: Some(iced::Background::Color(Color::from_rgb8(38, 108, 90))),
                ..Default::default()
            })
        )
        .push(Space::new().height(10));
    }

    content = content.push(email_row)
    .push(role_row)
    .push(created_at_row)
    .push(logout_button);

    container(
        content
    )
    .style(|_theme| container::Style{
        border: Border { 
            color: Color::from_rgb8(200, 170, 255), 
            width: 2.0, 
            radius: 30.0.into()
        },
        ..Default::default()
    })
    .width(Length::Shrink)
    .height(Length::Shrink)
    .align_x(Alignment::Center)
    .align_y(Alignment::Center)
    .padding(Padding{
        top: 25.0,
        bottom: 25.0,
        left: 25.0,
        right: 25.0
    })
    .into()
}

#[derive(Debug, Clone)]
pub enum ProfileMessage{
    ProfileStatus(Result<Profile, String>),
    ProfilePage
}

pub fn get_my_profile(token: Token) -> Task<Message>{
    Task::perform(async move {
        let client = reqwest::Client::new();

        let mut cookies = Vec::new();
        
        if let Some(t) = token.token{
            cookies.push(format!("token={}", t));
        }
        if let Some(rt) = token.refresh_token{
            cookies.push(format!("refresh_token={}", rt));
        }

        let cookie_header_value = cookies.join("; ");

        let response = client
            .get(format!("{}/{}", API_URL, "me"))
            .header(COOKIE, cookie_header_value)
            .send()
            .await;

        match response {
            Ok(r) => {
                if r.status() == 200{
                    match r.json::<Profile>().await {
                        Ok(profile_data) => {
                            Ok(profile_data)
                        }
                        Err(_err) => {
                            Err("JSON could not be parsed".to_string())
                        }
                    }
                }else if r.status() == 401{
                    Err(r.text().await.unwrap())
                }else{
                    Err("We encountered a temporary issue. Please try again.".to_string())
                }
            },
            Err(_err) => Err(format!("There is a temporary issue with the background connection.")),
        }

    }, |result| Message::Profile(ProfileMessage::ProfileStatus(result)))
}