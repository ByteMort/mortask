use chrono::DateTime;
use iced::{Theme, window::icon};
use reqwest::header::{HeaderMap, SET_COOKIE};

use crate::State;



pub fn extract_message(input: &str) -> String {
    serde_json::from_str::<serde_json::Value>(input)
        .ok()
        .and_then(|v| v.get("message")?.as_str().map(String::from))
        .unwrap_or_else(|| input.to_string()) 
}

pub fn extract_tokens_from_cookies(headers: &HeaderMap) -> (Option<String>, Option<String>){
    let mut request_token = None;
    let mut refresh_token = None;

    for cookie in headers.get_all(SET_COOKIE){
        if let Ok(cookie_str) = cookie.to_str(){
            if let Some(first_part) = cookie_str.split(";").next() {
                if let Some((key, value)) = first_part.split_once("="){
                    match key.trim() {
                        "token" => request_token = Some(value.trim().to_string()),
                        "refresh_token" => refresh_token = Some(value.trim().to_string()),
                        _ => {}
                    }
                }
            }
        }
    }

    (request_token, refresh_token)
}

pub fn format_date_time(date_str: &str) -> String{
    if let Ok(parsed_date) = DateTime::parse_from_rfc3339(date_str){
        parsed_date.format("%d.%m.%Y %H:%M").to_string()
    }else{
        "Date could not be resolved.".to_string()
    }
}

pub fn load_icon(path: &str) -> icon::Icon{
    let img = image::open(path)
        .expect("Something went wrong while finding image!")
        .into_rgba8();

    let (width, height) = img.dimensions();
    let rgba = img.into_raw();

    icon::from_rgba(rgba, width, height)
        .expect("Something went wrong while loading image!")
}

pub fn theme(_state: &State) -> Theme{
    Theme::Dracula
}