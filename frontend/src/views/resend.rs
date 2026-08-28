use iced::{Alignment, Color, Element, Font, Length, Padding, Task, font, widget::{Space, button, column, container, mouse_area, text, text_input}};

use crate::{API_URL, Message, State, needs::style::button_transparant_style};

#[derive(Debug, Clone)]
pub enum VerifyMessage{
    EmailChanged(String),
    CodeChanged(String),
    VerifyPressed,
    VerifyStatus(Result<String, String>),
    SendCodePressed,
    SendCodeStatus(Result<String, String>),
}

pub fn view(state: &State) -> Element<'_, Message>{
    let title = text("VERIFY ACCOUNT")
    .size(24)
        .font(Font {
            weight: font::Weight::Bold,
            ..Font::DEFAULT
    })
    .color(Color::from_rgb8(200, 170, 255));

    let mut content = column![
        title,
        Space::new().height(10)
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
    }else if let Some(scss) = state.errors.success_message.clone(){
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

    let email_input = text_input("Enter your email address",
    &state.login_user_info.email)
    .on_input(|val| Message::Verify(VerifyMessage::EmailChanged(val)))
    .width(400)
    .padding(10);

    content = content.push(email_input);

    let link_to_page: Element<'_, Message> =  match state.login_user_info.code_sended {
        true => {
            mouse_area(
            text("Request a new verification code.")
            .style(|_theme| text::Style { color: Some(Color::from_rgb8(200, 170, 255)) })
            ).on_press(Message::GoToResend).into()
        }
        false => {
            mouse_area(
            text("I already have a verification code.")
            .style(|_theme| text::Style { color: Some(Color::from_rgb8(200, 170, 255)) })
            ).on_press(Message::GoToVerify).into()
        }
    };

    if state.login_user_info.code_sended == true{
        let code_input = text_input("Enter your code",
        &state.login_user_info.code)
        .on_input(|val| Message::Verify(VerifyMessage::CodeChanged(val)))
        .width(400)
        .padding(10);

        content = content.push(code_input);
        content = content.push(link_to_page);
        content = content.push(Space::new().height(10));

        let verify_code_button = button(
            container(
                    text("VERIFY")
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
        )
        .style(|_theme, status| button_transparant_style(status))
        .width(170)
        .height(40)
        .on_press(Message::Verify(VerifyMessage::VerifyPressed));

        content = content.push(verify_code_button);
    }else{
        content = content.push(link_to_page);
        content = content.push(Space::new().height(10));

        let send_code_button = button(
            container(
                    text("SEND CODE")
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
        )
        .style(|_theme, status| button_transparant_style(status))
        .width(170)
        .height(40)
        .on_press(Message::Verify(VerifyMessage::SendCodePressed));

        content = content.push(send_code_button);
    } 

    container(
        content
    )
    .width(Length::Shrink)
    .height(Length::Shrink)
    .align_x(Alignment::Center)
    .align_y(Alignment::Center)
    .padding(Padding{
        bottom: 110.0,
        ..Default::default()
    })
    .into()
}

pub fn verify(email:String, code: String) -> Task<Message>{
    Task::perform(async move{
        let client = reqwest::Client::new();

        let mut payload = serde_json::Map::new();

        if !email.trim().is_empty(){
            payload.insert("email".to_string(), serde_json::Value::String(email));
        }
        if !code.trim().is_empty(){
            payload.insert("code".to_string(), serde_json::Value::String(code));
        }

        let response = client
            .post(format!("{}/{}", API_URL, "verify"))
            .json(&payload)
            .send()
            .await;

        match response {
            Ok(res) if res.status() == 200 => {
                match res.text().await {
                    Ok(body) => {
                        Ok(body)
                    },
                    Err(err) => Err(format!("The code could not be verified: {}", err)),
                }
            }
            Ok(r) => {
                let status = r.status();
                match r.text().await {
                    Ok(body) if !body.is_empty() => Err(body),
                    _ => Err(format!("HTTP Error: {}", status)),
                }
            }
            Err(_err) => {
                return Err(format!("There is a temporary issue with the background connection."));
            }
        }
    }, |result| Message::Verify(VerifyMessage::VerifyStatus(result)))
}

pub fn send_code(email: String) -> Task<Message>{
    Task::perform(async move{
        let client = reqwest::Client::new();

        let mut payload = serde_json::Map::new();

        if !email.trim().is_empty(){
            payload.insert("email".to_string(), serde_json::Value::String(email));
        }

        let response = client
            .post(format!("{}/{}", API_URL, "resend"))
            .json(&payload)
            .send()
            .await;

        match response {
            Ok(res) if res.status() == 200 => {
                match res.text().await {
                    Ok(body) => {
                        Ok(body)
                    },
                    Err(err) => Err(format!("The code could not be sent: {}", err)),
                }
            }
            Ok(r) => {
                let status = r.status();
                match r.text().await {
                    Ok(body) if !body.is_empty() => Err(body),
                    _ => Err(format!("HTTP Error: {}", status)),
                }
            }
            Err(_err) => {
                return Err(format!("There is a temporary issue with the background connection."));
            }
        }

    }, |result| Message::Verify(VerifyMessage::SendCodeStatus(result))) 
}