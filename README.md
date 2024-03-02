# Observatory

Welcome to my experimental Blog software. I use this to learn various things about Rust and to build  a high performance
website engine that can host my personal blog website. Please explore the rest of this README to get a better understanding
of what I'm building.

## Getting started

* Clone this repository
* Run `npm install && npm run build` in the `admin` directory of the repository.
* Run `cargo run` in the root of the repository.

## Documentation

### Libraries used in this project

* [Tokio](https://tokio.rs/) - Async runtime.
* [Axum](https://docs.rs/axum/latest/axum/) - HTTP webserver framework.
* [Tower HTTP](https://docs.rs/tower-http/latest/tower_http/) - HTTP middleware for use with Axum.
* [Clap](https://docs.rs/clap/latest/clap/) - Command-line argument parser. 


### Project structure

To keep things sane, I split the project into feature slices. Please find the description of the feature slices
below:

* `src/admin` - The admin panel endpoints and business logic
* `src/frontpage` - The front page endpoints and business logic

I use Vue to implement the admin panel pages. You can find the sources under `admin`. The pages connect
to the API endpoints in `src/admin/endpoints.rs`.