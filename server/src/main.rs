use failure::Fail;

mod api;
mod error;
mod maze;

fn main() {
    env_logger::init();

    if let Err(err) = api::run_server("localhost:4000") {
        for cause in Fail::iter_chain(&err) {
            println!("{}: {}", cause.name().unwrap_or("Error"), cause);
        }
    }
}
