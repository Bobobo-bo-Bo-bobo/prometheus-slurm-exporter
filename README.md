# General information
This is an exporter to export job and partition state information of the (https://slurm.schedmd.com/)[SLURM workload manager]to be scraped by (https://prometheus.io)[Prometheus].

# Information
## Building
As a Rust program, a stable Rust build chain is required to build the exporter

## Running
The node running this exporter *must* have a valid SLURM configuration and the SLURM client binaries - `sinfo` and `squeue` - installed and in it's path.

# Command line options

| *Option* | *Parameter* | *default* | *Description* |
|:---------|:------------|:----------|:--------------|
| `-V` / `--version` | - | - | Show version information |
| `-h` / `--help` | - | - | Show help information |
| `-q` / `--quiet` | - | - | Quiet operation, only warnings and errors are logged |
| `-l` / `--listen` | `<addr>` | `localhost:9703` | Address to listen for Prometheus scrapes |

