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

    let mut job_cpus = true;
    let mut job_count = true;
    let mut job_nodes = true;
    let mut job_tasks = true;
    let mut partitions = true;

    options.optflag("C", "no-job-cpus", "Don't export job CPUs");
    options.optflag("D", "debug", "Enable debug mode");
    options.optflag("J", "no-job-count", "Don't export number of jobs");
    options.optflag("N", "no-job-nodes", "Don't export number of nodes for jobs");
    options.optflag("P", "no-partitions", "Don't export partition states");
    options.optflag("T", "no-job-tasks", "Don't export number of tasks for jobs");
    options.optflag("V", "version", "Show version information");
    options.optflag("h", "help", "Show help text");
    options.optflag("q", "quiet", "Quiet operation");
    options.optopt("c", "cluster", "cluster", "Export data for given cluster");
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

    if opts.opt_present("C") {
        job_cpus = false;
    }

    if opts.opt_present("J") {
        job_count = false;
    }

    if opts.opt_present("N") {
        job_nodes = false;
    }

    if opts.opt_present("P") {
        partitions = false;
    }

    if opts.opt_present("T") {
        job_tasks = false;
    }

    let clusters = opts
        .opt_str("c")
        .unwrap_or_else(|| constants::SLURM_CLUSTERS.to_string());

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

    let mut export_bitmask: u8 = 0x00;
    if job_cpus {
        export_bitmask |= constants::BITMASK_JOB_CPUS;
    }
    if job_count {
        export_bitmask |= constants::BITMASK_JOB_COUNT;
    }
    if job_nodes {
        export_bitmask |= constants::BITMASK_JOB_NODES;
    }
    if job_tasks {
        export_bitmask |= constants::BITMASK_JOB_TASKS;
    }
    if partitions {
        export_bitmask |= constants::BITMASK_PARTITIONS;
    }

    exporter::register(export_bitmask);

    let socketaddr = match socketaddr_from_listen(&listen_address) {
        Ok(v) => v,
        Err(e) => {
            error!("Can't resolve {} to socket address: {}", listen_address, e);
            process::exit(1);
        }
    };

    let prometheus_route = warp::path(constants::DEFAULT_METRICS_PATH)
        .and(warp::get())
        .map(move || exporter::metrics(&clusters, export_bitmask));

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
