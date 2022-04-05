use crate::constants;
use crate::exporter;
use std::collections::HashMap;
use std::error::Error;
use std::process::Command;

pub fn update_job_metrics(slurm_cluster: &str, bitmask: u8) -> Result<(), Box<dyn Error>> {
    // <cluster>: {
    //  <partition>: {
    //      <state>: <count>
    //  }
    // }
    let mut job_node_states: HashMap<String, HashMap<String, HashMap<String, i64>>> =
        HashMap::new();
    let mut job_task_states: HashMap<String, HashMap<String, HashMap<String, i64>>> =
        HashMap::new();
    let mut job_cpu_states: HashMap<String, HashMap<String, HashMap<String, i64>>> = HashMap::new();
    let mut job_count_states: HashMap<String, HashMap<String, HashMap<String, i64>>> =
        HashMap::new();

    let squeue = Command::new("squeue")
        .arg("--noheader")
        .arg("--Format=Cluster,Partition,State,NumNodes,NumTasks,NumCPUs")
        .arg(format!("--clusters={}", slurm_cluster))
        .arg("--all")
        .output()?;

    let rc = match squeue.status.code() {
        Some(v) => v,
        None => {
            bail!("Can't get return code of squeue command");
        }
    };

    if !squeue.status.success() {
        bail!("squeue command exited with non-normal exit code {}", rc);
    }

    let stdout = String::from_utf8(squeue.stdout)?;
    for line in stdout.lines() {
        let (c, p, s, nodes, tasks, cpus) = match split_job_state_line(line) {
            Some(v) => v,
            None =>  bail!(
                "Can't extract tuple cluster, partition, state, nodes, tasks and cpus from output '{}'",
                line
            ),
        };
        if bitmask & constants::BITMASK_JOB_NODES == constants::BITMASK_JOB_NODES {
            let cluster = job_node_states
                .entry(c.clone())
                .or_insert_with(HashMap::<String, HashMap<String, i64>>::new);
            let partition = cluster
                .entry(p.clone())
                .or_insert_with(HashMap::<String, i64>::new);
            *partition.entry(s.clone()).or_insert(0) += nodes;
        }

        if bitmask & constants::BITMASK_JOB_TASKS == constants::BITMASK_JOB_TASKS {
            let cluster = job_task_states
                .entry(c.clone())
                .or_insert_with(HashMap::<String, HashMap<String, i64>>::new);
            let partition = cluster
                .entry(p.clone())
                .or_insert_with(HashMap::<String, i64>::new);
            *partition.entry(s.clone()).or_insert(0) += tasks;
        }

        if bitmask & constants::BITMASK_JOB_CPUS == constants::BITMASK_JOB_CPUS {
            let cluster = job_cpu_states
                .entry(c.clone())
                .or_insert_with(HashMap::<String, HashMap<String, i64>>::new);
            let partition = cluster
                .entry(p.clone())
                .or_insert_with(HashMap::<String, i64>::new);
            *partition.entry(s.clone()).or_insert(0) += cpus;
        }

        if bitmask & constants::BITMASK_JOB_COUNT == constants::BITMASK_JOB_COUNT {
            let cluster = job_count_states
                .entry(c)
                .or_insert_with(HashMap::<String, HashMap<String, i64>>::new);
            let partition = cluster.entry(p).or_insert_with(HashMap::<String, i64>::new);
            *partition.entry(s).or_insert(0) += 1;
        }
    }

    if bitmask & constants::BITMASK_JOB_NODES == constants::BITMASK_JOB_NODES {
        for (clu, cpart) in job_node_states.iter() {
            for (part, pstate) in cpart.iter() {
                for (state, count) in pstate.iter() {
                    exporter::JOBS_NODES
                        .with_label_values(&[clu, part, state])
                        .set(*count);
                }
            }
        }
    }

    if bitmask & constants::BITMASK_JOB_TASKS == constants::BITMASK_JOB_TASKS {
        for (clu, cpart) in job_task_states.iter() {
            for (part, pstate) in cpart.iter() {
                for (state, count) in pstate.iter() {
                    exporter::JOBS_TASKS
                        .with_label_values(&[clu, part, state])
                        .set(*count);
                }
            }
        }
    }

    if bitmask & constants::BITMASK_JOB_CPUS == constants::BITMASK_JOB_CPUS {
        for (clu, cpart) in job_cpu_states.iter() {
            for (part, pstate) in cpart.iter() {
                for (state, count) in pstate.iter() {
                    exporter::JOBS_CPUS
                        .with_label_values(&[clu, part, state])
                        .set(*count);
                }
            }
        }
    }

    if bitmask & constants::BITMASK_JOB_COUNT == constants::BITMASK_JOB_COUNT {
        for (clu, cpart) in job_count_states.iter() {
            for (part, pstate) in cpart.iter() {
                for (state, count) in pstate.iter() {
                    exporter::JOBS_COUNT
                        .with_label_values(&[clu, part, state])
                        .set(*count);
                }
            }
        }
    }
    Ok(())
}

pub fn update_partition_metrics(slurm_cluster: &str) -> Result<(), Box<dyn Error>> {
    // HashMap of
    //  <cluster>: {
    //   <partition>: {
    //       <state>: <count>,
    //   },
    // }
    let mut cluster_partition_states: HashMap<String, HashMap<String, HashMap<String, i64>>> =
        HashMap::new();
    let sinfo = Command::new("sinfo")
        .arg("--noheader")
        .arg("--Format=Cluster,Partition,NodeHost,StateLong")
        .arg(format!("--clusters={}", slurm_cluster))
        .output()?;

    let rc = match sinfo.status.code() {
        Some(v) => v,
        None => {
            bail!("Can't get return code of sinfo command");
        }
    };

    if !sinfo.status.success() {
        bail!("sinfo command exited with non-normal exit code {}", rc);
    }

    let stdout = String::from_utf8(sinfo.stdout)?;
    for line in stdout.lines() {
        let (c, p, _, s) = match split_part_state_line(line) {
            Some(v) => v,
            None => bail!(
                "Can't extract tuple cluster, partition, hostname and state from output '{}'",
                line
            ),
        };
        let cluster = cluster_partition_states
            .entry(c)
            .or_insert_with(HashMap::<String, HashMap<String, i64>>::new);
        let partition = cluster.entry(p).or_insert_with(HashMap::<String, i64>::new);
        *partition.entry(s).or_insert(0) += 1;
    }
    for (clu, cpart) in cluster_partition_states.iter() {
        for (part, pstate) in cpart.iter() {
            for (state, count) in pstate.iter() {
                exporter::PARTITIONS
                    .with_label_values(&[clu, part, state])
                    .set(*count);
            }
        }
    }
    Ok(())
}

fn split_part_state_line(s: &str) -> Option<(String, String, String, String)> {
    let (cluster, remain) = match s.split_once(' ') {
        Some(v) => v,
        None => return None,
    };
    let remain = remain.trim();
    let (partition, remain) = match remain.split_once(' ') {
        Some(v) => v,
        None => return None,
    };
    let remain = remain.trim();
    let (host, remain) = match remain.split_once(' ') {
        Some(v) => v,
        None => return None,
    };
    let state = remain.trim();

    // * marks the default partition
    let mut partition = partition;
    if partition.ends_with('*') {
        partition = match partition.strip_suffix('*') {
            Some(v) => v,
            None => panic!("BUG in slurm.rs:split_part_state_line"),
        };
    };
    Some((
        cluster.to_string(),
        partition.to_string(),
        host.to_string(),
        state.to_string(),
    ))
}

fn split_job_state_line(s: &str) -> Option<(String, String, String, i64, i64, i64)> {
    let (cluster, remain) = match s.split_once(' ') {
        Some(v) => v,
        None => return None,
    };
    let remain = remain.trim();
    let (partition, remain) = match remain.split_once(' ') {
        Some(v) => v,
        None => return None,
    };
    let remain = remain.trim();
    let (state, remain) = match remain.split_once(' ') {
        Some(v) => v,
        None => return None,
    };
    let remain = remain.trim();
    let (_nodes, remain) = match remain.split_once(' ') {
        Some(v) => v,
        None => return None,
    };
    let nodes = match _nodes.parse::<i64>() {
        Ok(v) => v,
        Err(_) => return None,
    };
    let remain = remain.trim();
    let (_tasks, remain) = match remain.split_once(' ') {
        Some(v) => v,
        None => return None,
    };
    let tasks = match _tasks.parse::<i64>() {
        Ok(v) => v,
        Err(_) => return None,
    };

    let _cpus = remain.trim();
    let cpus = match _cpus.parse::<i64>() {
        Ok(v) => v,
        Err(_) => return None,
    };

    Some((
        cluster.to_string(),
        partition.to_string(),
        state.to_string(),
        nodes,
        tasks,
        cpus,
    ))
}
