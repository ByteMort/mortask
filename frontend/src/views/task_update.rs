use iced::{Alignment, Border, Color, Element, Font, Length, Padding, Task, font, widget::{ Space, button, column, container, pick_list, row, slider, stack, svg, text, text_editor, text_input}};
use iced_aw::{DatePicker, date_picker::{self, Date}};
use reqwest::header::COOKIE;
use crate::{API_URL, Message, State, needs::style::{button_transparant_style, priority_color}, views::{login::Token, task_update::TaskUpdateMessage::{TaskDescriptionEdited, TaskNameChanged}, tasks::MyTask, }};

#[derive(Debug, Default)]
pub struct TaskUpdate{
    pub name: String,
    pub description: text_editor::Content,
    pub show_start_date_picker: bool,
    pub start_date: Option<Date>,
    pub show_end_date_picker: bool,
    pub end_date: Option<Date>,
    pub priority: u8,
    pub status: String,
}

#[derive(Debug, Clone)]
pub enum TaskError{
    Forbidden(String),
    Other(String)
}

#[derive(Debug, Clone)]
pub enum TaskUpdateMessage{
    TaskNameChanged(String),
    TaskDescriptionEdited(text_editor::Action),
    OpenStartDatePicker,
    CloseStartDatePicker,
    SubmitStartDate(Date),
    OpenEndDatePicker,
    CloseEndDatePicker,
    SubmitEndDate(Date),
    ResetStartDate,
    ResetEndDate,
    PriorityChanged(u8),
    TaskStatusChanged(String),
    TaskUpdateClicked,
    TaskUpdateStatus(Result<MyTask, TaskError>)
}

pub fn view(state: &State) -> Element<'_, Message> {
    let mut content = column![].align_x(Alignment::Center);
    
    let exit_button = button(
        svg(svg::Handle::from_path("assets/icons/x.svg")).width(23).height(23)
    ).style(|_theme, status| button_transparant_style(status)).on_press(Message::GoToTaskDetails);
    
    let exit_row = container(exit_button)
        .width(500.0)
        .height(Length::Shrink)
        .align_x(Alignment::End);

    let icon = svg(svg::Handle::from_path("assets/icons/pencil.svg"))
        .width(50).height(50);
    content = content.push(icon).push(Space::new().height(6));

    let title = text("EDIT TASK")
        .size(24)
        .font(Font {
            weight: font::Weight::Bold,
            ..Font::DEFAULT
        })
        .color(Color::from_rgb8(200, 170, 255))
        .width(Length::Fixed(500.0))
        .align_x(Alignment::Center);
    content = content.push(title).push(Space::new().height(15));
    
    if let Some(err) = state.errors.error_message.clone() {
        content = content.push(
            container(text(err))
                .padding(8)
                .style(|_theme| container::Style {
                    background: Some(iced::Background::Color(Color::from_rgb8(140, 51, 51))),
                    ..Default::default()
                })
                .width(Length::Fixed(400.0))
                .align_x(Alignment::Center)
        ).push(Space::new().height(10));
    } else if let Some(scss) = state.errors.success_message.clone() {
        content = content.push(
            container(text(format!("{} - Redirecting...", scss)))
                .padding(8)
                .style(|_theme| container::Style {
                    background: Some(iced::Background::Color(Color::from_rgb8(38, 108, 90))),
                    ..Default::default()
                })
                .width(Length::Fixed(400.0))
                .align_x(Alignment::Center)
        ).push(Space::new().height(10));
    }
    
    let task_name_row = column![
        row![
            svg(svg::Handle::from_path("assets/icons/type.svg")).width(20).height(20),
            text("Name").size(13)
        ].align_y(Alignment::Center).spacing(5),
        text_input("Task Name...", &state.update_task.name)
            .on_input(|val| Message::TaskUpdate(TaskNameChanged(val)))
            .width(400)
            .padding(10)
    ].spacing(5);
    content = content.push(task_name_row).push(Space::new().height(15));

    let task_description_row = column![
        row![
            svg(svg::Handle::from_path("assets/icons/description.svg")).width(20).height(20),
            text("Description").size(13)
        ].align_y(Alignment::Center).spacing(5),
        text_editor(&state.update_task.description)
            .placeholder("Task Description...")
            .on_action(|action| Message::TaskUpdate(TaskDescriptionEdited(action)))
            .width(400)
            .height(Length::Fixed(100.0))
            .wrapping(text::Wrapping::Glyph)
            .padding(10)
    ].spacing(5);
    content = content.push(task_description_row).push(Space::new().height(15));

    let statuses = vec![
        "PENDING".to_string(),
        "IN PROGRESS".to_string(),
        "COMPLETED".to_string()
    ];
    let task_status_row = column![
        row![
            svg(svg::Handle::from_path("assets/icons/activity.svg")).width(20).height(20),
            text("Status").size(13)
        ].align_y(Alignment::Center).spacing(5),
        pick_list(
            statuses, 
            Some(state.update_task.status.clone()), 
            |val| Message::TaskUpdate(TaskUpdateMessage::TaskStatusChanged(val))
        ).width(400)
    ].spacing(5);
    content = content.push(task_status_row).push(Space::new().height(15));

    let start_date_label = match state.update_task.start_date{
        Some(v) => v.to_string(),
        None => "Select date...".to_string()
    };
    let end_date_label = match state.update_task.end_date{
        Some(v) => v.to_string(),
        None => "Select date...".to_string()
    };

    let start_date_picker = DatePicker::new(
        state.update_task.show_start_date_picker, 
        state.update_task.start_date.unwrap_or(date_picker::Date::today()), 
        button(text(start_date_label)
            .align_x(Alignment::Center).align_y(Alignment::Center))
            .on_press(Message::TaskUpdate(TaskUpdateMessage::OpenStartDatePicker))
            .width(150).padding(6), 
        Message::TaskUpdate(TaskUpdateMessage::CloseStartDatePicker), 
        |date| Message::TaskUpdate(TaskUpdateMessage::SubmitStartDate(date))
    ).font_size(16);
    let end_date_picker = DatePicker::new(
            state.update_task.show_end_date_picker, 
            state.update_task.end_date.unwrap_or(date_picker::Date::today()), 
            button(text(end_date_label)
                .align_x(Alignment::Center).align_y(Alignment::Center))
                .on_press(Message::TaskUpdate(TaskUpdateMessage::OpenEndDatePicker))
                .width(150).padding(6), 
            Message::TaskUpdate(TaskUpdateMessage::CloseEndDatePicker), 
            |date| Message::TaskUpdate(TaskUpdateMessage::SubmitEndDate(date))
        ).font_size(16);
    let start_date_field = if state.update_task.start_date.is_some(){
        stack![
            start_date_picker,
            container(
                button(svg(svg::Handle::from_path("assets/icons/x.svg")).width(17).height(17))
                    .on_press(Message::TaskUpdate(TaskUpdateMessage::ResetStartDate))
                    .padding(2)
                    .style(|_theme, _status| button::Style {
                        background: Some(iced::Background::Color(Color::from_rgba8(255, 95, 95, 0.12))),
                        text_color: Color::from_rgb8(255, 128, 128),
                        border: Border {
                            color: Color::TRANSPARENT,
                            width: 0.0,
                            radius: 20.0.into(),   
                        },
                        ..Default::default()
                    })
            )
                .width(Length::Fill)
                .height(Length::Fill)
                .align_x(Alignment::End)
                .align_y(Alignment::Center)
                .padding(Padding { right: 3.0, ..Default::default() })
        ]
    }else{
        stack![start_date_picker]
    };
    let end_date_field = if state.update_task.end_date.is_some(){
        stack![
            end_date_picker,
            container(
                button(svg(svg::Handle::from_path("assets/icons/x.svg")).width(17).height(17))
                    .on_press(Message::TaskUpdate(TaskUpdateMessage::ResetEndDate))
                    .padding(2)
                    .style(|_theme, _status| button::Style {
                        background: Some(iced::Background::Color(Color::from_rgba8(255, 95, 95, 0.12))),
                        text_color: Color::from_rgb8(255, 128, 128),
                        border: Border {
                            color: Color::TRANSPARENT,
                            width: 0.0,
                            radius: 20.0.into(),   
                        },
                        ..Default::default()
                    })
            )
                .width(Length::Fill)
                .height(Length::Fill)
                .align_x(Alignment::End)
                .align_y(Alignment::Center)
                .padding(Padding { right: 3.0, ..Default::default() })
        ]
    }else{
        stack![end_date_picker]
    };
    let date_pickers = row![
        column![row![
            svg(svg::Handle::from_path("assets/icons/calendar-check.svg")).width(20).height(20),
            text("Start Date").size(13)
        ].align_y(Alignment::Center).spacing(5),
            start_date_field
        ].spacing(5),

        Space::new().width(Length::Fill),

        column![row![
            svg(svg::Handle::from_path("assets/icons/calendar-check.svg")).width(20).height(20),
            text("End Date").size(13)
        ].align_y(Alignment::Center).spacing(5),
            end_date_field
        ].spacing(5)
    ].width(Length::Fixed(400.0));
    content = content
        .push(date_pickers)
        .push(Space::new().height(15));

    let color = priority_color(state.update_task.priority);

    let priority_row = column![
        row![
            row![
                svg(svg::Handle::from_path("assets/icons/flag.svg")).width(20).height(20),
                text("Priority").size(13)
            ].align_y(Alignment::Center).spacing(5),
            Space::new().width(Length::Fill),
            text(format!("{}/10", state.update_task.priority))
            .style(move |_theme| text::Style { 
                color: Some(color)
            })
        ].width(400.0),
        slider(0..=10, state.update_task.priority,
        |v| Message::TaskUpdate(TaskUpdateMessage::PriorityChanged(v)))
        .step(1)
        .style(move |_theme, _status| {
            slider::Style {
                rail: slider::Rail {
                    backgrounds: (
                        iced::Background::Color(color),
                        iced::Background::Color(Color::from_rgb8(60, 62, 78)),
                    ),
                    width: 6.0,
                    border: Border { radius: 3.0.into(), width: 0.0, color: Color::TRANSPARENT },
                },
                handle: slider::Handle {
                    shape: slider::HandleShape::Circle { radius: 8.0 },
                    background: iced::Background::Color(color),
                    border_width: 2.0,
                    border_color: Color::from_rgb8(45, 47, 63),
                },
            }
        })
        .width(Length::Fixed(400.0))
    ].spacing(5);
    content = content
        .push(priority_row)
        .push(Space::new().height(25));
    
    let task_update_button = button(
        text("UPDATE Task").align_x(Alignment::Center).align_y(Alignment::Center)
    )
    .width(400)
    .style(|_theme, _status| button::Style {
        background: Some(iced::Background::Color(Color::from_rgb8(74, 64, 105))),
        text_color: Color::from_rgb8(230, 225, 245),
        border: Border {
            color: Color::from_rgb8(140, 120, 191),
            width: 1.0,
            radius: 8.0.into(),
        },
        ..Default::default()
    })
    .padding(10)
    .on_press(Message::TaskUpdate(TaskUpdateMessage::TaskUpdateClicked));
    content = content.push(task_update_button);

    container(
        column![
            exit_row,
            content
        ].spacing(13).align_x(Alignment::Center)
    )
    .style(|_theme| container::Style {
         border: Border { 
            color: Color::from_rgb8(200, 170, 255), 
            width: 2.0, 
            radius: 30.0.into()
        },
        ..Default::default() 
    })
    .width(Length::Shrink)
    .height(Length::Shrink)
    .padding(25)
    .into()
}

pub fn update_task(token: Token, id: Option<i32>, task_update: &TaskUpdate) -> Task<Message> {
    let name = task_update.name.clone();
    let desc = task_update.description.text();
    let status = task_update.status.clone();
    let start_date = task_update.start_date.map(|d| d.to_string()).unwrap_or_default();
    let end_date = task_update.end_date.map(|d| d.to_string()).unwrap_or_default();
    let priority = task_update.priority;

    Task::perform(async move{
        if let Some(id) = id{
            let client = reqwest::Client::new();
            let mut cookies= Vec::new();
            if let Some(t) = token.token{
                cookies.push(format!("token={}", t));
            }
            if let Some(rt) = token.refresh_token{
                cookies.push(format!("refresh_token={}", rt));
            }
            let cookie_with_headers = cookies.join("; ");

            let mut payload = serde_json::Map::new();
            payload.insert("name".to_string(), serde_json::Value::String(name));
            if !desc.trim().is_empty(){
                payload.insert("description".to_string(), serde_json::Value::String(desc));
            }else{
                payload.insert("description".to_string(), serde_json::Value::Null);
            }
            payload.insert("status".to_string(), serde_json::Value::String(status));
            if !start_date.trim().is_empty(){
                payload.insert("start_date".to_string(), serde_json::Value::String(start_date));
            }else{
                payload.insert("start_date".to_string(), serde_json::Value::Null);
            }
            if !end_date.trim().is_empty(){
                payload.insert("end_date".to_string(), serde_json::Value::String(end_date));
            }else{
                payload.insert("end_date".to_string(), serde_json::Value::Null);
            }
            if priority == 0 {
                payload.insert("priority_score".to_string(), serde_json::Value::Null);
            }else{
                payload.insert("priority_score".to_string(), serde_json::Value::Number(priority.into()));
            }

            let response = client
            .put(format!("{}/{}/{}", API_URL, "tasks", id))
            .header(COOKIE, cookie_with_headers)
            .json(&payload)
            .send()
            .await;

            match response{
                Ok(res) if res.status() == 401 => {
                    Err(TaskError::Forbidden(res.text().await.unwrap()))
                },
                Ok(res) if res.status() == 202 => {
                    Ok(res.json::<MyTask>().await.unwrap())
                }
                Ok(res) => {
                    let status = res.status();
                    match res.text().await{
                        Ok(body) if !body.is_empty() => Err(TaskError::Other(format!("{}", body))),
                        _ => Err(TaskError::Other(format!("HTTP Error: {}", status)))
                    }
                }
                Err(_err) => {
                    Err(TaskError::Other("There is a temporary issue with the background connection.".to_string()))
                }
            }
        }else{
            Err(TaskError::Other("There is an issue with the task ID".to_string()))
        }
    }, 
    |result| Message::TaskUpdate(TaskUpdateMessage::TaskUpdateStatus(result)))
}