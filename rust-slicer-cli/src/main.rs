use anyhow::{Context, Result};
use clap::clap_app;
use log::debug;
use rust_slicer::{Config, Range, config::EvalMode};
use serde::Serialize;
use generate_rustc_flags::generate_rustc_flags;

#[derive(Serialize)]
struct SliceOutput {
  ranges: Vec<Range>,
}

fn run() -> Result<()> {
  let _ = env_logger::try_init();
  
  let matches = clap_app!(app =>
    (@arg debug: -d)
    (@arg likec: -l)
    (@arg path:)
    (@arg start_line:)
    (@arg start_col:)
    (@arg end_line:)
    (@arg end_col:)
  )
  .get_matches();

  macro_rules! arg {
    ($key:expr) => {
      matches.value_of($key).context($key)?
    };
  }

  let flags = generate_rustc_flags(arg!("path"))?;
  debug!("Generated rustc command:\n{}", flags.join(" "));

  let config = Config {
    range: Range {
      start_line: arg!("start_line").parse::<usize>()?,
      start_col: arg!("start_col").parse::<usize>()?,
      end_line: arg!("end_line").parse::<usize>()?,
      end_col: arg!("end_col").parse::<usize>()?,
      filename: arg!("path").to_owned(),
    },
    debug: matches.is_present("debug"),
    eval_mode: if matches.is_present("likec") { EvalMode::LikeC } else { EvalMode::Standard },
  };

  let output = rust_slicer::slice(config, &flags)?;
  let cli_output = SliceOutput {
    ranges: output.ranges().clone(),
  };
  println!("{}", serde_json::to_string(&cli_output)?);

  Ok(())
}

fn main() {
  run().unwrap();
}