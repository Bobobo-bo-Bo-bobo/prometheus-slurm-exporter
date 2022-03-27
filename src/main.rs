#[macro_use]
extern crate simple_error;

mod constants;
mod exporter;
mod logging;
mod slurm;
mod usage;

use getopts::Options;
use log::error;
use std::error::Error;
use std::net::ToSocketAddrs;
use std::{env, process};
use warp::Filter;

#[tokio::main]
async fn main() {
    let argv: Vec<String> = env::args().collect();
    let mut options = Options::new();
    let mut log_level = log::LevelFilter::Info;

    options.optflag("V", "version", "Show version information");
    options.optflag("h", "help", "Show help text");
    options.optflag("q", "quiet", "Quiet operation");
    options.optopt(
        "l",
        "listen",
        "listen address",
        "Address to listen for scrape requests",
    );

    let opts = match options.parse(&argv[1..]) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error: Can't parse command line arguments ({})", e);
            println!();
            usage::show_usage();
            process::exit(1);
        }
    };

    if opts.opt_present("h") {
        usage::show_usage();
        process::exit(0);
    }

    if opts.opt_present("V") {
        usage::show_version();
        process::exit(0);
    }

    if opts.opt_present("q") {
        log_level = log::LevelFilter::Warn;
    }

    let listen_address = opts
        .opt_str("l")
        .unwrap_or_else(|| constants::DEFAULT_LISTEN_ADDRESS.to_string());

    match logging::init(log_level) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error: Initialisation of logging failed: {}", e);
            process::exit(1);
        }
    };

    exporter::register();

    let socketaddr = match socketaddr_from_listen(&listen_address) {
        Ok(v) => v,
        Err(e) => {
            error!("Can't resolve {} to socket address: {}", listen_address, e);
            process::exit(1);
        }
    };

    let prometheus_route = warp::path(constants::DEFAULT_METRICS_PATH)
        .and(warp::get())
        .map(exporter::metrics);

    let root_route = warp::path::end()
        .and(warp::get())
        .map(move || warp::reply::html(constants::ROOT_HTML.to_string()));

    let route = root_route.or(prometheus_route);
    warp::serve(route).run(socketaddr).await;
}

pub fn socketaddr_from_listen(listen: &str) -> Result<std::net::SocketAddr, Box<dyn Error>> {
    let sockaddrs = listen.to_socket_addrs()?;
    let addresses: Vec<_> = sockaddrs.collect();
    if addresses.is_empty() {
        bail!("can't resolve listener address");
    }
    Ok(addresses[0])
}
