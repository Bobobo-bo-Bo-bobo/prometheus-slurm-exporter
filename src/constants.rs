pub const NAME: &str = "prometheus-slurm-exporter";
pub const VERSION: &str = "1.0.0-20220327";
pub const DEFAULT_LISTEN_ADDRESS: &str = "localhost:9703";
pub const DEFAULT_METRICS_PATH: &str = "metrics";
pub const ROOT_HTML: &str = "<html>\n<head><title>SLURM exporter</title></head>\n<body>\n<h1>SLURM exporter</h1>\n<p><a href=\"/metric\">Metrics</a></p>\n</body>\n</html>\n";

pub const METRIC_PARTITIONS_NAME: &str = "partition_states";
pub const METRIC_PARTITIONS_HELP: &str = "State of partitions of each cluster";

pub const METRIC_JOBS_NODES_NAME: &str = "slurm_job_nodes";
pub const METRIC_JOBS_NODES_HELP: &str = "SLURM jobs number of allocated or requested nodes";
pub const METRIC_JOBS_TASKS_NAME: &str = "slurm_job_tasks";
pub const METRIC_JOBS_TASKS_HELP: &str = "SLURM jobs number of allocated or requested tasks";
pub const METRIC_JOBS_CPUS_NAME: &str = "slurm_job_cpus";
pub const METRIC_JOBS_CPUS_HELP: &str = "SLURM jobs number of allocated or requested CPUs";
