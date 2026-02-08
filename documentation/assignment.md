# Specification

## Assignment

Create a simple Webserver that handles / serves HTTP Requests on a given port.
No language requirements and use of libraries is permitted (no complete webservice / http libraries).

### Minimal requirements

Application should run as Linux service serving HTML files from filesystem using HTTP protocol version 1.1.

### Additional additions / variations

- Other HTTP versions
- Load Balancer
- Reverse Proxy
- Auth
- Configuration-Management
- Caching

## Goal

Implement a simple standalone http 1.1 webserver inspired by nginx.
Main objectives:

- Basic HTTP 1.1 Specification compliant
- Directory serving
- Reverse proxy
- File based configuration
- Docker deployment
- Terminal user interface to visualize load

To make it more challenging, the implementation is in Rust (which I'm not familiar). 
Additionally usage of external libraries should be kept to a minimum.
Correctness and performance are a secondary priority that is demonstrated by a small bench suite. 

### Week 1

- Project setup
- Simple static implementation
- Familiarize with HTTP 1.1 specification

### Week 2

- Request parsing
- Directory file serving
- Simple configuration

### Week 3

- Multithreading
- Error handling
- Deployment
- Service interaction and better observability

### Week 4

- Full routing and action handling
- Reverse proxy
- Terminal user interface for observability
- Bench suite
- Docker setup
