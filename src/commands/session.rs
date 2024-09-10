use std::collections::HashMap;
use clap::{Subcommand, Args};
use reqwest::blocking as reqwest;
use serde::{Serialize, Deserialize};
use serde_json::{Number, Value};
use prettytable::{Table, row};
use std::error::Error;

use crate::utils::{fetch_session_path, year_validator};

// ************* TeamRadios struct ************* //
#[derive(Serialize, Deserialize, Debug)]
struct TeamRadios {
    #[serde(rename = "Captures")]
    captures: Vec<TeamRadio>,
}

#[derive(Serialize, Deserialize, Debug)]
struct TeamRadio {
    #[serde(rename = "Path")]
    path: String,

    #[serde(rename = "RacingNumber")]
    racing_number: String,

    #[serde(rename = "Utc")]
    utc: String,
}

// ************* INFO struct ************* //

#[derive(Serialize, Deserialize, Debug)]
struct ArchiveStatus {
    #[serde(rename = "Status")]
    status: String
}

#[derive(Serialize, Deserialize, Debug)]
struct Circuit {
    #[serde(rename = "Key")]
    key: Number,

    #[serde(rename = "ShortName")]
    short_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Country {
    #[serde(rename = "Code")]
    code: String,

    #[serde(rename = "Key")]
    key: Number,

    #[serde(rename = "Name")]
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Meeting {
    #[serde(rename = "Key")]
    key: Number,

    #[serde(rename = "Location")]
    location: String,

    #[serde(rename = "Name")]
    name: String,

    #[serde(rename = "OfficialName")]
    official_name: String,

    #[serde(rename = "Country")]
    country: Country,

    #[serde(rename = "Circuit")]
    circuit: Circuit,
}

#[derive(Serialize, Deserialize, Debug)]
struct Info {
    #[serde(rename = "Meeting")]
    meeting: Meeting,

    #[serde(rename = "ArchiveStatus")]
    archive_status: ArchiveStatus,

    #[serde(rename = "Name")]
    name: String,

    #[serde(rename = "Path")]
    path: String,

    #[serde(rename = "EndDate")]
    end_date: String,

    #[serde(rename = "StartDate")]
    start_date: String,
    
    #[serde(rename = "Type")]
    session_type: String,

    #[serde(rename = "GmtOffset")]
    gmt_offset: String,

    #[serde(rename = "Key")]
    key: Number,
}

// ************* Status struct ************* //
#[derive(Serialize, Deserialize, Debug)]
struct Series {
    #[serde(rename = "Lap")]
    lap: i32,

    #[serde(rename = "Utc")]
    utc_timestamp: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct StatusSeries {
    #[serde(rename = "TrackStatus")]
    track_status: Option<String>,

    #[serde(rename = "SessionStatus")]
    session_status: Option<String>,

    #[serde(rename = "Utc")]
    utc_timestamp: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Status {
    #[serde(rename = "StatusSeries")]
    status_series: Vec<StatusSeries>,

    #[serde(rename = "Series")]
    series: Vec<Series>,
}

// ************* Drivers struct ************* //
#[derive(Debug, Deserialize, Serialize)]
struct Driver {
    #[serde(rename = "BroadcastName")]
    broadcast_name: String,

    #[serde(rename = "CountryCode")]
    country_code: String,

    #[serde(rename = "FirstName")]
    first_name: String,

    #[serde(rename = "FullName")]
    full_name: String,

    #[serde(rename = "HeadshotUrl")]
    headshot_url: String,

    #[serde(rename = "LastName")]
    last_name: String,

    #[serde(rename = "Line")]
    line: i32,

    #[serde(rename = "RacingNumber")]
    racing_number: String,

    #[serde(rename = "Reference")]
    reference: String,

    #[serde(rename = "TeamColour")]
    team_colour: String,

    #[serde(rename = "TeamName")]
    team_name: String,

    #[serde(rename = "Tla")]
    tla: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Drivers {
    drivers: HashMap<String, Driver>,
}

// ************************** //

#[derive(Subcommand, Debug)]
pub enum SessionCommands {
    Info(SessionArgs),
    Status(SessionArgs),
    Radios(SessionArgs),
    Drivers(SessionArgs),
}

#[derive(Args, Debug)]
pub struct SessionArgs {
    #[clap(long, value_parser = year_validator)]
    pub year: i32,

    #[clap(long)]
    pub gp: String,

    #[clap(long)]
    pub session: String,

    #[clap(long)]
    pub json: bool,
}

pub fn handle_session_command(command: SessionCommands) {
    // Parsing session arguments based on the subcommand
    match command {
        SessionCommands::Info(args) => {
            fetch_session_data(args, "SessionInfo.json");
        },
        SessionCommands::Status(args) => {
            fetch_session_data(args, "SessionData.json");
        },
        SessionCommands::Radios(args) => {
            fetch_session_data(args, "TeamRadio.json");
        },
        SessionCommands::Drivers(args) => {
            fetch_session_data(args, "DriverList.json");
        },
    }
}

fn fetch_session_data(args: SessionArgs, key_frame_path: &str) {
    let session_path_result: Result<String, Box<dyn Error>> = fetch_session_path(args.year, &args.gp, &args.session);

    if let Err(e) = session_path_result {
        eprintln!("Error fetching session path: {}", e);
        return;
    }

    let full_url: String = format!("https://livetiming.formula1.com/static/{}{}", session_path_result.unwrap(), key_frame_path);
    let response: Result<String, ::reqwest::Error> = reqwest::get(&full_url).and_then(|res: reqwest::Response| res.text());

    match response {
        Ok(json_text) => {
            let json: Value = serde_json::from_str(&json_text).unwrap_or_default();
            if args.json {
                print_json(&json, &full_url, key_frame_path);
            } else {
                print_table(&json, &full_url, key_frame_path);
            }
        },
        Err(_) => println!("Error fetching the data."),
    }
}

fn print_json(json: &Value, base_url: &str, key_frame_path: &str) {
    match key_frame_path {
        "TeamRadio.json" => print_json_for_radios_subcommand(json, base_url.to_string()),
        _ => println!("{}", serde_json::to_string_pretty(json).unwrap()),
    }
}

fn print_table(json: &Value, base_url: &str, key_frame_path: &str) {
    match key_frame_path {
        "SessionInfo.json" => print_table_for_info_subcommand(json),
        "SessionData.json" => print_table_for_status_subcommand(json),
        "DriverList.json" => print_table_for_drivers_subcommand(json),
        "TeamRadio.json" => print_table_for_radios_subcommand(json, base_url.to_string()),
        _ => println!("Unsupported table print for '{}'", key_frame_path),
    }
}

fn print_table_for_drivers_subcommand(json: &Value) {
    let mut table: Table = Table::new();

    let drivers: HashMap<String, Driver> = serde_json::from_value(json.clone())
    .expect("Error parsing JSON");
                
    table.set_titles(row!["#", "Driver", "Country", "Team"]);

    for (key, driver) in drivers.iter() {
        table.add_row(row![key, driver.full_name, driver.country_code, driver.team_name]);
    }
    

    table.printstd();
}

fn print_json_for_radios_subcommand(json: &Value, base_url: String) {
    let mut team_radios: TeamRadios = serde_json::from_value(json.clone()).expect("Error parsing JSON");

    for team_radio in &mut team_radios.captures {
        team_radio.path = format!("{}{}", base_url.rsplitn(2, '/').last().unwrap(), team_radio.path);
    }

    println!("{}", serde_json::to_string_pretty(&team_radios).unwrap());
}

fn print_table_for_radios_subcommand(json: &Value, base_url: String) {
    let mut table: Table = Table::new();
                    
    table.set_titles(row!["#", "Audio Link", "UTC Timestamp"]);

    let mut team_radios: TeamRadios = serde_json::from_value(json.clone()).expect("Error parsing JSON");

    for team_radio in &mut team_radios.captures {
        let audio_url: String = format!("{}/{}", base_url.rsplitn(2, '/').last().unwrap(), team_radio.path);
        table.add_row(row![team_radio.racing_number, audio_url, team_radio.utc]);
    }

    table.printstd();
}

fn print_table_for_info_subcommand(json: &Value) {
    let mut table: Table = Table::new();

    let info: Info = serde_json::from_value(json.clone()).expect("Error parsing JSON");
                
    let meeting: Meeting = info.meeting;

    table.set_titles(row!["", format!("{} - {}", meeting.official_name, meeting.name)]);

    table.add_row(row!["Country", meeting.country.name]);
    table.add_row(row!["Location", meeting.location]);
    table.add_row(row!["Start Date", info.start_date]);
    table.add_row(row!["End Date", info.end_date]);
    table.add_row(row!["GMT Offset", info.gmt_offset]);

    table.printstd();
}

fn print_table_for_status_subcommand(json: &Value) {
    let mut table: Table = Table::new();
    table.set_titles(row!["TimeStamp", "Status", "Status Type"]);

    let status: Status = serde_json::from_value(json.clone()).expect("Error parsing JSON");

    for status_series in &status.status_series {
        let (status_type, status_value) = match &status_series.track_status {
            Some(track_status) => ("Track", track_status.as_str()),
            None => {
                let session_status: &str = status_series.session_status.as_deref().unwrap_or("");
                ("Session", session_status)
            }
        };
        table.add_row(row![status_series.utc_timestamp, status_value, status_type]);
    }

    table.printstd();
}
