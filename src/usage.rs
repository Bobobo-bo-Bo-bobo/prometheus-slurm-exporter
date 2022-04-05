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
        "Usage {} [-C|--no-job-cpus] [-J|--no-job-count] [-N|--no-job-nodes] [-T|--no-job-tasks]
        [-V|--version] [-c <cluster>,...|--cluster=<cluster>,...] [-h|--help]
        [-l <addr>|--listen=<addr>] [-q|--quiet]

    -C                      Don't export number of allocated or requested CPUs for jobs
    --no-job-cpus

    -J                      Don't export number of jobs
    --no-job-count

    -N                      Don't export number of allocated or requested nodes for jobs
    --no-job-nodes

    -P                      Don't export SLURM partition states
    --no-partitions

    -T                      Don't export number of allocated or requested tasks for jobs
    --no-job-tasks

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
