extern crate docopt;
extern crate rustc_serialize;

use docopt::Docopt;
mod ipster;

const USAGE: &'static str = "
Usage: ipster -s <patchfile>
       ipster [-p] <orig> <change> <outfile>
       ipster --help

Options:
    -p, --patch  Apply patch
    -s, --show  Decode a patch file
    -h, --help  Show this message.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_patchfile: Option<String>,
    flag_patch: bool,
    arg_orig: Option<String>,
    arg_change: Option<String>,
    arg_outfile: Option<String>,
}

fn main () {
    let argv = std::env::args();
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.argv(argv.into_iter()).decode())
        .unwrap_or_else(|e| e.exit());

    match args {
        Args { arg_orig: Some(arg_orig), arg_change: Some(arg_change), arg_outfile: Some(arg_outfile), flag_patch, arg_patchfile: None } => {
            let some_bytes = {
                if flag_patch {
                    ipster::files::patch_files(&arg_orig, &arg_change)
                } else {
                    ipster::files::diff_files(&arg_orig, &arg_change)
                }
            };
            some_bytes.map(|bytes| ipster::files::write_file(&arg_outfile, &bytes));
        },
        Args { arg_orig: None, arg_change: None, arg_outfile: None, flag_patch: false, arg_patchfile: Some(arg_patchfile) } => {
            ipster::files::with_file(&arg_patchfile, |bytes| {
                let patches = ipster::unserialize_patches(bytes);
                println!("{:#?}", patches);
                Some(())
            });
        }
        _ => {
            println!("Invalid combination of arguments\n{:#?}", args);
            std::process::exit(1);
        }
    }

}
