use clap::Parser;
use rs_rawzip2entry1st::stdin2slice2archive2entry1st_raw2stdout;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The maximum number of bytes of the zip archive to read from stdin
    #[arg(short, long = "max-zip-bytes", default_value_t = 1048576)]
    max_zip_bytes: u64,
}

fn main() {
    let args = Args::parse();
    if let Err(e) = stdin2slice2archive2entry1st_raw2stdout(args.max_zip_bytes) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
