use super::arguments::Args;
use procfs::process::{all_processes, Process as fsProcess};
use rayon::prelude::*;
use std::iter::FromIterator;

#[derive(Default)]
pub struct Process {
    pub name: String,
    pub private: u64,
    pub shared: u64,
    pub swap: u64,
    pub total: u64,
    pub count: u16,
}

impl Process {
    pub fn get_name(&self) -> String {
        if self.count > 1 {
            format!("{} ({})", self.name.to_string(), self.count)
        } else {
            self.name.to_string()
        }
    }
}

#[derive(Default)]
struct MemInfo {
    pub private: u64,
    pub shared: u64,
    pub swap: u64,
}

impl MemInfo {
    pub fn total(&self) -> u64 {
        self.private + self.shared
    }
}

pub fn collect(options: Args) -> Vec<Process> {
    if options.pid_list.is_empty() {
        all_processes()
            .unwrap()
            .into_par_iter()
            .map(|x| get_process(options.clone(), x))
            .collect::<Vec<Process>>()
    } else {
        options
            .pid_list
            .clone()
            .into_par_iter()
            .map(|x| {
                get_process(
                    options.clone(),
                    procfs::process::Process::new(x).unwrap_or_else(|e| {
                        eprintln!("Error: {}, exiting", e);
                        std::process::exit(1)
                    }),
                )
            })
            .collect::<Vec<Process>>()
    }
}

pub fn sort_and_dedup(mut p_vec: Vec<Process>) -> Vec<Process> {
    p_vec.sort_by(|a, b| a.name.cmp(&b.name));
    p_vec.dedup_by(|a, b| {
        if a.name == b.name {
            b.private += a.private;
            b.shared += a.shared;
            b.total += a.total;
            b.swap += a.swap;
            b.count += a.count;
            true
        } else {
            false
        }
    });
    p_vec.sort_by(|a, b| a.total.cmp(&b.total));
    p_vec
}

fn get_process(options: Args, prc: procfs::process::Process) -> Process {
    let cmdline = match prc.cmdline() {
        Ok(cmdline) => cmdline,
        _ => Vec::new(),
    };
    if cmdline.is_empty() {
        return Process::default();
    }

    let command = if options.full {
        String::from_iter(cmdline)
    } else {
        get_command(prc.clone(), cmdline[0].clone())
    };

    // TODO implement smamps
    let meminfo = get_mem_status(prc.clone());

    Process {
        name: if options.pid {
            format!("{} [{}]", command, prc.pid)
        } else {
            command
        },
        private: meminfo.private,
        shared: meminfo.shared,
        swap: meminfo.swap,
        total: meminfo.total(),
        count: 1,
    }
}

fn get_command(prc: fsProcess, cmdline: String) -> String {
    match prc.exe() {
        Ok(process) => {
            let exe = process.display().to_string();
            let is_deleted = exe.contains(" (deleted)");

            let path = exe.split_whitespace().next().unwrap();

            let cmd = path.split('/').last().unwrap().to_string();

            if is_deleted {
                match std::path::Path::new(&path).exists() {
                    true => return format!("{} [updated]", cmd),
                    false => return format!("{} [deleted]", cmd),
                }
            }
            cmd
        }
        _ => cmdline
            .split_whitespace()
            .next()
            .unwrap()
            .split('/')
            .last()
            .unwrap()
            .to_string(),
    }
}

fn get_mem_status(prc: fsProcess) -> MemInfo {
    match prc.status() {
        Ok(res) => MemInfo {
            private: res.vmrss.unwrap(),
            shared: 0,
            swap: 0,
        },
        _ => MemInfo::default(),
    }
}
