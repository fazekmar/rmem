use super::arguments::Args;
use super::procinfo::Process;
use bytesize::{ByteSize, KB};

pub fn format(si: bool, mem: u64) -> String {
    ByteSize(mem * KB).to_string_as(si)
}

pub fn usage() {
    println!(
        "
Usage: rmem [OPTIONS]...

Options:
    -t, --total            Show only the total usage
    -r, --rss              Show only the VmRSS usage (less precision but faster)
    -S, --swap             Show swap information (no effect with -r/-t)
    -f  --full-args        Show all command line arguments
    -p  --pid              Show process pid
    -s  --si               Use SI units
    -P  <pid>[,...pidN]    Show memory usage of pids, comma separated list
    -h, --help             Print this help
    "
    );
    std::process::exit(0);
}

pub fn header(options: &Args) {
    if options.rss {
        println!("{: >9}    Program\n", "Total");
    } else if options.swap {
        println!(
            "{0: >9} + {1: >9} = {2: >9} {3: >10}    Program\n",
            "Private", "Shared", "Total", "Swap"
        )
    } else {
        println!(
            "{0: >9} + {1: >9} = {2: >9}    Program\n",
            "Private", "Shared", "Total"
        )
    }
}

pub fn process(process_data: Process, options: &Args) {
    if process_data.name.is_empty() {
        return;
    }
    if options.rss {
        println!(
            "{0: >9}    {1}",
            format(options.si, process_data.total),
            process_data.get_name()
        );
    } else if options.swap {
        println!(
            "{0: >9} + {1: >9} = {2: >9} {3: >10}    {4}",
            format(options.si, process_data.private),
            format(options.si, process_data.shared),
            format(options.si, process_data.total),
            format(options.si, process_data.swap),
            process_data.get_name()
        );
    } else {
        println!(
            "{0: >9} + {1: >9} = {2: >9}    {3}",
            format(options.si, process_data.private),
            format(options.si, process_data.shared),
            format(options.si, process_data.total),
            process_data.get_name()
        );
    }
}

pub fn footer(options: &Args, ram: u64, swap: u64) {
    if options.rss {
        println!("{0:->9}\n{1: >9}\n{0:=>9}", "", format(options.si, ram));
    } else if options.swap {
        println!(
            "{0:->44}\n{1: >33} {2: >10}\n{0:=>44}",
            "",
            format(options.si, ram),
            format(options.si, swap)
        );
    } else {
        println!("{0:->33}\n{1: >33}\n{0:=>33}", "", format(options.si, ram));
    }
}
