extern crate docopt;
extern crate rustc_serialize;

use docopt::Docopt;
mod ipster;

const USAGE: &'static str = "
Usage: ipster [-p] <orig> <change> (-o <outfile>)
       ipster --help

Options:
    -o, --outfile  Write output to this file
    -p, --patch    Apply patch instead of diffing files.
    -h, --help     Show this message.
";

#[derive(RustcDecodable)]
struct Args {
    arg_orig: String,
    arg_change: String,
    arg_outfile: String,
    flag_patch: bool,
}

fn main () {
    let argv = std::env::args();
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.argv(argv.into_iter()).decode())
        .unwrap_or_else(|e| e.exit());

    let some_bytes = {
        if args.flag_patch {
            ipster::files::patch_files(&args.arg_orig, &args.arg_change)
        } else {
            ipster::files::diff_files(&args.arg_orig, &args.arg_change)
        }
    };
    some_bytes.map(|bytes| ipster::files::write_file(&args.arg_outfile, &bytes));
}
