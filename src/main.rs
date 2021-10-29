#[macro_use]
extern crate serde_derive;
extern crate clap;
extern crate dockworker;
extern crate log;
extern crate pretty_bytes;

use dockworker::Docker;
use log::{info, warn, error, LevelFilter};
use std;

mod clean;
mod size;

mod logger;

fn main() {
    let args = parse_args();

    log::set_logger(&logger::CONSOLE_LOGGER).unwrap();
    let quiet_mode = args.is_present("quiet");
    let verbose_mode = args.is_present("verbose");

    if verbose_mode {
        log::set_max_level(LevelFilter::Debug);
    } else if quiet_mode {
        log::set_max_level(LevelFilter::Warn);
    } else {
        log::set_max_level(LevelFilter::Info);
    }

    let docker_connection = Docker::connect_with_defaults();
    if docker_connection.is_err() {
        error!("Error while connecting to docker: {:?}", docker_connection);
        std::process::exit(1);
    }
    let docker_client = docker_connection.unwrap();

    let is_alive = docker_client.ping();
    if is_alive.is_err() {
        error!("Error while connecting to docker: {:?}", is_alive);
        std::process::exit(1);
    }

    if let Some(_) = args.subcommand_matches("clean") {
        info!("Going make some cleanup here....");
        clean::clean(&docker_client)
    } else if let Some(_) = args.subcommand_matches("size") {
        size::show_size(&docker_client)
    } else {
        warn!("Please, provide command to run. See 'docu --help' for more info")
    }
}

fn parse_args() -> clap::ArgMatches<'static> {
    clap::App::new("Docker utils")
        .version("0.1.0")
        .author("Alex <aleosd@gmail.com>")
        .about("Collection of usefull utils to operate with docker")
        .arg(
            clap::Arg::with_name("verbose")
                .help("Print additional info to console")
                .short("v")
                .long("verbose")
                .conflicts_with("quiet"),
        )
        .arg(
            clap::Arg::with_name("quiet")
                .help("Limit console output to errors only")
                .short("q")
                .conflicts_with("verbose")
                .long("quiet"),
        )
        .subcommand(
            clap::SubCommand::with_name("clean").about("Cleans outdated images and containers"),
        )
        .subcommand(clap::SubCommand::with_name("size").about("Show size of images"))
        .get_matches()
}
