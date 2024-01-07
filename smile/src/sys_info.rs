use std::env::var;

// get user
pub fn get_user_info() -> String {
    match var("USER") {
        Ok(username) => username,
        Err(_) => "-".to_string(),
    }
}

// get hostname
pub fn get_host_info() -> String {
    match var("HOST") {
        Ok(host) => host,
        Err(e) => e.to_string(),
    }
}

// get distro

// get kernel

// get uptime

// get app info ??
