use iced::{Alignment, Border, Color, Element, Length, Padding, Task, widget::{Space, button, column, container, row, rule, svg, text}};
use reqwest::header::COOKIE;

use crate::{API_URL, Message, State, needs::{help_methods::format_date_time, style::{button_transparant_style, priority_color}}, views::{login::Token, tasks::{MyTask, TaskData, TaskError, TaskMessages}}};


pub fn view(state: &State) -> Element<'static, Message> {
    let mut content = column![].spacing(10);

    let exit_button = button(
            svg(svg::Handle::from_path("assets/icons/x.svg")).width(25).height(25)
    ).style(|_theme, status| button_transparant_style(status)).on_press(Message::GoToTasks);
    let exit_row = container(exit_button)
        .width(500.0)
        .height(Length::Shrink)
        .align_x(Alignment::End);

    if let Some(id) = state.selected_task.as_ref(){
        match state.tasks.clone() {
            TaskData::Tasks(v) => {
                let task: Option<&MyTask> = v.iter().find(|x| x.id == *id);
                if let Some(task) = task {

                    let status_lower = task.status.to_lowercase();
                    let status_trimmed = status_lower.trim(); 

                    let (background_color, border_color) = if status_trimmed == "completed" {
                        (
                            Color::from_rgb8(20, 60, 35),  
                            Color::from_rgba8(80, 250, 123, 0.4),
                        )
                    } else if status_trimmed == "pending" {
                        (
                            Color::from_rgb8(15, 45, 70),   
                            Color::from_rgba8(98, 114, 164, 0.4),
                        )
                    } else {
                        (
                            Color::from_rgb8(70, 40, 10),                            
                            Color::from_rgba8(139, 233, 253, 0.4),
                        )
                    };

                    let top_row = row![
                        text(task.name.clone()).size(18).width(380).wrapping(text::Wrapping::Glyph),
                        Space::new().width(Length::Fill),
                        container(text(task.status.clone()))
                        .style(move |_theme| container::Style { 
                            background: Some(iced::Background::Color(background_color)), 
                            text_color: Some(Color::from_rgb8(230, 225, 245)),
                            border: Border {
                                color: border_color,
                                width: 1.0,
                                radius: 8.0.into(),
                            },
                            ..Default::default()
                        })
                        .padding(7.0)
                        .width(Length::Shrink)
                        .height(Length::Shrink)
                    ].width(500);

                    let description_str = task.description.as_deref().unwrap_or("No Description");
                    let description = text(description_str.to_string())
                    .size(15)
                    .width(500);

                    let start_date_str = task.start_date.as_deref().unwrap_or("No Start Date");
                    let row_start = row![
                        row![
                            svg(svg::Handle::from_path("assets/icons/calendar-check.svg")).width(30).height(30),
                            text("Start Date").size(15)
                        ].spacing(10).align_y(Alignment::Center),
                        Space::new().width(Length::Fill),
                        text(start_date_str.to_string())
                    ].width(500);

                    let end_start_str = task.end_date.as_deref().unwrap_or("No End Date");
                    let row_end = row![
                        row![
                            svg(svg::Handle::from_path("assets/icons/calendar-check.svg")).width(30).height(30),
                            text("End Date").size(15)
                        ].spacing(10).align_y(Alignment::Center),
                        Space::new().width(Length::Fill),
                        text(end_start_str.to_string())
                    ].width(500);
                    
                    let priority_bck = match task.priority_score {
                        Some(val) => priority_color(val as u8), 
                        None => Color::from_rgb8(68, 71, 90),
                    };

                    let priority_string = task.priority_score.map(|p| p.to_string()).unwrap_or("No Priority".to_string());
                    
                    let row_priority = row![
                        row![
                            svg(svg::Handle::from_path("assets/icons/flag.svg")).width(30).height(30),
                            text("Priority").size(15)
                        ].spacing(10).align_y(Alignment::Center),
                        Space::new().width(Length::Fill),
                        container(text(priority_string.to_string()))
                        .style(move |_theme| container::Style {
                            background: Some(iced::Background::Color(priority_bck)),
                            border: Border {
                                color: Color::from_rgba8(200, 170, 255, 0.35),
                                width: 1.0,
                                radius: 8.0.into(),
                            },
                            ..Default::default()
                        }).padding(5.0)
                    ].width(500);

                    let row_created_at = row![
                        row![
                            svg(svg::Handle::from_path("assets/icons/clock.svg")).width(30).height(30),
                            text("Created").size(15)
                        ].spacing(10).align_y(Alignment::Center),
                        Space::new().width(Length::Fill),
                        text(format_date_time(task.created_at.as_ref()))
                    ].width(500);

                    let row_buttons = row![
                        button(
                            row![
                                svg(svg::Handle::from_path("assets/icons/pencil.svg")).width(25).height(25),
                                text("Edit")
                            ].spacing(5).align_y(Alignment::Center)
                        ).style(|_theme, _status| button::Style { 
                            background: Some(iced::Background::Color(Color::from_rgb8(74, 64, 105))),
                            text_color: Color::from_rgb8(230, 225, 245),
                            border: Border {
                                color: Color::from_rgb8(140, 120, 191),
                                width: 1.0,
                                radius: 8.0.into(),
                            },
                            ..Default::default()
                        }).on_press(Message::GoToTaskUpdate),
                        button(
                            row![
                                svg(svg::Handle::from_path("assets/icons/trash.svg")).width(25).height(25),
                                text("Delete")
                            ].spacing(5).align_y(Alignment::Center)
                        ).style(|_theme, _status| button::Style { 
                            background: Some(iced::Background::Color(Color::from_rgb8(89, 41, 46))),
                            text_color: Color::from_rgb8(245, 225, 226),
                            border: Border {
                                color: Color::from_rgb8(191, 89, 96),
                                width: 1.0,
                                radius: 8.0.into(),
                            },
                            ..Default::default()
                        }).on_press(Message::Tasks(TaskMessages::TaskDelete(task.id)))
                    ].spacing(20);

                    let row_buttons_centered = container(row_buttons)
                    .width(Length::Fixed(500.0))
                    .align_x(Alignment::Center);

                    content = content
                    .push(top_row)
                    .push(Space::new().height(30))
                    .push(description)
                    .push(Space::new().height(20))
                    .push(rule::horizontal(1))
                    .push(Space::new().height(20))
                    .push(row_start)
                    .push(Space::new().height(20))
                    .push(row_end)
                    .push(Space::new().height(20))
                    .push(row_priority)
                    .push(Space::new().height(20))
                    .push(row_created_at)
                    .push(Space::new().height(30))
                    .push(row_buttons_centered);

                    
                }else{
                    content = content.push(text("Oops no task with this id found."));
                }
            }
            _ => {
                println!("BAD");
            }
        }

    }else{
        let t = text("Oops");
        content = content.push(t);
    }

    container(
        column![
            exit_row,
            content
        ].spacing(13).align_x(Alignment::Center)
    )
    .width(Length::Shrink)
    .height(Length::Shrink)
    .align_x(Alignment::Center)
    .align_y(Alignment::Center)
    .style(|_theme| container::Style{
        border: Border { 
            color: Color::from_rgb8(200, 170, 255), 
            width: 2.0, 
            radius: 30.0.into()
        },
        ..Default::default()
    })
    .padding(Padding{
        top: 25.0,
        bottom: 25.0,
        left: 25.0,
        right: 25.0
    })
    .into()
}

pub fn delete(token: Token, id: i32) -> Task<Message> {
    Task::perform(async move{
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
        .delete(format!("{}/{}/{}", API_URL, "tasks", id))
        .header(COOKIE, cookie_header_value)
        .send()
        .await;

        match response {
            Ok(res) if res.status() == 200 => {
                Ok(res.text().await.unwrap())
            }
            Ok(res) if res.status() == 401 => {
                Err(TaskError::Forbidden(res.text().await.unwrap()))
            }
            Ok(res) if res.status() == 404 => {
                Err(TaskError::NotFound(res.text().await.unwrap()))
            }
            Ok(res) => {
                let status = res.status();
                match res.text().await {
                    Ok(body) if !body.is_empty() => Err(TaskError::Other(body)),
                    _ => Err(TaskError::Other(format!("HTTP Error: {}", status)))
                }
            }
            Err(_err) => {
                Err(TaskError::Other(format!("There is a temporary issue with the background connection.")))
            }
        }

    }, |result| Message::Tasks(TaskMessages::TaskDeleteStatus(result)))
}