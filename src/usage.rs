use crate::constants;

pub fn show_version() {
    println!(
        "{} version {}
Copyright (C) 2022 by Andreas Maus <maus@ypbind.de>
This program comes with ABSOLUTELY NO WARRANTY.

{} is distributed under the Terms of the GNU General
Public License Version 3. (http://www.gnu.org/copyleft/gpl.html)
",
        constants::NAME,
        constants::VERSION,
        constants::NAME
    );
}

pub fn show_usage() {
    show_version();
    println!(
        "Usage {} [-N|--no-job-nodes] [-V|--version] [-c <cluster>,...|--cluster=<cluster>,...]
        [-h|--help] [-l <addr>|--listen=<addr>] [-q|--quiet]

    -V                      Show version information
    --version

    -c <cluster>,...        Export metrics for comma separated list of clusters
    --cluster=<cluster>,... Default: export data for all SLURM clusters

    -h                      Show help text
    --help

    -l <addr>               Address to listen for Prometheus scrape requests
    --listen=<addr>         Default: {}

    -q                      Quiet operation. Only warning and error messages
    --quiet                 are logged
",
        constants::NAME,
        constants::DEFAULT_LISTEN_ADDRESS
    );
}
