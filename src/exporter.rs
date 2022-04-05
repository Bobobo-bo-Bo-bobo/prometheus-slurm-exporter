use crate::constants;
use crate::slurm;

use lazy_static::lazy_static;
use log::error;
use prometheus::{IntGaugeVec, Opts, Registry, TextEncoder};

lazy_static! {
    pub static ref REGISTRY: Registry = Registry::new();
    pub static ref JOBS_NODES: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_JOBS_NODES_NAME,
            constants::METRIC_JOBS_NODES_HELP
        ),
        &["cluster", "partition", "state"],
    )
    .unwrap();
    pub static ref JOBS_TASKS: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_JOBS_TASKS_NAME,
            constants::METRIC_JOBS_TASKS_HELP
        ),
        &["cluster", "partition", "state"],
    )
    .unwrap();
    pub static ref JOBS_CPUS: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_JOBS_CPUS_NAME,
            constants::METRIC_JOBS_CPUS_HELP
        ),
        &["cluster", "partition", "state"],
    )
    .unwrap();
    pub static ref PARTITIONS: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_PARTITIONS_NAME,
            constants::METRIC_PARTITIONS_HELP
        ),
        &["cluster", "partition", "state"],
    )
    .unwrap();
    pub static ref JOBS_COUNT: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            constants::METRIC_JOBS_COUNT_NAME,
            constants::METRIC_JOBS_COUNT_HELP,
        ),
        &["cluster", "partition", "state"],
    )
    .unwrap();
}

pub fn register(bitmask: u8) {
    if bitmask & constants::BITMASK_JOB_NODES == constants::BITMASK_JOB_NODES {
        REGISTRY.register(Box::new(JOBS_NODES.clone())).unwrap();
    }
    if bitmask & constants::BITMASK_JOB_TASKS == constants::BITMASK_JOB_TASKS {
        REGISTRY.register(Box::new(JOBS_TASKS.clone())).unwrap();
    }
    if bitmask & constants::BITMASK_JOB_CPUS == constants::BITMASK_JOB_CPUS {
        REGISTRY.register(Box::new(JOBS_CPUS.clone())).unwrap();
    }
    if bitmask & constants::BITMASK_JOB_COUNT == constants::BITMASK_JOB_COUNT {
        REGISTRY.register(Box::new(JOBS_COUNT.clone())).unwrap();
    }
    if bitmask & constants::BITMASK_PARTITIONS == constants::BITMASK_PARTITIONS {
        REGISTRY.register(Box::new(PARTITIONS.clone())).unwrap();
    }
}

pub fn metrics(slurm_cluster: &str, bitmask: u8) -> String {
    let encoder = TextEncoder::new();
    let mut buffer = String::new();

    if bitmask & constants::BITMASK_PARTITIONS == constants::BITMASK_PARTITIONS {
        if let Err(e) = slurm::update_partition_metrics(slurm_cluster) {
            error!("Can't update SLURM partition metrics: {}", e);
            return buffer;
        }
    }

    if let Err(e) = slurm::update_job_metrics(slurm_cluster, bitmask) {
        error!("Can't update SLURM job metrics: {}", e);
        return buffer;
    }

    if let Err(e) = encoder.encode_utf8(&REGISTRY.gather(), &mut buffer) {
        error!("Can't encode metrics as UTF8 string: {}", e);
    }

    if let Err(e) = encoder.encode_utf8(&prometheus::gather(), &mut buffer) {
        error!("Can't encode metrics as UTF8 string: {}", e);
    };
    buffer
}
