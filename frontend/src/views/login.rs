use iced::{Alignment, Color, Element, Font, Length, Padding, Task, font, widget::{Space, button, column, container, mouse_area, stack, svg, text, text_input}};
use reqwest::header::COOKIE;

use crate::{API_URL, Message, State, needs::{help_methods::{extract_message, extract_tokens_from_cookies}, style::button_transparant_style}};

#[derive(Default, Debug)]
pub struct LoginUserInfo{
    pub email: String,
    pub password: String,
    pub show_password: bool,
    pub code: String,
    pub code_sended: bool,
}

#[derive(Default, Debug)]
pub struct MyErrors{
    pub error_message: Option<String>,
    pub success_message: Option<String>
}

#[derive(Debug, Clone)]
pub enum LoginError{
    Forbidden(String),
    Other(String)
}

#[derive(Debug, Clone)]
pub enum LoginMessage{
    EmailChanged(String),
    PasswordChanged(String),
    ToggleShowPassword,
    LoginPressed,
    LoginStatus(Result<(String, Option<String>, Option<String>), LoginError>)
}

#[derive(Debug, Clone, Default)]
pub struct Token{
    pub token: Option<String>,
    pub refresh_token: Option<String>
}

pub fn view(state: &State) -> Element<'_, Message> {
    let icon = svg(svg::Handle::from_path("assets/icons/mortask-icon.svg"))
        .width(70)
        .height(70);

    let title = text("LOGIN PAGE!")
        .size(24)
        .font(Font {
            weight: font::Weight::Bold,
            ..Font::DEFAULT
        })
        .color(Color::from_rgb8(200, 170, 255));

    let email_input = text_input("Enter your email address", &state.login_user_info.email)
        .on_input(|val| Message::Login(LoginMessage::EmailChanged(val)))
        .width(400)
        .padding(10);

    let password_input = text_input("Enter your password", &state.login_user_info.password)
        .on_input(|val| Message::Login(LoginMessage::PasswordChanged(val)))
        .width(400)
        .padding(Padding {
            top: 10.0,
            right: 45.0,
            bottom: 10.0,
            left: 10.0,
        })
        .secure(!state.login_user_info.show_password);

    let toggle_icon_path = if state.login_user_info.show_password {
        "assets/icons/eye-open.svg"
    } else {
        "assets/icons/eye-closed.svg"
    };

    let toggle_button = container(
        button(svg(svg::Handle::from_path(toggle_icon_path)).width(20).height(20))
            .on_press(Message::Login(LoginMessage::ToggleShowPassword))
            .style(|_theme, status| button_transparant_style(status)),
    )
    .width(400)
    .height(Length::Shrink)
    .align_x(Alignment::End)
    .align_y(Alignment::Center)
    .padding(8);

    let password_stack = stack![password_input, toggle_button];

    let login_button = button(
        container(text("LOGIN"))
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center),
    )
    .width(100)
    .height(40)
    .style(|_theme, status| button_transparant_style(status))
    .on_press(Message::Login(LoginMessage::LoginPressed));

    let mut content = column![
        icon,
        title,
        Space::new().height(10),
    ]
    .align_x(Alignment::Center)
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

    let link_to_page = mouse_area(
        text("Don't you have an account yet?")
        .style(|_theme| text::Style { color: Some(Color::from_rgb8(200, 170, 255)) })
        ).on_press(Message::GoToRegister);

    content = content
    .push(email_input)
    .push(password_stack)
    .push(Space::new().height(3))
    .push(link_to_page)
    .push(login_button);

    let main_container = container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
        .padding(Padding{
            bottom: 110.0,
            ..Default::default()
        });

    main_container.into()
}

pub fn login(email: String, password: String) -> Task<Message> {
    Task::perform(async move{
        let client = reqwest::Client::new();

        let mut payload = serde_json::Map::new();

        if !email.trim().is_empty(){
            payload.insert("email".to_string(), serde_json::Value::String(email));
        }
        if !password.trim().is_empty(){
            payload.insert("password".to_string(), serde_json::Value::String(password));
        }

        let response = client
            .post(format!("{}/{}", API_URL, "login"))
            .json(&payload)
            .send()
            .await;

        match response {
            Ok(res) if res.status() == 200 => {
                let (request_token, refresh_token) = extract_tokens_from_cookies(res.headers());

                match res.text().await {
                    Ok(body) => Ok((body, request_token, refresh_token)),
                    Err(err) => Err(LoginError::Other(format!("Loged in but bad: {}", err))),
                }
            }
            Ok(res) if res.status() == 403 => {
                Err(LoginError::Forbidden(format!("{} - Redirecting...", extract_message(&res.text().await.unwrap()))))
            }
            Ok(res) => {
                let status = res.status();
                match res.text().await {
                    Ok(body) if !body.is_empty() => Err(LoginError::Other(body)),
                    _ => Err(LoginError::Other(format!("HTTP Error: {}", status))),
                }
            }
            Err(_err) => Err(LoginError::Other(format!("There is a temporary issue with the background connection."))),
        }
    }, 
    |result| Message::Login(LoginMessage::LoginStatus(result)))
}


#[derive(Debug, Clone)]
pub enum LogoutMessage{
    LogoutPressed,
    LogoutStatus(Result<String, String>)
}

pub fn logout(token: Token) -> Task<Message>{
    Task::perform(
        async move{
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
                .post(format!("{}/{}", API_URL, "logout"))
                .header(COOKIE, cookie_header_value)
                .send()
                .await;
            
            match response{
                Ok(r) => {
                    if r.status() == 200{
                        Ok(r.text().await.unwrap())
                    }else if r.status() == 401{
                        Err(r.text().await.unwrap())
                    }else{
                        Err("HTTP error while logout".to_string())
                    }
                }
                Err(_err) => {
                    return Err(format!("There is a temporary issue with the background connection."));
                }
            }
        },
        |result| Message::Logout(LogoutMessage::LogoutStatus(result))
    )
}