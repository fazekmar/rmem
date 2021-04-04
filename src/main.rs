mod modules;
use modules::*;

fn main() {
    let options: arguments::Args = arguments::get_args();

    let process_vec = procinfo::collect(&options);

    if options.total {
        let total = process_vec.iter().map(|p| p.total).sum();
        println!("{}", printer::format(options.si, total));
        return;
    }

    let (mut ram_total, mut swap_total) = (0, 0);
    printer::header(&options);
    for process in procinfo::sort_and_dedup(process_vec) {
        ram_total += process.total;
        swap_total += process.swap;
        printer::process(process, &options);
    }
    printer::footer(&options, ram_total, swap_total);
}
