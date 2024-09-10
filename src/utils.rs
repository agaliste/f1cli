use chrono::Datelike;
use reqwest::blocking::get;
use serde_json::Value;

pub fn year_validator(s: &str) -> Result<i32, String> {
    let year: i32 = s.parse::<i32>().map_err(|_| "Year must be a number")?;
    let current_year: i32 = chrono::Utc::now().year();
    if (2018..=current_year).contains(&year) {
        Ok(year)
    } else {
        Err(format!("Year must be between 2018 and {}", current_year))
    }
}

pub fn fetch_session_path(year: i32, gp: &str, session: &str) -> Result<String, Box<dyn std::error::Error>> {
    let url: String = format!("https://livetiming.formula1.com/static/{}/Index.json", year);
    let response: String = get(&url)?.text()?;
    let data: Value = serde_json::from_str(&response)?;

    // Find the GP meeting
    let meetings: &Vec<Value> = data["Meetings"].as_array().ok_or("Meetings not found")?;
    let gp_lower: String = gp.to_lowercase();
    let session_lower: String = session.to_lowercase();

    let matching_gps: Vec<&Value> = meetings.iter()
        .filter(|m: &&Value| m["Name"].as_str().unwrap_or_default().to_lowercase().contains(&gp_lower))
        .collect();

    if matching_gps.is_empty() {
        return Err("No GP matches found".into());
    } else if matching_gps.len() > 1 {
        return Err("Multiple GP matches found".into());
    }

    // Assuming a single match for GP, now find the session
    let sessions: &Vec<Value> = matching_gps[0]["Sessions"].as_array().ok_or("Sessions not found")?;
    let matching_sessions: Vec<&Value> = sessions.iter()
        .filter(|s: &&Value| s["Name"].as_str().unwrap_or_default().to_lowercase().contains(&session_lower))
        .collect();

    if matching_sessions.is_empty() {
        return Err("No sessions matches found".into());
    } else if matching_sessions.len() > 1 {
        return Err("Multiple Sessions matches found".into());
    }

    // Assuming a single match for session, retrieve the session path
    let session_path: &str = matching_sessions[0]["Path"].as_str().ok_or("Session path not found")?;

    Ok(session_path.to_string())
}
