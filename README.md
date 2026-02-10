# Webserver
FHNW Individual Project - Webserver

## Documentation

| File               | Description                                                        |
|--------------------|--------------------------------------------------------------------|
| Assignment         | Assignement from the FHNW Module and defined Goals for the Project |
| Software           | Function / Non-Function Requirments, Architecture, Testing         |
| Infrastructure     | Role, Setup, Configuration, example                                |
| HTTP Specification | Overview of HTTP Specification compliance                          |
| Knowledge          | Goals, learnings and notes                                         |
| Timetable          | Schedule overview                                                  |
| Meetings           | Coach meeting notes                                                |
| Diary              | Daily work overview                                                |
| References         | Used resources and AI usage                                        |

## Source

In the `/crates` folder are the two applications:
- `webserver`
  - The server implementation, for more details `/documentation/software.md`
- `bench`
  - Tool to validate Functional and Non-Functional requirements like specification compliance or load tests

## Build and Run

Run the application `cargo run --package webserver`

Build executable `cargo build --package webserver`

