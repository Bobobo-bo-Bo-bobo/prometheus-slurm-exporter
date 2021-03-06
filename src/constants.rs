pub const NAME: &str = "prometheus-slurm-exporter";
pub const VERSION: &str = "1.2.1-20220406";
pub const DEFAULT_LISTEN_ADDRESS: &str = "localhost:9703";
pub const DEFAULT_METRICS_PATH: &str = "metrics";
pub const ROOT_HTML: &str = "<html>\n<head><title>SLURM exporter</title></head>\n<body>\n<h1>SLURM exporter</h1>\n<p><a href=\"/metric\">Metrics</a></p>\n</body>\n</html>\n";
pub const SLURM_CLUSTERS: &str = "all";

pub const METRIC_PARTITIONS_NAME: &str = "partition_states";
pub const METRIC_PARTITIONS_HELP: &str = "State of partitions of each cluster";

pub const METRIC_JOBS_NODES_NAME: &str = "slurm_job_nodes";
pub const METRIC_JOBS_NODES_HELP: &str = "SLURM jobs: number of allocated or requested nodes";
pub const METRIC_JOBS_TASKS_NAME: &str = "slurm_job_tasks";
pub const METRIC_JOBS_TASKS_HELP: &str = "SLURM jobs: number of allocated or requested tasks";
pub const METRIC_JOBS_CPUS_NAME: &str = "slurm_job_cpus";
pub const METRIC_JOBS_CPUS_HELP: &str = "SLURM jobs: number of allocated or requested CPUs";
pub const METRIC_JOBS_COUNT_NAME: &str = "slurm_job_count";
pub const METRIC_JOBS_COUNT_HELP: &str = "Number of SLURM jobs in a given state";

pub const BITMASK_JOB_COUNT: u8 = 0x01;
pub const BITMASK_JOB_CPUS: u8 = 0x02;
pub const BITMASK_JOB_NODES: u8 = 0x04;
pub const BITMASK_JOB_TASKS: u8 = 0x08;
pub const BITMASK_PARTITIONS: u8 = 0x10;
