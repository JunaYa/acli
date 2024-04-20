use acli::{process_csv, Opts, SubCommand};
use clap::Parser;

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(csv_opts) => {
            let outout = if let Some(output) = csv_opts.output {
                output.clone()
            } else {
                format!("output.{}", csv_opts.format)
            };
            process_csv(&csv_opts.input, outout, csv_opts.format)?;
        }
    }
    Ok(())
}
