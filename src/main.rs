use anyhow::bail;
use clap::Parser;
use mdbook_summary_tools::{
    Cli, Command, GenerateArgs, config,
    writeback::{build_summary, diff_summaries, read_summary, summary_output_path, write_summary},
};

fn main() {
    if let Err(error) = run() {
        eprintln!("{error:#}");
        std::process::exit(1);
    }
}

fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let config_path = config::resolve_config_path(cli.book.as_deref(), cli.config.as_deref())?;
    let book_root = config::resolve_book_root(cli.book.as_deref(), &config_path)?;

    match cli
        .command
        .unwrap_or(Command::Generate(GenerateArgs::default()))
    {
        Command::PrintConfig => {
            let resolved = config::load_config(&config_path)?;
            println!("{resolved:#?}");
        }
        Command::Generate(args) => {
            let resolved = config::load_config(&config_path)?;
            let summary = build_summary(&book_root, &resolved)?;

            if args.stdout {
                print!("{summary}");
            } else {
                write_summary(&book_root, &resolved, &summary)?;
            }
        }
        Command::Check(args) => {
            let resolved = config::load_config(&config_path)?;
            let generated = build_summary(&book_root, &resolved)?;
            let summary_path = summary_output_path(&book_root, &resolved);
            let current = read_summary(&summary_path)?;

            if current != generated {
                if args.diff {
                    eprintln!("{}", diff_summaries(&current, &generated));
                }
                bail!("SUMMARY.md is out of date");
            }
        }
    }

    Ok(())
}
