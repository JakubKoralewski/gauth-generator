use google_authenticator::{ErrorCorrectionLevel, GoogleAuthenticator};
use quicli::prelude::*;
use std::{fs::File, process::exit};
use std::io::Write;
use std::path::PathBuf;
use structopt::StructOpt;

/// https://stackoverflow.com/questions/54687403/how-can-i-use-enums-in-structopt-in-rust
fn parse_error_correction_level(ecl: &str) -> anyhow::Result<ErrorCorrectionLevel> {
    match ecl.to_lowercase().as_str() {
        "l" | "low" | "0" => Ok(ErrorCorrectionLevel::Low),
        "m" | "medium" | "1" => Ok(ErrorCorrectionLevel::Medium),
        "q" | "quartile" | "2" => Ok(ErrorCorrectionLevel::Quartile),
        "h" | "high" | "3" => Ok(ErrorCorrectionLevel::High),
        _ => Err(anyhow::anyhow!(
            "Invalid error correction level. Choose one from [low, medium, quartile, high]."
        )),
    }
}

#[derive(StructOpt)]
struct Cli {
    #[structopt(subcommand)]
    cmd: Option<SubCommand>,

    #[structopt(
        long = "length",
        short = "l",
        default_value = "32",
        help = "Length of secret"
    )]
    length: u8,

    #[structopt(
        long = "ht",
        help="Hide UNIX timestamp"
    )]
    hide_timestamp: bool,

    /// Whether to generate the QR Code
    ///
    /// QR Code will be written as SVG to `qrcode.svg`. Won't overwrite.
    #[structopt(long = "qr", short = "q")]
    qrcode: bool,

    // #[structopt(long = "qr-as-path", short="p", help="Whether to return QR Code as path", default_value=)]
    // qrcode: bool,
    /// QR Code name. For TOTP Url. Will display in app.
    ///
    /// Eg. "your company name". Requires --qr.
    #[structopt(
        long = "qr-name",
        short = "n",
        // requires("qrcode"),
        default_value = "gauth-generator"
    )]
    qr_name: String,

    /// QR Code title. For TOTP url. Will display in app.
    ///
    /// E.g. "hello". Requires --qr.
    #[structopt(
        long = "qr-title",
        short = "t",
        // requires("qrcode"),
        default_value = "cli"
    )]
    qr_title: String,

    /// QR Code width.
    ///
    /// Minimum is 200px. Requires --qr.
    #[structopt(
        long = "qr-width",
        short = "w",
        // requires("qrcode"),
        default_value = "200"
    )]
    qr_width: u32,

    /// QR Code height.
    ///
    /// Minimum is 200px. Requires --qr.
    #[structopt(
        long = "qr-height",
        short = "h",
        // requires("qrcode"),
        default_value = "200"
    )]
    qr_height: u32,

    /// Error correction level. 
    ///
    /// Requires --qr.
    /// "l" | "low" | "0" => Low  
    ///
    /// "m" | "medium" | "1" => Medium
    ///
    /// "q" | "quartile" | "2" => Quartile
    ///
    /// "h" | "high" | "3" => High
    ///
    /// _ => Err
    ///
    #[structopt(
        long = "ecl",
        short="e",
        parse(try_from_str = parse_error_correction_level),
        default_value="l",
        // requires("qrcode")
    )]
    qr_ecl: ErrorCorrectionLevel,

}

#[derive(StructOpt)]
enum SubCommand {
    Validate { 
        #[structopt(help="The secret")]
        secret: String,

        #[structopt(help="The code that you want to test")]
        code: String,

        #[structopt(
            help="The +/- error in seconds of error you accept",
            short="d",
            long="discrepancy",
            default_value="0"
        )]
        discrepancy: u64,

        #[structopt(help="UNIX time at which code was generated. Leave empty for current time.", default_value="0", short="t", long="time-slice")]
        time_slice: u64
    },
}

const DEFAULT_SVG_NAME: &'static str = "qrcode";

fn main() -> CliResult {
    let args = Cli::from_args();

    let auth = GoogleAuthenticator::new();

    match args.cmd {
        Some(cmd) => match cmd {
            SubCommand::Validate { 
                code, secret,
                discrepancy, time_slice 
            } => {
                let rv = auth.verify_code(&secret, &code, discrepancy, time_slice);
                if rv {
                    println!("OK")
                } else {
                    println!("Invalid!");
                    exit(1)
                }
            }
        },
        None => {
            let secret = auth.create_secret(args.length);
            println!("{}", secret);
            if args.qrcode {
                let svg = auth
                    .qr_code(
                        &secret,
                        &args.qr_name,
                        &args.qr_title,
                        args.qr_width,
                        args.qr_height,
                        args.qr_ecl,
                    )
                    .expect("the provided data for generating SVG to be correct");

                let mut path = PathBuf::from(format!("./{}.svg", DEFAULT_SVG_NAME));
                let mut i = 0;
                while path.exists() {
                    path = path.with_file_name(format!("{}{}.svg", DEFAULT_SVG_NAME, i));
                    i += 1;
                }
                let mut file = File::create(path.clone())?;
                file.write(svg.as_bytes())
                    .expect("qr svg to be written to fs");
                let path_str = path.to_str().expect("path to be correct");
                println!("{}", path_str);
            }
        }
    }

    Ok(())
}
