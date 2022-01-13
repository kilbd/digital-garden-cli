use clap::{Parser, Subcommand};
use color_eyre::eyre::{eyre, Result, WrapErr};
use digital_garden::write;
use directories::UserDirs;
use std::path::PathBuf;

/// A CLI for the growing and curation of a digital garden
///
/// Visit https://www.rustadventure.rs/garden for more!
#[derive(Debug, Parser)]
#[clap(name = "garden")]
struct Cli {
    #[clap(parse(from_os_str), short = 'p', long, env)]
    garden_path: Option<PathBuf>,
    #[clap(subcommand)]
    cmd: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Write something in your garden
    ///
    /// This command will open your $EDITOR, wait for you
    /// to write something, and then save the file to
    /// your garden.
    Write {
        /// Optionally set a title for what you are going to write about
        #[clap(short, long)]
        title: Option<String>,
    },
}

fn get_default_garden_dir() -> Result<PathBuf> {
    let user_dirs = UserDirs::new().ok_or_else(|| eyre!("Could not find home directory"))?;
    Ok(user_dirs.home_dir().join(".garden"))
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();
    let garden_path = match cli.garden_path {
        Some(pathbuf) => Ok(pathbuf),
        None => get_default_garden_dir().wrap_err("`garden_path` was not supplied"),
    }?;
    match cli.cmd {
        Command::Write { title } => write(garden_path, title),
    }
}
