mod config;
mod dir;
mod error;
mod repo;
mod texc_gen;
mod auto_complete;

use crate::config::Config;
use crate::texc_gen::Commands;
use dir::Dir;
use error::*;
use repo::*;
use std::io::stdin;
use std::path::PathBuf;
use structopt::StructOpt;
use termcolor::Color;
use texc_v3_compiler_conf::Compiler;
use tokio::fs::{remove_file, File};
use tokio::io::AsyncWriteExt;
use tokio::spawn;
use texc_v3_web::web;
use tokio::process::Command;
use crate::auto_complete::auto_complete;

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
pub enum Cli {
    #[structopt(about = "Initialize TexCreate.")]
    Init,
    #[structopt(about = "Create a new project's config file.")]
    New{
        #[structopt(short, long)]
        ignore: Option<bool>
    },
    #[structopt(about = "Build a project using a config file.")]
    Build {
        #[structopt(short, long, parse(from_os_str))]
        file: Option<PathBuf>,
        #[structopt(short, long)]
        ignore: Option<bool>
    },
    #[structopt(about = "Zip a project using a config file.")]
    Zip {
        #[structopt(short, long, parse(from_os_str))]
        file: Option<PathBuf>,
        #[structopt(short, long)]
        ignore: Option<bool>
    },
    #[structopt(about = "Updates to the latest MKProject templates.")]
    Update,
    #[structopt(about = "Updates TexCreate (`cargo`) and templates to the latest version.")]
    Upgrade,
    #[structopt(about = "Shows all available templates (default MKProj).")]
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
    #[structopt(about = "Generate an autocomplete script for TexCreate.")]
    GenComplete{
        #[structopt(short, long)]
        shell: String
    },
    #[structopt(about = "Send feedback on email to the TexCreate project!")]
    Feedback{
        #[structopt(short, long)]
        subject: String
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::from_args();
    match cli {
        Cli::Init => {
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
        Cli::New{ignore} => {
            // checks to see if there is a new template
            if ignore == Some(false) || ignore.is_none(){
                alert().await;
            }
            // prompts the user to create a new config
            let config = Config::new()?;
            // get the TOML string
            let s = config.to_string();
            let mut file_name = String::new();
            // prompt the user for a file name for the config file
            cprint!(
                Color::Yellow,
                "Enter config file name (default: texcreate.toml): "
            );
            stdin().read_line(&mut file_name)?;
            // check if the file name has something or is empty, if so it will
            // default to `texcreate.toml`.
            let file_name = {
                let file_name = file_name.trim();
                if file_name.is_empty() {
                    "texcreate.toml"
                } else {
                    file_name
                }
            };
            // create the configuration file in the current path
            let mut file = File::create(file_name).await?;
            // write the TOML string as bytes to the file
            file.write_all(s.as_bytes()).await?;
            // let the user know the project has successfully been created
            cprint!(Color::Green, "Successfully created `{}`", file_name);
        }
        Cli::Build { file, ignore } => {
            // checks to see if there is a new template
            if ignore == Some(false) || ignore.is_none(){
                alert().await;
            }
            // read config
            let path = file.unwrap_or(PathBuf::from("texcreate.toml"));
            // get `Config` by reading from the file's path
            let config = Config::from_file(path).await?;
            // get the name of the project
            let name = config.name();
            // build the project in a separate thread
            let task = spawn(async move { config.build().await }).await.ok();
            // handle the task's error
            match task {
                None => {
                    cprint!(Color::Red, "Failed to build project!");
                    return Ok(());
                }
                Some(r) => r?,
            }
            cprint!(Color::Green, "Successfully created `{}`", name);
        }
        Cli::Zip { file, ignore } => {
            // checks to see if there is a new template
            if ignore == Some(false) || ignore.is_none(){
                alert().await;
            }
            // get the config path
            let path = file.unwrap_or(PathBuf::from("texcreate.toml"));
            // get `Config` by reading from the file's path
            let config = Config::from_file(path).await?;
            // zip the project in a separate thread
            let task = spawn(async move { config.zip().await }).await.ok();
            // handle the error of the task and get the zip file name
            let name = match task {
                None => {
                    cprint!(Color::Red, "Failed to zip project!");
                    return Ok(());
                }
                Some(s) => s?,
            };
            cprint!(Color::Green, "Successfully created `{}`", name);
        }
        Cli::Update => {
            // updates to the latest repo
            repo_update().await?;
        }
        Cli::Upgrade => {
            cprint!(Color::Yellow, "Updating TexCreate...");
            // update TexCreate
            let _ = Command::new("cargo")
                        .args(["install", "texcreate"])
                        .output()
                        .await?;

            cprint!(Color::Green, "Successfully updated TexCreate!");
            cprint!(Color::Yellow, "Checking for new templates...");
            // if there is an available update for the template, then we will update
            // if not then we are done
            if update_alert().await.is_some(){
                repo_update().await?;
            }
            cprint!(Color::Green, "Done!");
        }
        Cli::List { repo } => match repo {
            // the default is to list out mkproj templates
            None => mkproj_repo_list().await?,
            // if the repo is custom we will list the custom templates
            // if not we will do mkproj templates
            Some(repo) => match repo.as_str() {
                "custom" => {
                    let dir = Dir::new();
                    dir.read_custom_repo().await?;
                }
                _ => mkproj_repo_list().await?,
            },
        },
        Cli::Compile => {
            // get the compiler from the config file
            let compiler = Compiler::from_file().await?;
            // compile the project using the appropriate compiler
            compiler.compile().await?;
        }
        Cli::Texcgen(c) => {
            // run the given texcgen command
            c.run_command().await;
        }
        Cli::Open => {
            // open the homepage for TexCreate
            open::that("https://texcreate.mkproj.com")?;
        }
        Cli::Web => {
            // launch the web application for TexCreate
            let _ = web().launch().await.unwrap();
            // remove the html file when it closes
            remove_file("index.html").await?;
        }
        Cli::GenComplete{shell} => {
            auto_complete(shell)?
        }
        Cli::Feedback{subject} => {
            let url = format!("mailto:texcreate_feedback@mkproj.com?subject={subject}");
            open::that(url)?
        }
    }
    Ok(())
}

// A helper function to list out mkproj templates
async fn mkproj_repo_list() -> Result<()> {
    alert().await;
    repo_display().await?;
    Ok(())
}
// a helper function to alert a new template
async fn alert() {
    if let Some(msg) = update_alert().await {
        cprint!(Color::Red, "{}", msg)
    }
}
