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
    
    let matching_gp = meetings.iter().find(|m| {
        m["Name"].as_str().unwrap_or_default().to_lowercase().contains(&gp_lower)
    });

    let matching_gp = matching_gp.ok_or("No GP matches found")?;

    // Find the session
    let sessions: &Vec<Value> = matching_gp["Sessions"].as_array().ok_or("Sessions not found")?;

    let matching_session = sessions.iter().find(|s| {
        s["Name"].as_str().unwrap_or_default().to_lowercase() == session_lower
    });

    let matching_session = matching_session.ok_or("No sessions matches found")?;

    // Retrieve the session path
    let session_path: &str = matching_session["Path"].as_str().ok_or("Session path not found")?;

    Ok(session_path.to_string())
}
