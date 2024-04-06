use chrono::Datelike;
use reqwest::blocking::get;
use serde_json::Value;

pub fn year_validator(s: &str) -> Result<i32, String> {
    let year = s.parse::<i32>().map_err(|_| "Year must be a number")?;
    let current_year = chrono::Utc::now().year();
    if (2018..=current_year).contains(&year) {
        Ok(year)
    } else {
        Err(format!("Year must be between 2018 and {}", current_year))
    }
}

pub fn fetch_session_path(year: i32, gp: &str, session: &str) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!("https://livetiming.formula1.com/static/{}/Index.json", year);
    let response = get(&url)?.text()?;
    let data: Value = serde_json::from_str(&response)?;

    let meetings = data["Meetings"].as_array().ok_or("Meetings not found")?;
    let gp_lower = gp.to_lowercase();
    let session_lower = session.to_lowercase();

    // Filter for matching GP
    let matching_gps: Vec<_> = meetings.iter()
        .filter(|m| m["Name"].as_str().unwrap_or_default().to_lowercase().contains(&gp_lower))
        .collect();

    // Handle multiple GP matches or no matches
    if matching_gps.len() > 1 {
        println!("Multiple GPs found matching '{}'. Please refine your search.", gp);
        return Err("Multiple GP matches found".into());
    } else if matching_gps.len() < 1 {
        println!("No GPs found matching '{}'. Please refine your search.", gp);
        return Err("No GP matches found".into());
    }

    // Assuming a single match for GP, now find the session
    let sessions = matching_gps[0]["Sessions"].as_array().ok_or("Sessions not found")?;
    let matching_sessions: Vec<_> = sessions.iter()
        .filter(|s| s["Name"].as_str().unwrap_or_default().to_lowercase().contains(&session_lower))
        .collect();

    // Handle multiple session matches or no matches
    if matching_sessions.len() > 1 {
        println!("Multiple sessions found matching '{}'. Please refine your search.", session);
        return Err("Multiple Sessions matches found".into());
    } else if matching_sessions.len() < 1 {
        println!("No sessions found matching '{}'. Please refine your search.", session);
        return Err("No sessions matches found".into());
    }

    // Assuming a single match for session, retrieve the session path
    let session_path = matching_sessions[0]["Path"].as_str().ok_or("Session path not found")?;

    Ok(session_path.to_string())
}
