# Contributing
Thanks for your interest in contributing to Squint!

## System Pre-requisites
There's three core components you'll need to have in your environment in order to work on the Squint codebase:

- [Rust](https://rustup.rs/) (Be sure to use rustup and not a pre-installed package)
  - Contributors are encouraged to install the `clippy` and `rustfmt` components as well.
- [Docker](https://docs.docker.com/get-docker/)
- [GTK 3](https://www.gtk.org/docs/installations/) - *NB* you can skip this step if you are content to just build and run locally using Docker

Windows users are encouraged to use the [Windows Subsystem for Linux 2 (WSL2)](https://docs.microsoft.com/en-us/windows/wsl/install) given the GTK dependency and Docker-based runtime, however, you may go for the MSYS2 route if you prefer.

### Optional Font Packs

Depending on your system, you may also need to install some font packs if you planning on running the Squint binary directly on your system instead of a Docker container:

- Microsoft's TrueType Core Fonts
  - Installation methods and exact package name vary by system, but can typically be found by searching for variants of `ms ttf core fonts`
- A font pack that covers the full unicode block in order to handle emoji. The Squint production environment utilizes the `Noto Color Emoji` font.

## Development Activities

Squint development tasks/activities utilize canonical [cargo](https://github.com/rust-lang/cargo) command invocations, and as a recommendation, `docker` commands for running the application.

- `cargo build` - compiles the application
- `cargo test` - runs the automated tests
- `cargo run` - used to start the application, though we encourage using docker to run the application locally to mirror production, see [below documentation for running the application](#running-the-application-locally)
- `cargo fmt` - rewrites the code to comply with formatting rules
- `cargo fmt -- --check` - checks if the current code formatting complies with formatting rules
- `cargo clippy` - runs the linter
- `cargo clippy --all-targets -- -D warnings` - runs the linter with the full rule set

## Running the Application Locally

We strongly encourage running the application locally in a Docker container, both to simplify the management of system dependencies, and also so that the runtime, and thus the generated badges, will be the same on your machine as they are in the staging and production environments.

Note that you may need to prefix the below commands with `sudo` depending on whether you've [configured your system to be able to manage docker as a non-root user](https://docs.docker.com/engine/install/linux-postinstall/)

First you'll need to build the Docker image. Note that the first time you do this it may take a while, but subsequent builds should be much faster due to layer caching. You may name the image whatever you want, but we'd recommend using a name for the image that clearly indicates it is your local copy, e.eg.:

```shell
docker build . -t squint-local
```

You can then use the image you've built to start a container. By default the server will listen on port `3001`, however, you can change this by setting your desired port number in the `PORT` environment variable, e.g.:

```shell
docker run -d -p 3001:3001 --name local-squint-container squint-local
# or
docker run -d -p 4000:8080 -e PORT=8080 --name local-squint-container squint-local
```

Your local instance should then be available to serve up raster badges utilizing the host port, e.g.:

`http://localhost:3001/badge/foo-bar-blue`

or, using the second example from above mapped to host port 4000

`http://localhost:4000/badge/foo-bar-yellow`

You can stop and remove the container via standard Docker commands, e.g. 

```shell
docker stop local-squint-container && sudo docker rm local-squint-container
```
