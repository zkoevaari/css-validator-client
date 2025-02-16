/*
    Created by Zoltan Kovari, 2025.

    Licensed under the Apache License, Version 2.0
    http://www.apache.org/licenses/LICENSE-2.0
    (see LICENSE.txt)
*/

use std::error::Error;
use std::io::{Read, Write};
use std::path::PathBuf;

use clap::Parser;
use percent_encoding::{NON_ALPHANUMERIC, utf8_percent_encode};

/// CLI client for the W3C CSS validator API.
///
/// Important: Please operate the tool responsibly, with respectful usage of
/// shared resources. If you wish to call the validator programmatically for a
/// batch of documents, please make sure that your script will sleep for at least
/// 1 second between requests.
#[derive(Debug, Parser)]
#[command(
    after_long_help(
        "Created by Zoltan Kovari, 2025. Licensed under the Apache License, Version 2.0",
    ),
    verbatim_doc_comment,
    version,
)]
struct Args {
    /// Input file, or use '-' to read from STDIN (max len: 8KB encoded)
    #[arg(long_help = "Input file, or use '-' to read from STDIN\n
Maximum supported percent-encoded length (which is to be used in the
request URL) is 8KB. See the \"--length\" option to help if you need
to slice your input.")]
    file: PathBuf,

    /// Print the calculated length of input, then exit
    ///
    /// This can help with the 8KB payload size limitation, which seem to be
    /// related to the percent-encoded length.
    #[arg(short, long, verbatim_doc_comment)]
    length: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let content = if args.file == std::ffi::OsStr::new("-") {
        let mut buf = String::new();
        std::io::stdin().read_to_string(&mut buf)?;
        buf
    } else {
        match args.file.try_exists() {
            Err(e) => return Err(format!("could not check input file: {}", e.kind()).into()),
            Ok(false) => return Err("input file does not exist".into()),
            Ok(true) if !args.file.is_file() => {
                return Err("input file is not a regular file".into());
            }
            _ => {
                let len = std::fs::metadata(&args.file)?.len();
                if len > 8192 {
                    return Err(format!("input file is too large ({} bytes)", len).into());
                } else {
                    std::fs::read_to_string(args.file)?
                }
            }
        }
    };

    let encoded = utf8_percent_encode(&content, NON_ALPHANUMERIC).to_string();
    if args.length {
        writeln!(std::io::stderr(), "Input length: {} bytes", content.len())?;
        writeln!(std::io::stderr(), "Encoded length: {} bytes", encoded.len())?;
        return Ok(());
    }

    let mut ub = url_builder::URLBuilder::new();
    ub.set_protocol("https")
        .set_host("jigsaw.w3.org")
        .add_route("css-validator/validator")
        .add_param("output", "text")
        .add_param("text", &encoded);
    let url = ub.build();

    let res = ureq::get(url)
        .config()
        .http_status_as_error(false)
        .build()
        .call()?;
    if res.status() == ureq::http::status::StatusCode::OK {
        let body = res.into_body().read_to_string()?;
        writeln!(std::io::stdout(), "{}", body)?;
    } else {
        writeln!(std::io::stderr(), "Server responded with status: {}", res.status())?;
    }

    Ok(())
}
