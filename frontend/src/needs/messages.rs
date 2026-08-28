use std::time::Duration;

use chrono::NaiveDate;
use iced::{Task, widget::text_editor, window};
use iced_anim::Event;
use iced_aw::date_picker::Date;

use crate::{Message, Page, State, needs::{auth_storage, help_methods::extract_message}, views::{login::{self, LoginError, LoginMessage, LogoutMessage, Token, logout}, profile::{self, ProfileMessage}, register::{RegisterMessage, register}, resend::{VerifyMessage, send_code, verify}, task_add::{self, TaskAdd, TaskAddMessage, create_task}, task_detail, task_update::{self, TaskUpdate, TaskUpdateMessage, update_task}, tasks::{SortDirection, TaskData, TaskError, TaskFilterMessage, TaskMessages, delete_completed_tasks, get_tasks, get_tasks_with_filter}}};

pub fn give_messages(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::GoToTasks => {
            state.errors.success_message = None;
            state.errors.error_message = None;
            state.page = Page::Tasks;
            state.selected_task = None;
            state.add_task = TaskAdd{ priority: 0, ..Default::default() };
            state.update_task = TaskUpdate{priority: 0, ..Default::default()};
            state.sort = None;

            get_tasks(state.token.clone())
        },
        Message::GoToTaskDetails => {
            if state.selected_task.is_none(){
                state.page = Page::Tasks;
            }else{
                state.page = Page::TaskDetail;
            }
            Task::none()
        },
        Message::GoToTaskUpdate => {
            if state.selected_task.is_none(){
                state.page = Page::Tasks;
            }else{
                state.page = Page::TaskUpdate;
                state.update_task = TaskUpdate{priority: 0, ..Default::default()};
                if let TaskData::Tasks(ref v) = state.tasks{
                    if let Some(task) = v.iter().find(|x| x.id == state.selected_task.unwrap()){
                        state.update_task.name = task.name.clone();
                        state.update_task.status = task.status.clone();
                        if task.description.is_some(){
                            state.update_task.description = text_editor::Content::with_text(task.description.as_ref().unwrap());
                        }
                        state.update_task.start_date = task.start_date.as_deref()
                            .and_then(|d| NaiveDate::parse_from_str(d, "%Y-%m-%d").ok())
                            .map(Date::from);

                        state.update_task.end_date = task.end_date.as_deref()
                            .and_then(|d| NaiveDate::parse_from_str(d, "%Y-%m-%d").ok())
                            .map(Date::from);

                        if let Some(score) = task.priority_score{
                            if score != 0 {
                                state.update_task.priority = score as u8;
                            }
                        }
                    }
                }
            }
            state.errors.error_message = None;
            state.errors.success_message = None;
            
            Task::none()
        },
        Message::GoToTaskAdd => {
            state.page = Page::TaskAdd;
            state.errors.success_message = None;
            state.errors.error_message = None;
            state.add_task = TaskAdd{ priority: 0, ..Default::default() };
            Task::none()
        },
        Message::GoToAbout => {
            state.errors.success_message = None;
            state.errors.error_message = None;
            state.page = Page::About;
            Task::none()
        },
        Message::GoToLogin => {
            state.errors.success_message = None;
            // state.errors.error_message = None;
            state.profile = None;
            state.page = Page::Login;
            Task::none()
        },
        Message::GoToRegister => {
            state.errors.success_message = None;
            state.errors.error_message = None;
            state.profile = None;
            state.page = Page::Register;
            Task::none()
        },
        Message::GoToProfile => {
            state.errors.success_message = None;
            state.errors.error_message = None;
            state.page = Page::Profile;
            profile::get_my_profile(state.token.clone())
        }
        Message::GoToResend => {
            state.page = Page::Resend;
            state.errors.success_message = None;
            state.errors.error_message = None;
            state.login_user_info.code_sended = false;
            Task::none()
        },
        Message::GoToVerify => {
            state.login_user_info.code_sended = true;
            state.login_user_info.code = "".to_string();
            Task::none()
        },
        Message::Exit => {
            match (&state.token.token, &state.profile) {
                (Some(_), Some(profile)) => {
                    let _ = auth_storage::save_session(&state.token, profile);
                }
                _ => {
                    auth_storage::clear_session();
                }
            }
            iced::exit()
        },
        Message::Minimize => {
            window::latest().and_then(|id| window::minimize(id, true))
        },
        /*Message::Maximize => {
            window::latest().and_then(window::toggle_maximize)
        },*/
        Message::WindowDrag => {
            window::latest().and_then(|id| window::drag(id))
        },
        Message::OpenDeveloper => {
            if let Err(error) = webbrowser::open("https://github.com/ByteMort"){
                eprintln!("Could not open browser: {error}");
            }
            Task::none()
        },
        Message::ToggleSidebar => {
            state.sidebar_open = !state.sidebar_open;
            Task::none()
        },
        Message::HoverSidebarButton(is_hovered) => {
            let target = if is_hovered {26.0} else {20.0};
            state.sidebar_button_size.update(Event::Target(target));
            Task::none()
        },
        Message::SidebarButtonAnimation(event) => {
            state.sidebar_button_size.update(event);
            Task::none()
        },
        Message::Login(login_msg) => match login_msg {
            LoginMessage::EmailChanged(email) => {
                state.login_user_info.email = email;
                Task::none()
            },
            LoginMessage::PasswordChanged(password) => {
                state.login_user_info.password = password;
                Task::none()
            },
            LoginMessage::ToggleShowPassword => {
                state.login_user_info.show_password = !state.login_user_info.show_password;
                Task::none()
            },
            LoginMessage::LoginPressed => {
                let email = state.login_user_info.email.clone();
                let password = state.login_user_info.password.clone();

                login::login(email, password)
            },
            LoginMessage::LoginStatus(status) => {
                match status {
                    Ok((message, request_token, refresh_token)) => {
                        state.errors.success_message = Some(extract_message(&message));
                        state.errors.error_message = None;

                        state.token.token = request_token;
                        state.token.refresh_token = refresh_token;
                        
                        profile::get_my_profile(state.token.clone())
                    }
                    Err(LoginError::Forbidden(e)) =>{
                        state.errors.error_message = Some(extract_message(&e));
                        state.errors.success_message = None;
                        state.profile = None;
                        state.token = Token{..Default::default()};
                        Task::future(async{
                            tokio::time::sleep(Duration::from_secs(2)).await;
                            Message::GoToResend
                        })
                    }
                    Err(LoginError::Other(e)) => {
                        state.errors.error_message = Some(extract_message(&e));
                        state.errors.success_message = None;

                        Task::none()
                    }
                }
            }
        },
        Message::Profile(profile_msg) => match profile_msg {
            ProfileMessage::ProfileStatus(status) => {
                match status {
                    Ok(profile) => {                     
                        state.profile = Some(profile);
                        
                        Task::future(async {
                            tokio::time::sleep(Duration::from_secs(2)).await;
                            Message::Profile(ProfileMessage::ProfilePage)
                        })
                    },
                    Err(e) => {
                        state.page = Page::Login;
                        state.errors.success_message = None;
                        state.errors.error_message = Some(extract_message(&e));
                        Task::none()
                    }
                }
            },
            ProfileMessage::ProfilePage => {
                state.page = Page::Profile;

                Task::none()
            }
        } 
        Message::Logout(logout_msg) => match logout_msg {
            LogoutMessage::LogoutPressed => {
                logout(state.token.clone())
            },
            LogoutMessage::LogoutStatus(status) => {
                match status {
                    Ok(r) => {
                        if let Some(profile) = state.profile.as_mut() {
                            profile.success_msg = Some(extract_message(&r));
                            profile.error_msg = None;
                        }
                        
                        state.token.token = None;
                        state.token.refresh_token = None;  
                        state.login_user_info.email = "".to_string();
                        state.login_user_info.password = "".to_string();

                        Task::future(async {
                            tokio::time::sleep(Duration::from_secs(2)).await;
                            Message::GoToLogin
                        })
                    },
                    Err(err) => {
                        if let Some(profile) = state.profile.as_mut() {
                           profile.error_msg = Some(extract_message(&err));
                            profile.success_msg = None;
                        }
                        
                        Task::none()
                    }
                }
            }
        }
        Message::Verify(verify_msg) => match verify_msg{
            VerifyMessage::EmailChanged(txt) => {
                state.login_user_info.email = txt;
                Task::none()
            },
            VerifyMessage::CodeChanged(txt) => {
                state.login_user_info.code = txt;
                Task::none()
            },
            VerifyMessage::VerifyPressed => {
                verify(state.login_user_info.email.clone(), state.login_user_info.code.clone())
            },
            VerifyMessage::VerifyStatus(result) => {
                match result {
                    Ok(r) => {
                        state.errors.success_message = Some(format!("{} - Redirecting...", extract_message(&r)));
                        state.errors.error_message = None;

                        Task::future(async {
                            tokio::time::sleep(Duration::from_secs(2)).await;
                            Message::GoToLogin
                        })
                    }
                    Err(err) => {
                        state.errors.error_message = Some(extract_message(&err));
                        state.errors.success_message = None;

                        Task::none()
                    }
                }
            },
            VerifyMessage::SendCodePressed => {
                send_code(state.login_user_info.email.clone())
            }
            VerifyMessage::SendCodeStatus(result) => {
                match result {
                    Ok(r) => {
                        state.errors.success_message = Some(extract_message(&r));
                        state.errors.error_message = None;
                        state.login_user_info.code_sended = true;

                        Task::none()
                    }
                    Err(err) => {
                        state.errors.error_message = Some(extract_message(&err));
                        state.errors.success_message = None;

                        Task::none()
                    }
                }
            }
        }
        Message::Register(register_msg) => match register_msg {
            RegisterMessage::UsernameChanged(txt) => {
                state.register_user_info.username = txt;
                Task::none()
            },
            RegisterMessage::EmailChanged(txt) => {
                state.register_user_info.email = txt;
                Task::none()
            },
            RegisterMessage::PasswordChanged(txt) => {
                state.register_user_info.password = txt;
                Task::none()
            },
            RegisterMessage::ToggleShowPassword => {
                state.register_user_info.show_password = !state.register_user_info.show_password;
                Task::none()
            },
            RegisterMessage::RegisterPressed => {
                register(
                    state.register_user_info.username.clone(),
                    state.register_user_info.email.clone(),
                    state.register_user_info.password.clone()
                )
            },
            RegisterMessage::RegisterStatus(status) => {
                match status {
                    Ok(r) => {
                        state.errors.success_message = Some(extract_message(&r));
                        state.errors.error_message = None;

                        Task::future(async {
                            tokio::time::sleep(Duration::from_secs(2)).await;
                            Message::GoToLogin
                        })
                    }
                    Err(err) => {
                        state.errors.error_message = Some(extract_message(&err));
                        state.errors.success_message = None;

                        Task::none()
                    }
                }
            }
        },
        Message::Tasks(task_msg) => match task_msg{
            TaskMessages::TaskStatus(status) => match status {
                Ok(result) => {
                    state.tasks = TaskData::Tasks(result);
                    Task::future(async{
                        Message::Tasks(TaskMessages::TaskPage)
                    })
                }
                Err(TaskError::Forbidden(err)) => {          
                    state.errors.error_message = Some(extract_message(&err));
                    state.profile = None;
                    state.token = Token { ..Default::default() };
                    Task::future(async{
                        tokio::time::sleep(Duration::from_secs(2)).await;
                        Message::GoToLogin
                    })
                }
                Err(TaskError::NotFound(err)) => {
                    state.tasks = TaskData::Text(extract_message(&err));

                    Task::future(async{
                        Message::Tasks(TaskMessages::TaskPage)
                    })
                },
                Err(TaskError::Other(err)) => {
                    state.tasks = TaskData::Text(extract_message(&err));

                    Task::future(async{
                        Message::Tasks(TaskMessages::TaskPage)
                    })
                }
            }
            TaskMessages::TaskPage => {
                state.page = Page::Tasks;

                Task::none()
            },
            TaskMessages::RefreshTasks => {
                state.task_err = None;
                state.task_suc = None;
                state.sort = None;

                get_tasks(state.token.clone())
            },
            TaskMessages::TaskDetails(task_id) => {
                state.selected_task = Some(task_id);
                Task::future(async move{
                    Message::GoToTaskDetails
                })
            },
            TaskMessages::TaskDelete(id) => {
                task_detail::delete(state.token.clone(), id)
            },
            TaskMessages::TaskDeleteStatus(status) => {
                match status {
                    Ok(result) => {
                        state.task_suc = Some(extract_message(&result));

                        Task::future(async{
                            Message::GoToTasks
                        })
                    },
                    Err(TaskError::Forbidden(err)) => {
                        state.errors.error_message = Some(extract_message(&err));
                        state.profile = None;
                        state.token = Token { ..Default::default() };
                        Task::future(async{
                            Message::GoToLogin
                        })
                    }
                    Err(TaskError::NotFound(err)) => {
                        state.task_err = Some(extract_message(&err));

                        Task::future(async{
                            Message::GoToTasks
                        })
                    }
                    Err(TaskError::Other(err)) => {
                        state.task_err = Some(extract_message(&err));

                        Task::future(async{
                            Message::GoToTasks
                        })
                    }
                }
            },
            TaskMessages::TaskDeleteCompleted => {
                delete_completed_tasks(state.token.clone())
            }
            
        },
        Message::TaskAdd(task_add_msg) => match task_add_msg{
            TaskAddMessage::TaskNameChanged(txt) => {
                state.add_task.name = txt;
                Task::none()
            },
            TaskAddMessage::TaskDescriptionEdited(action) => {
                state.add_task.description.perform(action);
                Task::none()
            },
            TaskAddMessage::OpenStartDatePicker => {
                state.add_task.show_start_date_picker = true;
                state.add_task.show_end_date_picker = false;
                Task::none()
            },
            TaskAddMessage::CloseStartDatePicker => {
                state.add_task.show_start_date_picker = false;
                Task::none()
            },
            TaskAddMessage::SubmitStartDate(date) => {
                state.add_task.start_date = Some(date);
                state.add_task.show_start_date_picker = false;
                Task::none()
            },
            TaskAddMessage::OpenEndDatePicker => {
                state.add_task.show_end_date_picker = true;
                state.add_task.show_start_date_picker = false;
                Task::none()
            },
            TaskAddMessage::CloseEndDatePicker => {
                state.add_task.show_end_date_picker = false;
                Task::none()
            },
            TaskAddMessage::SubmitEndDate(date) => {
                state.add_task.end_date = Some(date);
                state.add_task.show_end_date_picker = false;
                Task::none()
            },
            TaskAddMessage::PriorityChanged(val) => {
                state.add_task.priority = val;
                Task::none()
            },
            TaskAddMessage::TaskAddClicked => {
                create_task(state.token.clone(), &state.add_task)
            },
            TaskAddMessage::ResetStartDate => {
                state.add_task.show_start_date_picker = false;
                state.add_task.start_date = None;
                Task::none()
            },
            TaskAddMessage::ResetEndDate => {
                state.add_task.show_end_date_picker = false;
                state.add_task.end_date = None;
                Task::none()
            },
            TaskAddMessage::TaskAddStatus(task_add_msg) => match task_add_msg{
                Ok(v) => {
                    state.errors.success_message = Some(extract_message(&v));
                    state.errors.error_message = None;
                    Task::future(async {
                        tokio::time::sleep(Duration::from_secs(2)).await;
                        Message::GoToTasks
                    })
                },
                Err(task_add::TaskError::Forbidden(err)) => {
                    state.errors.error_message = Some(extract_message(&err));
                    state.errors.success_message = None;
                    state.profile = None;
                    state.token = Token{..Default::default()};
                    Task::future(async {
                        tokio::time::sleep(Duration::from_secs(2)).await;
                        Message::GoToLogin
                    })
                },
                Err(task_add::TaskError::Other(err)) => {
                    state.errors.error_message = Some(extract_message(&err));
                    state.errors.success_message = None;
                    Task::none()
                }
            }
        },
        Message::TaskUpdate(task_update_msg) => match task_update_msg{
            TaskUpdateMessage::TaskNameChanged(txt) => {
                state.update_task.name = txt;
                Task::none()
            },
            TaskUpdateMessage::TaskDescriptionEdited(action) => {
                state.update_task.description.perform(action);
                Task::none()
            },
            TaskUpdateMessage::OpenStartDatePicker => {
                state.update_task.show_start_date_picker = true;
                state.update_task.show_end_date_picker = false;
                Task::none()
            },
            TaskUpdateMessage::CloseStartDatePicker => {
                state.update_task.show_start_date_picker = false;
                Task::none()
            },
            TaskUpdateMessage::SubmitStartDate(date) => {
                state.update_task.start_date = Some(date);
                state.update_task.show_start_date_picker = false;
                Task::none()
            },
            TaskUpdateMessage::OpenEndDatePicker => {
                state.update_task.show_end_date_picker = true;
                state.update_task.show_start_date_picker = false;
                Task::none()
            },
            TaskUpdateMessage::CloseEndDatePicker => {
                state.update_task.show_end_date_picker = false;
                Task::none()
            },
            TaskUpdateMessage::SubmitEndDate(date) => {
                state.update_task.end_date = Some(date);
                state.update_task.show_end_date_picker = false;
                Task::none()
            },
            TaskUpdateMessage::ResetStartDate => {
                state.update_task.show_start_date_picker = false;
                state.update_task.start_date = None;
                Task::none()
            },
            TaskUpdateMessage::ResetEndDate => {
                state.update_task.show_end_date_picker = false;
                state.update_task.end_date = None;
                Task::none()
            },
            TaskUpdateMessage::PriorityChanged(val) => {
                state.update_task.priority = val;
                Task::none()
            },
            TaskUpdateMessage::TaskStatusChanged(val) => {
                state.update_task.status = val;
                Task::none()
            }
            TaskUpdateMessage::TaskUpdateClicked => {
                update_task(state.token.clone(), state.selected_task, &state.update_task)
            },
            TaskUpdateMessage::TaskUpdateStatus(status) => {
                match status{
                    Ok(v) => {
                        state.errors.success_message = Some(extract_message("Task successfully updated."));
                        state.errors.error_message = None;
                        
                        if let TaskData::Tasks(ref mut list) = state.tasks {
                            if let Some(task) = list.iter_mut().find(|x| Some(x.id) == state.selected_task) {
                                task.name = v.name;
                                task.status = v.status;
                                task.description = v.description;
                                task.start_date = v.start_date;
                                task.end_date = v.end_date;
                                task.priority_score = v.priority_score
                            }
                        }

                        Task::future(async{
                            tokio::time::sleep(Duration::from_secs(2)).await;
                            Message::GoToTaskDetails
                        })
                    },
                    Err(task_update::TaskError::Forbidden(err)) => {
                        state.errors.error_message = Some(extract_message(&err));
                        state.errors.success_message = None;
                        state.profile = None;
                        state.token = Token { ..Default::default() };
                        Task::future(async{
                            tokio::time::sleep(Duration::from_secs(2)).await;
                            Message::GoToLogin
                        })
                    },
                    Err(task_update::TaskError::Other(err)) => {
                        state.errors.error_message = Some(extract_message(&err));
                        state.errors.success_message = None;
                        Task::none()
                    }
                }
            }
        },
        Message::TaskFilter(task_filter_msg) => match task_filter_msg{
            TaskFilterMessage::SortBy(field) => {
                state.sort = match state.sort{
                    Some((current_field, direction)) if current_field == field => {
                        let new_direction = if direction == SortDirection::Asc{
                            SortDirection::Desc
                        }else{ 
                            SortDirection::Asc
                        };
                        Some((field, new_direction))
                    }
                    _ => {
                        Some((field, SortDirection::Asc))
                    }
                };
                get_tasks_with_filter(state.token.clone(), state.sort.unwrap().0, state.sort.unwrap().1)
            }
        }
    }
}
