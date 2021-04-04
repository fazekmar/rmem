use super::printer::usage;
use getargs::{Error, Opt, Options, Result};

#[derive(Default, Clone, Debug)]
pub struct Args {
    pub total: bool,
    pub rss: bool,
    pub full: bool,
    pub pid: bool,
    pub pid_list: Vec<u32>,
    pub si: bool,
    pub swap: bool,
}

pub fn get_args() -> Args {
    let args: Vec<_> = std::env::args().skip(1).collect();
    let opts = Options::new(&args);
    match parse_args(&opts) {
        Ok(o) => o,
        Err(e) => {
            eprintln!("usage error: {}", e);
            usage();
            std::process::exit(1);
        }
    }
}

fn parse_args<'a>(opts: &'a Options<'a, String>) -> Result<Args> {
    let mut res = Args::default();
    while let Some(opt) = opts.next() {
        match opt? {
            Opt::Short('h') | Opt::Long("help") => usage(),
            Opt::Short('t') | Opt::Long("total") => res.total = true,
            Opt::Short('r') | Opt::Long("rss") => res.rss = true,
            Opt::Short('f') | Opt::Long("full-args") => res.full = true,
            Opt::Short('s') | Opt::Long("si") => res.si = true,
            Opt::Short('p') | Opt::Long("pid") => res.pid = true,
            Opt::Short('S') | Opt::Long("swap") => res.swap = true,
            Opt::Short('P') | Opt::Long("pid-list") => {
                res.pid_list = opts
                    .value_str()?
                    .split(',')
                    .filter_map(|x| x.parse::<u32>().ok())
                    .collect::<Vec<u32>>()
            }
            opt => return Err(Error::UnknownOpt(opt)),
        }
    }
    Ok(res)
}
