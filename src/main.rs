mod config;
mod dir;
mod error;
mod repo;
mod texc_gen;
mod web;

use crate::config::{Compiler, Config};
use crate::texc_gen::Commands;
use dir::Dir;
use error::*;
use repo::*;
use std::io::stdin;
use std::path::PathBuf;
use structopt::StructOpt;
use termcolor::Color;
use tokio::fs::{remove_file, File};
use tokio::io::AsyncWriteExt;
use tokio::spawn;

#[macro_export]
macro_rules! cprint {
    ($color: expr, $($arg: tt)*) => ({
        use std::io::Write;
        use termcolor::{ColorChoice, ColorSpec, StandardStream, WriteColor};
        let mut stdout = StandardStream::stdout(ColorChoice::Auto);
        let _ = stdout.set_color(ColorSpec::new().set_fg(Some($color)));
        let _ = writeln!(&mut stdout, $($arg)*);
    });
}

#[derive(StructOpt)]
#[structopt(name = "TexCreate", about = "A LaTeX Project Creator by Mustafif Khan")]
enum CLI {
    #[structopt(about = "Initialize TexCreate.")]
    Init,
    #[structopt(about = "Create a new project's config file.")]
    New,
    #[structopt(about = "Build a project using a config file.")]
    Build {
        #[structopt(short, long, parse(from_os_str))]
        file: Option<PathBuf>,
    },
    #[structopt(about = "Zip a project using a config file.")]
    Zip {
        #[structopt(short, long, parse(from_os_str))]
        file: Option<PathBuf>,
    },
    #[structopt(about = "Updates to the latest MKProject templates.")]
    Update,
    #[structopt(about = "Shows all available MKProject templates.")]
    List {
        #[structopt(short, long)]
        repo: Option<String>,
    },
    #[structopt(about = "Compiles a TexCreate project.")]
    Compile,
    #[structopt(about = "Runs a TexcGen Project.")]
    Texcgen(Commands),
    #[structopt(about = "Opens up `texcreate.mkproj.com` on default browser.")]
    Open,
    #[structopt(about = "Runs a web service to create TexCreate projects.")]
    Web,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = CLI::from_args();
    match cli {
        CLI::Init => {
            // initializes the texcreate directory
            // and gets the latest repo
            let dir = Dir::new();
            // creates the layout
            cprint!(
                Color::Magenta,
                "Creating TeXCreate directory layout at: {}",
                dir.main_dir.display()
            );
            let layout_task = spawn(async move { dir.build().await }).await.ok();
            match layout_task {
                None => {
                    cprint!(Color::Red, "Failed to build TexCreate directory!");
                    return Ok(());
                }
                Some(r) => r?,
            }
            // gets the latest repo for mkproj
            let update_task = spawn(async move { repo_update().await }).await.ok();
            match update_task {
                None => {
                    cprint!(Color::Red, "Failed to update to latest repo!");
                    return Ok(());
                }
                Some(r) => r?,
            }
        }
        CLI::New => {
            alert().await;
            // for the moment creates default config
            let config = Config::new()?;
            let s = config.to_string();
            let mut file_name = String::new();
            cprint!(
                Color::Yellow,
                "Enter config file name (default: texcreate.toml): "
            );
            stdin().read_line(&mut file_name)?;
            let file_name = {
                let file_name = file_name.trim();
                if file_name.is_empty() {
                    "texcreate.toml"
                } else {
                    file_name
                }
            };
            let mut file = File::create(file_name).await?;
            file.write_all(s.as_bytes()).await?;
            cprint!(Color::Green, "Successfully created `{}`", file_name);
        }
        CLI::Build { file } => {
            alert().await;
            // read config
            let path = file.unwrap_or(PathBuf::from("texcreate.toml"));
            let config = Config::from_file(path).await?;
            let name = config.name();
            let task = spawn(async move { config.build().await }).await.ok();
            match task {
                None => {
                    cprint!(Color::Red, "Failed to build project!");
                    return Ok(());
                }
                Some(r) => r?,
            }
            cprint!(Color::Green, "Successfully created `{}`", name);
        }
        CLI::Zip { file } => {
            alert().await;
            // read config
            let path = file.unwrap_or(PathBuf::from("texcreate.toml"));
            let config = Config::from_file(path).await?;
            let task = spawn(async move { config.zip().await }).await.ok();
            let name = match task {
                None => {
                    cprint!(Color::Red, "Failed to zip project!");
                    return Ok(());
                }
                Some(s) => s?,
            };
            cprint!(Color::Green, "Successfully created `{}`", name);
        }
        CLI::Update => {
            // updates to the latest repo
            repo_update().await?;
        }
        CLI::List { repo } => match repo {
            None => mkproj_repo_list().await?,
            Some(repo) => match repo.as_str() {
                "custom" => {
                    let dir = Dir::new();
                    dir.read_custom_repo().await?;
                }
                _ => mkproj_repo_list().await?,
            },
        },
        CLI::Compile => {
            let compiler = Compiler::from_file().await?;
            compiler.compile().await?;
        }
        CLI::Texcgen(c) => {
            c.run_command().await;
        }
        CLI::Open => {
            open::that("https://texcreate.mkproj.com")?;
        }
        CLI::Web => {
            let _ = web::web().launch().await.unwrap();
            remove_file("index.html").await?;
        }
    }
    Ok(())
}

async fn mkproj_repo_list() -> Result<()> {
    alert().await;
    repo_display().await?;
    Ok(())
}

async fn alert() {
    if let Some(msg) = update_alert().await { cprint!(Color::Red, "{}", msg) }
}
