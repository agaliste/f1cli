# F1CLI

F1CLI is a command-line interface (CLI) tool built with Rust. It provides various commands to interact with Formula 1 static post session data.

Generally, F1 generates the data 30 minutes after the session ends. This tool provides a way to access the data from the terminal.

## Introduction

This project is not serious and is just a fun way I found to learn Rust. So the pace of development will be slow and the features will be limited, or maybe not. Who knows?

## How does it work?

The tool uses the `livetiming.formula1.com/static/` API to fetch the data. The data is fetched in JSON format and then parsed to display the information in a human-readable format.

This API "as far as I know" is public but not documented. So, the tool might break if the API changes. I'll try to keep it updated as much as possible.

## Functionality

These are the features that are planned to be implemented or are already implemented. If I've missed something, I'll add it later.

The information available ranges from 2018 to the present.

### Session Information

-   [ ] List all the sessions available
-   [x] Get the session information
-   [x] Get the Team Radios
-   [x] Get the Drivers
-   [x] Get the Statuses (Flags, etc...)
-   [ ] Get the Standings
-   [ ] Get the Fastest Laps
-   [ ] Get the Pit Stops
-   [ ] Get the Lap Times
-   [ ] Get the top 3

### Season Information

-   [ ] List all the seasons available
-   [ ] Get the Standings
-   [ ] Get the Drivers
-   [ ] Get the Teams
-   [ ] Get the Standings
-   [ ] Get the Constructor Standings
-   [ ] Get the Calendar

## Building the Project

To build the project, use the following command:

```bash
cargo build
```

## Running the Project

To run the project, use the following command:

```bash
cargo run
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

##  Notice

F1CLI is unofficial and is not associated in any way shape or from with the Formula 1 companies. F1, FORMULA ONE, FORMULA 1, FIA FORMULA ONE WORLD CHAMPIONSHIP, GRAND PRIX and related marks are trade marks of Formula One Licensing B.V.
