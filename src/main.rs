use clap::Parser;
use anyhow::{Context, Result};
use log::{info, warn};
use env_logger;

/// Search for a pattern in a file and display the lines that contain it
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

#[derive(Debug)]
struct CustomError(String);

fn main() -> Result<()> {
    env_logger::init();
    info!("starting up");
    warn!("be careful!");
    let pb = indicatif::ProgressBar::new(100);
    // for i  in 0..100 {
    //     pb.set_position(i);
    //     pb.set_message("Downloading...");
    //     std::thread::sleep(std::time::Duration::from_millis(10));
    //     pb.println(format!("[+] downloaded #{}%", i));
    //     pb.inc(1);
    // }
    // pb.finish_with_message("Download complete!");

    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", &args.path.display()))?;

    grrs::find_matches(&content, &args.pattern, std::io::stdout());
    Ok(())

    // let file = File::open(&args.path).expect("cannot read file");
    // let reader = BufReader::new(file);

    // for line  in reader.lines() {
    //     match line {
    //         Ok(line) => {
    //             if line.contains(&args.pattern) {
    //                 println!("{}", line);
    //             }
    //         }
    //         Err(error) => {
    //             eprintln!("error reading line: {}", error);
    //         }
    //     }
    // }
}

