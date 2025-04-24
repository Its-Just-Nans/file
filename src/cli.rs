use clap::Parser;

use crate::Output;

/// Command line interface for the `whatisthis` library
/// This CLI allows you to process files and get information about their format.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// List of files to process
    file: Vec<String>,

    /// Output in JSON format
    #[arg(short, long)]
    json: bool,

    /// Suppress error messages
    #[arg(short, long)]
    no_error: bool,

    /// Show human readable size
    #[arg(long)]
    bytes: bool,

    /// Use IEC size
    #[arg(long)]
    iec: bool,
}

fn process_path(path: &str) -> Result<Output, String> {
    // check if file or directory
    let file_path = std::path::Path::new(path);
    if file_path.is_dir() {
        return Err(format!("{} - Directory", file_path.display()));
    }
    let data = match std::fs::read(file_path) {
        Ok(data) => data,
        Err(err) => {
            return Err(format!(
                "Error: {} is not readable - {}",
                file_path.display(),
                err
            ));
        }
    };
    let infos = crate::process_file_raw(&data, path);
    Ok(infos)
}

pub fn cli_main() -> i32 {
    let args = Args::parse();
    let mut code = 0;
    // println!("{:?}", args);
    if args.json {
        let mut res_vec = Vec::new();
        for one_file in args.file {
            match process_path(&one_file) {
                Ok(res) => {
                    res_vec.push(res);
                }
                Err(_err) => {
                    code = 3;
                }
            }
        }
        match serde_json::to_string_pretty(&res_vec) {
            Ok(json) => println!("{}", json),
            Err(_) => {
                println!("Error serializing output");
                return 1;
            }
        }
    } else {
        for one_file in args.file {
            match process_path(&one_file) {
                Ok(res) => {
                    println!(
                        "{} - {} - '{}' - {}",
                        one_file,
                        res.file_type.format_name,
                        res.file_type.media_type,
                        crate::get_size(res.file_len, !args.bytes, args.iec),
                    );
                }
                Err(err) => {
                    code = 3;
                    if !args.no_error {
                        println!("{}", err);
                    }
                }
            }
        }
    }
    code
}
