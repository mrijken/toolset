use std::process::Command;
use std::process::ExitCode;
use std::{fmt, io::Write};

use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Copy, Clone)]
pub enum ExitStatus {
    Success,
    Failure,
}

impl From<ExitStatus> for ExitCode {
    fn from(status: ExitStatus) -> Self {
        match status {
            ExitStatus::Success => ExitCode::from(0),
            ExitStatus::Failure => ExitCode::from(1),
        }
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
    // #[clap(flatten)]
    // verbose: clap_verbosity_flag::Verbosity,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Uninstall a tool
    Uninstall(UinstallArgs),
    /// Install a tool
    Install(InstallArgs),
    /// List all backend
    Backends,

    /// List all tools
    List(ListArgs),
}

#[derive(Args, Debug)]
struct ListArgs {
    /// Name of installer
    #[arg(value_enum)]
    backend: Option<Backend>,
}

#[derive(Args, Debug)]
struct UinstallArgs {
    /// Name of installer
    #[arg(value_enum)]
    backend: Backend,

    /// Name of tool
    #[arg(required = true)]
    name: String,
}

#[derive(Args, Debug)]
struct InstallArgs {
    /// Name of installer
    #[arg(value_enum)]
    backend: Backend,

    /// Name of tool
    #[arg(required = true)]
    name: String,

    /// Version of tool
    #[arg()]
    version: Option<String>,
}

#[derive(Copy, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Backend {
    // Use pipx
    Pipx,
    // Use uvx
    Uvx,
    // Use npx
    Npx,
    // Cargo
    Cargo,
}

impl fmt::Display for Backend {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            Backend::Pipx => "pipx",
            Backend::Uvx => "uvx",
            Backend::Npx => "npx",
            Backend::Cargo => "cargo",
        };
        write!(f, "{}", printable)
    }
}

pub fn cli() -> ExitCode {
    let cli = Cli::parse();
    match cli.command {
        Commands::Uninstall(args) => {
            println!("Uninstalling {} from {}", args.name, args.backend);
            dbg!(Command::new("uv")
                .arg("tool")
                .arg("uninstall")
                .arg(args.name)
                .output()
                .unwrap());
        }
        Commands::Install(args) => {
            println!("Installing {} from {}", args.name, args.backend);
            dbg!(Command::new("uv")
                .arg("tool")
                .arg("install")
                .arg(args.name)
                .output()
                .unwrap());
        }
        Commands::List(args) => {
            println!("Listing");
        }
        Commands::Backends => {
            println!("Listing backends");
            let uv_found = Command::new("uv").output().is_ok();
            dbg!(uv_found);
            let pipx_found = Command::new("pipx").output().is_ok();
            dbg!(pipx_found);
            let npx_found = Command::new("npx").output().is_ok();
            dbg!(npx_found);
        }
    }
    // env_logger::Builder::new()
    //     .filter_level(cli.verbose.log_level_filter())
    //     .format(|buf, record| writeln!(buf, "{}", record.args()))
    //     .init();
    ExitCode::from(ExitStatus::Success)
}
