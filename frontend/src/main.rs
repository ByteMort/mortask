mod views;
mod needs;

use iced::window::{self};
use iced::{Alignment, Element, Length, Size, Task}; 
use iced::widget::{Space, button, column, container, mouse_area, row, stack};
use iced_anim::{Animated, Motion, Event};

use crate::needs::auth_storage::load_session;
use crate::needs::help_methods::{load_icon, theme};
use crate::views::login::{LoginMessage, LoginUserInfo, LogoutMessage, MyErrors, Token};
use crate::views::profile::{self, ProfileMessage};
use crate::views::register::{RegisterMessage, RegisterUserInfo};
use crate::views::resend::{VerifyMessage};
use crate::needs::style::{ animated_button, button_style };
use crate::views::task_add::{TaskAdd, TaskAddMessage};
use crate::views::task_update::{TaskUpdate, TaskUpdateMessage};
use crate::views::tasks::{SortDirection, SortField, TaskData, TaskFilterMessage, TaskMessages, get_tasks};
use crate::needs::messages;

pub const API_URL:&str = "http://127.0.0.1:3000/api";

pub enum Page{
    Tasks,
    TaskDetail,
    TaskAdd,
    TaskUpdate,
    About,
    Login,
    Profile,
    Resend,
    Register
}

pub struct State {
    pub page: Page,
    pub sidebar_open: bool,
    pub sidebar_button_size: Animated<f32>,
    pub login_user_info: LoginUserInfo,
    pub register_user_info: RegisterUserInfo,
    pub token: Token,
    pub profile: Option<profile::Profile>,
    pub tasks: TaskData,
    pub selected_task: Option<i32>,
    pub errors: MyErrors,
    pub task_err: Option<String>,
    pub task_suc: Option<String>,
    pub add_task: TaskAdd,
    pub update_task: TaskUpdate,
    pub sort: Option<(SortField, SortDirection)>
}

#[derive(Debug, Clone)]
pub enum Message{
    // Buttons
    GoToAbout,
    GoToTasks,
    GoToTaskDetails,
    GoToLogin,
    GoToProfile,
    GoToResend,
    GoToVerify,
    GoToRegister,
    GoToTaskAdd,
    GoToTaskUpdate,

    // Window
    Exit,
    Minimize,
    // Maximize,
    WindowDrag,

    OpenDeveloper,
    ToggleSidebar,

    HoverSidebarButton(bool),
    SidebarButtonAnimation(Event<f32>),

    Login(LoginMessage),
    Profile(ProfileMessage),
    Logout(LogoutMessage),
    Verify(VerifyMessage),
    Register(RegisterMessage),
    Tasks(TaskMessages),
    TaskAdd(TaskAddMessage),
    TaskUpdate(TaskUpdateMessage),
    TaskFilter(TaskFilterMessage)
}

fn main() -> iced::Result{
    iced::application(boot, update, view)
    .window(window::Settings{
        size: Size::new(1500.0, 900.0),
        min_size: Some(Size::new(1500.0, 900.0)),
        resizable: false,
        decorations: false,
        icon: Some(load_icon("assets/icons/mortask-icon.png")),
        ..Default::default()
    })
    .title("MorTask")
    .font(iced_aw::ICED_AW_FONT_BYTES)
    .theme(theme)
    .run()
}

fn boot() -> (State, Task<Message>) {
    let (token, profile) = match load_session() {
        Some((token, profile)) => (token, Some(profile)),
        None => (Token{ token:None, refresh_token: None}, None)
    };
    
    let initial_task = if token.token.is_some() {
        get_tasks(token.clone())
    } else {
        Task::none()
    };

    (
        State{
            page: Page::Tasks,
            sidebar_open: false,
            sidebar_button_size: Animated::spring(20.0, Motion::SMOOTH),
            login_user_info: LoginUserInfo::default(),
            register_user_info: RegisterUserInfo::default(),
            token,
            profile,
            errors: MyErrors::default(),
            tasks: TaskData::Loading,
            selected_task: None,
            task_err: None,
            task_suc: None,
            add_task: TaskAdd{ priority: 0, ..Default::default()},
            update_task: TaskUpdate { priority: 0, ..Default::default() },
            sort: None
        },
        initial_task
    )
}

fn update(state: &mut State, message: Message) -> Task<Message> {
    messages::give_messages(state, message)
}

fn view(state: &State) -> Element<'_, Message>{
    let menu_bar = mouse_area(row![
        button("Tasks").on_press(Message::GoToTasks).style(button_style),
        button("About").on_press(Message::GoToAbout).style(button_style),
        button("Exit").on_press(Message::Exit).style(button_style),

        Space::new().width(Length::Fill),

        button("—").on_press(Message::Minimize).style(button_style),
        // button("▢").on_press(Message::Maximize).style(button_style),
        button("✕").on_press(Message::Exit).style(button_style),
    ]
    .padding(5)
    .align_y(Alignment::Center)
    ).on_press(Message::WindowDrag);
    
    let page = match state.page {
        Page::Tasks => views::tasks::view(state),
        Page::TaskDetail => views::task_detail::view(state),
        Page::TaskAdd => views::task_add::view(state),
        Page::TaskUpdate => views::task_update::view(state),
        Page::About => views::about::view(),
        Page::Login => views::login::view(state),
        Page::Profile => views::profile::view(state),
        Page::Resend => views::resend::view(state),
        Page::Register => views::register::view(state)
    };

    let sidebar_icon_path = if state.sidebar_open {
        "assets/icons/sidebar_arrow_right.svg"
    } else {
        "assets/icons/sidebar_arrow_left.svg"
    };

    let side_bar = container(
    animated_button(state, None, Some(sidebar_icon_path), Message::ToggleSidebar)
    )
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Alignment::End)  
        .align_y(Alignment::Center)  
        .padding(7);


    let sidebar_content =  needs::sidebar::view(state);

    column![
        menu_bar,
        row![
            stack![
                container(page)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x(Length::Fill)
                    .center_y(Length::Fill),
                side_bar,
            ]
            .width(Length::Fill)
            .height(Length::Fill),

            sidebar_content,
        ]
        .width(Length::Fill)
        .height(Length::Fill)
    ].into()
}

