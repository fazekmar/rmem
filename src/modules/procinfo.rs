use super::arguments::Args;
use super::proc;

use rayon::prelude::*;
use std::process;
use std::str::FromStr;

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

pub fn collect(options: &Args) -> Vec<Process> {
    if options.pid_list.is_empty() {
        all_processes()
            .into_par_iter()
            .map(|x| get_process(&options, x))
            .collect::<Vec<Process>>()
    } else {
        options
            .pid_list
            .clone()
            .into_par_iter()
            .map(|x| get_process(&options, proc::Process::new(x)))
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

pub fn all_processes() -> Vec<proc::Process> {
    let pid_list: Vec<u32> = std::fs::read_dir("/proc/")
        .unwrap()
        .into_iter()
        .filter_map(|entry| match entry {
            Ok(e) => {
                // Try to parse int, if failed the entry is probably not a process
                if let Ok(pid) = u32::from_str(&e.file_name().to_string_lossy()) {
                    // Do not include rmem to the list
                    if pid == process::id() {
                        return None;
                    } else {
                        return Some(pid);
                    }
                }
                None
            }
            _ => None,
        })
        .collect();

    pid_list
        .into_par_iter()
        .map(proc::Process::new)
        .collect::<Vec<proc::Process>>()
}

fn get_process(options: &Args, prc: proc::Process) -> Process {
    if prc.cmdline.is_empty() {
        return Process::default();
    }

    let command = if options.full {
        prc.cmdline.join(" ")
    } else {
        prc.get_command()
    };

    let meminfo = if options.rss {
        get_status(&prc)
    } else {
        get_smaps(&prc)
    };

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

fn get_status(prc: &proc::Process) -> MemInfo {
    match prc.get_status() {
        Ok(res) => MemInfo {
            private: res.vmrss,
            ..MemInfo::default()
        },
        _ => MemInfo::default(),
    }
}

fn get_smaps(prc: &proc::Process) -> MemInfo {
    match prc.get_smaps() {
        Ok(res) => MemInfo {
            private: res.private_clean + res.private_dirty,
            shared: res.pss - (res.private_clean + res.private_dirty),
            swap: res.swap,
        },
        _ => get_status(prc),
    }
}
