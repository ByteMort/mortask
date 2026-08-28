use iced::{Alignment, Border, Color, Element, Length, Padding, Task, widget::{Container, Space, button, column, container, mouse_area, row, scrollable::{Direction, Scrollbar}, svg, text}};
use iced::widget::scrollable;
use reqwest::header::COOKIE;
use serde::Deserialize;

use crate::{API_URL, Message, State, needs::{help_methods::format_date_time, style::{button_transparant_style, priority_color}}, views::login::Token};

#[derive(Debug, Clone, Deserialize)]
pub struct MyTask{
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub priority_score: Option<i32>,
    pub created_at: String,
    pub status: String,
}

#[derive(Debug, Clone)]
pub enum TaskData {
    Loading,
    Text(String),
    Tasks(Vec<MyTask>),
}

#[derive(Debug, Clone)]
pub enum TaskError{
    Forbidden(String),
    NotFound(String),
    Other(String)
}

#[derive(Debug, Clone)]
pub enum TaskMessages{
    TaskStatus(Result<Vec<MyTask>, TaskError>),
    TaskPage,
    RefreshTasks,
    TaskDetails(i32),
    TaskDelete(i32),
    TaskDeleteStatus(Result<String, TaskError>),
    TaskDeleteCompleted
}

pub fn view(state: &State) -> Element<'_, Message>{
    let mut content = column![].spacing(10).align_x(Alignment::Center);

    if state.token.token == None && state.token.refresh_token == None{
        content = content.push(
            text("To view your tasks, you must first log in.")
        );
    }else {
        let refresh_button = button(
            svg(svg::Handle::from_path("assets/icons/refresh.svg"))
                .width(Length::Fixed(25.0))
                .height(Length::Fixed(25.0))
        )
        .padding(10)
        .style(|_theme, status| button_transparant_style(status))
        .on_press(Message::Tasks(TaskMessages::RefreshTasks));

        let add_button = button(
            svg(svg::Handle::from_path("assets/icons/task-plus.svg"))
                .width(Length::Fixed(25.0))
                .height(Length::Fixed(25.0))
        )
        .padding(10)
        .style(|_theme, status| button_transparant_style(status))
        .on_press(Message::GoToTaskAdd);

        let trash_completed_tasks = button(
            svg(svg::Handle::from_path("assets/icons/trash2.svg"))
                .width(Length::Fixed(25.0))
                .height(Length::Fixed(25.0))
        )
        .padding(10)
        .style(|_theme, status| button_transparant_style(status))
        .on_press(Message::Tasks(TaskMessages::TaskDeleteCompleted));

        content = content.push(
            container(row![refresh_button, add_button, trash_completed_tasks].spacing(10))
                .width(Length::Fill)      
                .height(Length::Shrink)   
                .align_x(Alignment::Center)
                .padding(Padding{
                    top: 0.0,
                    bottom: 10.0,
                    left: 0.0,
                    right: 0.0
                })
        );
        match &state.tasks {
            TaskData::Loading => {
                content = content.push(text("Loading..."));
            }
            TaskData::Text(msg) => {
                content = content.push(text(msg.clone()));
            }
            TaskData::Tasks(tasks) => {
                if let Some(err) = state.task_err.clone(){
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
                }else if let Some(scss) = state.task_suc.clone() {
                    content = content.push(
                        container(
                            text(scss)
                        )
                        .padding(8)
                        .style(|_theme| container::Style{
                            background: Some(iced::Background::Color(Color::from_rgb8(38, 108, 90))),
                            ..Default::default()
                        })
                    )
                    .push(Space::new().height(10));
                }

                let row1 = create_row(
                "Name", "Status",
                    "Start Date", "End Date",
                    "Priority".to_string(), "Created At".to_string(),
                    true, state.sort
                );
                content = content.push(row1);

                for task in tasks{
                    let row = create_row(
                        &task.name,
                        &task.status,
                        task.start_date.as_deref().unwrap_or("None"),
                        task.end_date.as_deref().unwrap_or("None"),
                        task.priority_score.map(|p| p.to_string()).unwrap_or("None".to_string()),
                        format_date_time(&task.created_at),
                        false,
                        None
                    );
                    let mouse_area = mouse_area(
                        row
                    )
                    .on_press(Message::Tasks(TaskMessages::TaskDetails(task.id)));

                    content = content.push(mouse_area);
                }
                return container(
                    scrollable(
                        container(content)
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .align_x(Alignment::Center)
                            .align_y(Alignment::Start)
                        )
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .direction(Direction::Vertical(
                            Scrollbar::new()
                                .width(0)
                                .margin(0)
                                .scroller_width(0)
                        ))
                    )
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .padding(Padding {
                        top: 40.0,
                        left: 0.0,
                        bottom: 40.0,
                        right: 0.0
                    })
                    .into();
            }
        }
    }

    container(
        content
    )
    .align_x(Alignment::Center)
    .align_y(Alignment::Center)
    .width(Length::Fill)
    .height(Length::Fill)
    .padding(Padding{
        top: 40.0,
        left: 0.0,
        bottom: 0.0,
        right: 0.0
    })
    .into()
}

fn sort_arrow_colors(
    current_sort: Option<(SortField, SortDirection)>,
    field: SortField,
) -> (Option<Color>, Option<Color>) {
    match current_sort {
        Some((f, SortDirection::Asc)) if f == field => (Some(Color::WHITE), None),
        Some((f, SortDirection::Desc)) if f == field => (None, Some(Color::WHITE)),
        _ => (None, None),
    }
}

pub fn create_row<'a>(
    name: &'a str,
    status: &'a str,
    start_date: &'a str,
    end_date: &'a str,
    priority_score: String,
    created_at: String,
    is_header: bool,
    current_sort: Option<(SortField, SortDirection)>,
) -> Container<'a, Message> {
    let alpha = if is_header {1.0} else{0.35};
    let text_size = if is_header {16} else {14};
    let container_height = if is_header {70.0} else {60.0};

    let priority_score_color = match priority_score.parse::<u8>().ok() {
        Some(val) => Some(priority_color(val)),
        None => None,
    };

    let (name_up, name_down) = sort_arrow_colors(current_sort, SortField::Name);
    let (start_up, start_down) = sort_arrow_colors(current_sort, SortField::StartDate);
    let (end_up, end_down) = sort_arrow_colors(current_sort, SortField::EndDate);
    let (priority_up, priority_down) = sort_arrow_colors(current_sort, SortField::PriorityScore);
    let (created_up, created_down) = sort_arrow_colors(current_sort, SortField::CreatedAt);

    let name_column: Element<'_, Message> = if is_header{
        mouse_area(
            column![
                svg(svg::Handle::from_path("assets/icons/arrow-up.svg"))
                    .width(Length::Fixed(12.0))
                    .height(Length::Fixed(12.0))
                    .style(move |_theme, _status| svg::Style { color: name_up }),
                text(name).size(text_size),
                svg(svg::Handle::from_path("assets/icons/arrow-down.svg"))
                    .width(Length::Fixed(12.0))
                    .height(Length::Fixed(12.0))
                    .style(move |_theme, _status| svg::Style { color: name_down }),
            ].align_x(Alignment::Center).spacing(2)
        )
        .on_press(Message::TaskFilter(TaskFilterMessage::SortBy(SortField::Name)))
        .into()
    }else{
        text(name)
            .size(text_size)
            .wrapping(text::Wrapping::None)
            .into()
    };

    let priority_column: Element<'_, Message> = if is_header{
        mouse_area(
        column![
            svg(svg::Handle::from_path("assets/icons/arrow-up.svg"))
                .width(Length::Fixed(12.0))
                .height(Length::Fixed(12.0))
                .style(move |_theme, _status| svg::Style { color: priority_up }),
            text(priority_score.clone()).size(text_size),
            svg(svg::Handle::from_path("assets/icons/arrow-down.svg"))
                .width(Length::Fixed(12.0))
                .height(Length::Fixed(12.0))
                .style(move |_theme, _status| svg::Style { color: priority_down }),
        ].align_x(Alignment::Center).spacing(2)
        )
        .on_press(Message::TaskFilter(TaskFilterMessage::SortBy(SortField::PriorityScore)))
        .into()
    }else{
        text(priority_score)
            .size(text_size)
            .style(move |_theme| text::Style { color: priority_score_color })
            .into()
    };

    let start_column: Element<'_, Message> = if is_header{
        mouse_area(
        column![
            svg(svg::Handle::from_path("assets/icons/arrow-up.svg"))
                .width(Length::Fixed(12.0))
                .height(Length::Fixed(12.0))
                .style(move |_theme, _status| svg::Style { color: start_up }),
            text(start_date).size(text_size),
            svg(svg::Handle::from_path("assets/icons/arrow-down.svg"))
                .width(Length::Fixed(12.0))
                .height(Length::Fixed(12.0))
                .style(move |_theme, _status| svg::Style { color: start_down }),
        ].align_x(Alignment::Center).spacing(2)
        )
        .on_press(Message::TaskFilter(TaskFilterMessage::SortBy(SortField::StartDate)))
        .into()
    }else{
        text(start_date)
        .size(text_size)
        .into()
    };

    let end_column: Element<'_, Message> = if is_header{
        mouse_area(
        column![
            svg(svg::Handle::from_path("assets/icons/arrow-up.svg"))
                .width(Length::Fixed(12.0))
                .height(Length::Fixed(12.0))
                .style(move |_theme, _status| svg::Style { color: end_up }),
            text(end_date).size(text_size),
            svg(svg::Handle::from_path("assets/icons/arrow-down.svg"))
                .width(Length::Fixed(12.0))
                .height(Length::Fixed(12.0))
                .style(move |_theme, _status| svg::Style { color: end_down }),
        ].align_x(Alignment::Center).spacing(2)
        )
        .on_press(Message::TaskFilter(TaskFilterMessage::SortBy(SortField::EndDate)))
        .into()
    }else{
        text(end_date)
        .size(text_size)
        .into()
    };

    let created_column: Element<'_, Message> = if is_header{
        mouse_area(
        column![
            svg(svg::Handle::from_path("assets/icons/arrow-up.svg"))
                .width(Length::Fixed(12.0))
                .height(Length::Fixed(12.0))
                .style(move |_theme, _status| svg::Style { color: created_up }),
            text(created_at).size(text_size),
            svg(svg::Handle::from_path("assets/icons/arrow-down.svg"))
                .width(Length::Fixed(12.0))
                .height(Length::Fixed(12.0))
                .style(move |_theme, _status| svg::Style { color: created_down }),
        ].align_x(Alignment::Center).spacing(2)
        )
        .on_press(Message::TaskFilter(TaskFilterMessage::SortBy(SortField::CreatedAt)))
        .into()
    }else{
        text(created_at)
        .size(text_size)
        .align_x(Alignment::Center)
        .into()
    };

    container(
        row![
            container(
                name_column
            )
            .width(Length::Fixed(120.0))
            .align_x(Alignment::Start) 
            .clip(true),
            text(status).size(text_size).width(Length::Fixed(80.0)).align_x(Alignment::Center),
            container(start_column).width(Length::Fixed(80.0)).align_x(Alignment::Center),
            container(end_column).width(Length::Fixed(75.0)).align_x(Alignment::Center),
            container(priority_column).width(Length::Fixed(70.0)).align_x(Alignment::Center),
            container(created_column).width(Length::Fixed(80.0)).align_x(Alignment::Center)
        ]
        .spacing(120)
        .align_y(Alignment::Center)
    )
    .padding(Padding {
        top: 10.0,
        bottom: 10.0,
        left: 85.0,
        right: 85.0,
    })
    .width(Length::Shrink)
    .height(Length::Fixed(container_height))
    .align_y(Alignment::Center)
    .style(move |_theme| container::Style {
        background: Some(iced::Background::Color(Color::from_rgb8(68, 71, 90))),
        border: Border {
            color: Color::from_rgba8(200, 170, 255, alpha),
            width: 1.0,
            radius: 8.0.into(),
        },
        ..Default::default()
    })
}

pub fn get_tasks(token: Token) -> Task<Message>{
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
        .get(format!("{}/{}", API_URL, "tasks"))
        .header(COOKIE, cookie_header_value)
        .send()
        .await;

        match response {
            Ok(res) if res.status() == 401 => {
                Err(TaskError::Forbidden(res.text().await.unwrap()))
            },
            Ok(res) if res.status() == 404 => {
                Err(TaskError::NotFound(res.text().await.unwrap()))
            },
            Ok(res) => {
                match res.json::<Vec<MyTask>>().await{
                    Ok(v) => {
                        Ok(v)
                    }
                    Err(err) => {
                        Err(TaskError::Other(format!("JSON couldnt be parsed: {}", err)))
                    }
                }
            },
            Err(_err) => {
                Err(TaskError::Other(format!("There is a temporary issue with the background connection.")))
            }
        }        
    }, |result| Message::Tasks(TaskMessages::TaskStatus(result)))
}
pub fn delete_completed_tasks(token: Token) -> Task<Message>{
    Task::perform(async move{
        let client = reqwest::Client::new();

        let mut cookies = Vec::new();
        if let Some(t) = token.token{
            cookies.push(format!("token={}", t));
        }
        if let Some(rt) = token.refresh_token{
            cookies.push(format!("refresh_token={}", rt));
        }

        let cookie_header = cookies.join("; ");

        let response = client
        .delete(format!("{}/{}/{}", API_URL, "tasks", "completed"))
        .header(COOKIE, cookie_header)
        .send()
        .await;

        match response {
            Ok(res) if res.status() == 401 => {
                Err(TaskError::Forbidden(res.text().await.unwrap()))
            }
            Ok(res) if res.status() == 404 => {
                Err(TaskError::NotFound(res.text().await.unwrap()))
            }
            Ok(res) if res.status() == 200 => {
                Ok(res.text().await.unwrap())
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


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SortDirection{
    Asc,
    Desc,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SortField{
    Name,
    PriorityScore,
    StartDate,
    EndDate,
    CreatedAt
}

#[derive(Debug, Clone)]
pub enum TaskFilterMessage{
    SortBy(SortField)
}

pub fn get_tasks_with_filter(token: Token, field: SortField, direction: SortDirection) -> Task<Message>{
    Task::perform(async move{
        let client = reqwest::Client::new();

        let mut cookies = Vec::new();
        if let Some(t) = token.token{
            cookies.push(format!("token={}", t));
        }
        if let Some(rt) = token.refresh_token{
            cookies.push(format!("refresh_token={}", rt));
        }
        let cookie_with_headers = cookies.join("; ");

        let response = client
            .get(format!("{}/{}/filter?sort_by={:?}&order={:?}", API_URL, "tasks", field, direction))
            .header(COOKIE, cookie_with_headers)
            .send()
            .await;
        
        match response{
            Ok(res) if res.status() == 401 => {
                Err(TaskError::Forbidden(res.text().await.unwrap()))
            },
            Ok(res) if res.status() == 404 => {
                Err(TaskError::NotFound(res.text().await.unwrap()))
            },
            Ok(res) => {
                match res.json::<Vec<MyTask>>().await{
                    Ok(v) => {
                        Ok(v)
                    }
                    Err(err) => {
                        Err(TaskError::Other(format!("JSON couldnt be parsed: {}", err)))
                    }
                }
            },
            Err(_err) => {
                Err(TaskError::Other(format!("There is a temporary issue with the background connection.")))
            }
        }
    },
    |result| Message::Tasks(TaskMessages::TaskStatus(result)))
}