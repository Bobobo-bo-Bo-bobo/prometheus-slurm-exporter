use crate::exporter;
use std::collections::HashMap;
use std::error::Error;
use std::process::Command;

pub fn update_partition_metrics() -> Result<(), Box<dyn Error>> {
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
        .arg("--clusters=all")
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
