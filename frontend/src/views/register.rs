use iced::{Alignment, Color, Element, Font, Length, Padding, Task, font, widget::{Space, button, column, container, mouse_area, stack, svg, text, text_input}};

use crate::{API_URL, Message, State, needs::style::button_transparant_style};

#[derive(Debug, Default)]
pub struct RegisterUserInfo{
    pub username: String,
    pub email: String,
    pub password: String,
    pub show_password: bool,
}

#[derive(Debug, Clone)]
pub enum RegisterMessage{
    UsernameChanged(String),
    EmailChanged(String),
    PasswordChanged(String),
    ToggleShowPassword,
    RegisterPressed,
    RegisterStatus(Result<String, String>)
}

pub fn view(state: &State) -> Element<'_, Message>{
    let icon = svg(svg::Handle::from_path("assets/icons/mortask-icon.svg"))
        .width(70)
        .height(70);

    let title = text("REGISTER PAGE!")
        .size(24)
        .font(Font {
            weight: font::Weight::Bold,
            ..Font::DEFAULT
        })
        .color(Color::from_rgb8(200, 170, 255));

    let mut content = column![
        icon,
        title,
        Space::new().height(10)
    ].align_x(Alignment::Center)
    .spacing(10);

    if let Some(err) = state.errors.error_message.clone(){
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
    }else if let Some(scss) = state.errors.success_message.clone() {
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

    let username_input = text_input("Enter your username", &state.register_user_info.username)
        .on_input(|val| Message::Register(RegisterMessage::UsernameChanged(val)))
        .width(400)
        .padding(10);

    let email_input = text_input("Enter your email address", &state.register_user_info.email)
        .on_input(|val| Message::Register(RegisterMessage::EmailChanged(val)))
        .width(400)
        .padding(10);

    let password_input = text_input("Enter your password", &state.register_user_info.password)
        .on_input(|val| Message::Register(RegisterMessage::PasswordChanged(val)))
        .width(400)
        .padding(Padding {
            top: 10.0,
            right: 45.0,
            bottom: 10.0,
            left: 10.0,
        })
        .secure(!state.register_user_info.show_password);

    let toggle_icon_path = if state.register_user_info.show_password {
        "assets/icons/eye-open.svg"
    }else{
        "assets/icons/eye-closed.svg"
    };

     let toggle_button = container(
        button(svg(svg::Handle::from_path(toggle_icon_path)).width(20).height(20))
            .on_press(Message::Register(RegisterMessage::ToggleShowPassword))
            .style(|_theme, status| button_transparant_style(status)),
    )
    .width(400)
    .height(Length::Shrink)
    .align_x(Alignment::End)
    .align_y(Alignment::Center)
    .padding(8);

    let password_stack = stack![password_input, toggle_button];

    let link_to_page = mouse_area(
        text("Do you already have an account?")
        .style(|_theme| text::Style { color: Some(Color::from_rgb8(200, 170, 255)) })
        ).on_press(Message::GoToLogin);

    let register_button = button(
        container(text("REGISTER"))
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center),
    )
    .width(100)
    .height(40)
    .style(|_theme, status| button_transparant_style(status))
    .on_press(Message::Register(RegisterMessage::RegisterPressed));

    content = content
    .push(username_input)
    .push(email_input)
    .push(password_stack)
    .push(Space::new().height(3))
    .push(link_to_page)
    .push(register_button);

    container(
        content
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .align_x(Alignment::Center)
    .align_y(Alignment::Center)
    .padding(Padding{
        bottom: 110.0,
        ..Default::default()
    })
    .into()
}

pub fn register(username: String, email: String, password: String) -> Task<Message>{
    Task::perform(async move{

        let client = reqwest::Client::new();

        let mut payload = serde_json::Map::new();

        if !username.trim().is_empty() {
            payload.insert("username".to_string(), serde_json::Value::String(username));
        }
        if !email.trim().is_empty(){
            payload.insert("email".to_string(), serde_json::Value::String(email));
        }
        if !password.trim().is_empty(){
            payload.insert("password".to_string(), serde_json::Value::String(password));
        }

        let response = client
        .post(format!("{}/{}", API_URL, "register"))
        .json(&payload)
        .send()
        .await;

        match response {
            Ok(res) if res.status() == 201 => {
                match res.text().await {
                    Ok(_body) => Ok("Successfully Registered!".to_string()),
                    Err(err) => Err(format!("Registered but bad: {}", err)),
                }
            }
            Ok(res) => {
                let status = res.status();
                match res.text().await {
                    Ok(body) if !body.is_empty() => Err(body),
                    _ => Err(format!("HTTP Error: {}", status)),
                }
            }
            Err(_err) => {
                Err("There is a temporary issue with the background connection.".to_string())
            }
        }

    }, |result| Message::Register(RegisterMessage::RegisterStatus(result)))
}