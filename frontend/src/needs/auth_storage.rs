use keyring::Entry;
use serde::{Deserialize, Serialize};

use crate::views::{login::Token, profile::{Profile, ProfileRole}};


const SERVICE:&str = "MorTask";
const USER:&str = "session";

#[derive(Debug, Clone, Serialize, Deserialize)]
struct StoredSession{
    token: Option<String>,
    refresh_token: Option<String>,
    username: String,
    email: String,
    created_at: String,
    role: ProfileRole
}

pub fn save_session(token: &Token, profile: &Profile) -> keyring::Result<()> {
    let entry = Entry::new(SERVICE, USER)?;

    let data = StoredSession{
        token: token.token.clone(),
        refresh_token: token.refresh_token.clone(),
        username: profile.username.clone(),
        email: profile.email.clone(),
        created_at: profile.created_at.clone(),
        role: profile.role.clone()
    };

    let json = serde_json::to_string(&data)
        .map_err(|e| keyring::Error::Invalid("Serialize problem".into(), e.to_string()))?;

    entry.set_password(&json)
}

pub fn load_session() -> Option<(Token, Profile)> {
    let entry = Entry::new(SERVICE, USER).ok()?;
    let json = match entry.get_password() {
        Ok(j) => j,
        Err(e) => {
            eprintln!("Couldnt read session: {e}"); 
            return None;
        }
    };
    let data: StoredSession = serde_json::from_str(&json).ok()?;

    let token = Token{
        token: data.token,
        refresh_token: data.refresh_token
    };

    let profile = Profile{
        username: data.username,
        email: data.email,
        created_at: data.created_at,
        role: data.role,
        success_msg: None,
        error_msg: None
    };

    Some((token, profile))
}

pub fn clear_session() {
    if let Ok(entry) = Entry::new(SERVICE, USER){
        let _ = entry.delete_credential();
    }
}