use std::fmt;
use std::fs::File;
use std::io::{BufReader, Read};

#[derive(Default, Clone, Debug)]
pub struct Smaps {
    // shared_clean: u64,
    // shared_dirty: u64,
    pub private_clean: u64,
    pub private_dirty: u64,
    pub pss: u64,
    pub swap: u64,
}

#[derive(Default, Clone, Debug)]
pub struct Status {
    pub vmrss: u64,
}

#[derive(Default, Clone, Debug)]
pub struct Name {
    name: String,
    status: String,
}

impl fmt::Display for Name {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status = if self.status.is_empty() {
            "".to_string()
        } else {
            format!(" [{}]", self.status)
        };

        write!(fmt, "{}{}", self.name, status)
    }
}

#[derive(Default, Clone, Debug)]
pub struct Process {
    pub pid: u32,
    pub cmdline: Vec<String>,
    pub name: Option<Name>,
}

impl Process {
    pub fn new(pid: u32) -> Self {
        Self {
            pid,
            cmdline: cmdline(pid).unwrap_or_default(),
            name: get_name_status(pid),
        }
    }

    /// Get executed filename or try to extract command from cmdline
    pub fn get_command(&self) -> String {
        match &self.name {
            Some(process) => process.to_string(),
            _ => self.cmdline[0]
                .split_whitespace()
                .next()
                .unwrap_or_default()
                .split('/')
                .last()
                .unwrap_or_default()
                .to_string(),
        }
    }

    /// Get related informations from /proc/[pid]/status
    pub fn get_status(&self) -> Result<Status, std::io::Error> {
        let file = file_reader(format!("/proc/{}/status", self.pid))?;

        let mut status = Status::default();
        for line in file.lines() {
            let mut line_iterator = line.split_whitespace();
            if let Some("VmRSS:") = line_iterator.next() {
                status.vmrss += parse(line_iterator)
            }
        }

        Ok(status)
    }

    /// Get related informations from /proc/[pid]/smaps
    pub fn get_smaps(&self) -> Result<Smaps, std::io::Error> {
        let file = file_reader(format!("/proc/{}/smaps", self.pid))?;

        let mut smap = Smaps::default();
        for line in file.lines() {
            let mut line_iterator = line.split_whitespace();
            match line_iterator.next() {
                // Some("Shared_Clean:") => smap.shared_clean += parse(line_iterator),
                // Some("Shared_Dirty:") => smap.shared_dirty += parse(line_iterator),
                Some("Private_Clean:") => smap.private_clean += parse(line_iterator),
                Some("Private_Dirty:") => smap.private_dirty += parse(line_iterator),
                Some("Pss:") => smap.pss += parse(line_iterator),
                Some("Swap:") => smap.swap += parse(line_iterator),
                _ => {}
            }
        }

        Ok(smap)
    }
}

/// Try to get executed filename, if failed fallback to the cmdline property
pub fn get_name_status(pid: u32) -> Option<Name> {
    match std::fs::read_link(format!("/proc/{}/exe", pid)) {
        Ok(process) => {
            let cmd = process.file_name().unwrap_or_default().to_string_lossy();
            if cmd.contains(" (deleted)") {
                let cmd = cmd.split_whitespace().next().unwrap_or_default();

                let mut process = process.clone();
                process.set_file_name(cmd);
                let status = match process.exists() {
                    true => "updated".to_string(),
                    false => "deleted".to_string(),
                };

                return Some(Name {
                    name: cmd.to_string(),
                    status,
                });
            }
            Some(Name {
                name: cmd.to_string(),
                ..Name::default()
            })
        }
        _ => None,
    }
}

/// Get `interator`'s next element and try parse to `u64`
fn parse(mut iterator: std::str::SplitWhitespace) -> u64 {
    iterator
        .next()
        .unwrap_or_default()
        .parse::<u64>()
        .unwrap_or_default()
}

/// Parse `/proc/[pid]/cmdline`
fn cmdline(pid: u32) -> Result<Vec<String>, std::io::Error> {
    let file = file_reader(format!("/proc/{}/cmdline", pid))?;
    let file = file
        .split('\0')
        .filter_map(|arg| {
            if arg.is_empty() {
                return None;
            }
            Some(arg.to_string())
        })
        .collect();

    Ok(file)
}

/// Helper function to read one file
fn file_reader(path: String) -> Result<String, std::io::Error> {
    let raw_file = File::open(path)?;
    let mut buf_reader = BufReader::new(raw_file);
    let mut file = String::new();
    buf_reader.read_to_string(&mut file)?;

    Ok(file)
}
