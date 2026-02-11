# Deployment

## Analysis

The webserver needs to be runnable as a Linux service or docker container.
Static files from the filesystem should be served.

## Design

The webserver serves a similar role to Nginx and Apache.
It is intended to serve files directly or act as a reverse proxy for other applications.

### Docker

The webserver can be run from a docker container.
Images are hosted on `ghcr.io/oliumbi/webserver:latest` and are built automatically on new releases via Github actions.
For performance the images are minimal and get built in with a dedicated build step.
Running in containers is done in the headless mode since no terminal is attached.

### Terminal

Alternatively the webserver can be run directly from a binary and with a terminal user interface for more detailed
insights.

## Configuration

```toml
headless = true                # defines headless mode, optional

[server]
threads = 4                    # number of worker threads, optional
connections = 1024             # number of concurrent connections, optional
port = 80                      # port, optinal
timeout_in_secs = 10           # timeout in seconds, optional
max_header_length = 8192       # max header length, optional
max_body_length = 1048576      # max body length, optional

[[routes]]

[routes.path.Exact]            # exact route path
exact = "/"                    # exact path
methods = "GET"                # method, optional

[routes.action.Redirect]       # redirect route action 
location = "index.html"        # redirect location
status_code = 307              # redirect status code

[[routes]]

[routes.path.Regex]            # regex route path
regex = "/(something|other)"   # regex path
method = "GET,POST,PUT,DELETE" # method, optional

[routes.action.Proxy]          # proxy route action
location = "localhost:8080"    # proxy location

[[routes]]

[routes.path.Prefix]           # prefix route path
prefix = "/"                   # prefix path
methods = "GET,POST"           # method, optional

[routes.action.Fixed]          # fixed route action
root = "./example/demo/"       # root directory
fallback = "./notfound.html"   # fallback file, optional
```

## Example

In the folder `/example` is an example setup to run the webserver as a docker container.
The service get orchestrated with docker compose where the directory get mounted and the environment points to the
configuration. 

