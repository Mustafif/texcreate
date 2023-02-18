mod repo;
mod dir;
mod error;
mod config;

use std::io::stdin;
use std::path::PathBuf;
use dir::Dir;
use repo::*;
use error::*;
use structopt::StructOpt;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use crate::config::{Compiler, Config};
use termcolor::Color;

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
#[structopt(
name = "TexCreate",
about = "A LaTeX Project Creator by Mustafif Khan"
)]
enum CLI{
    #[structopt(about = "Initialize TexCreate.")]
    Init,
    #[structopt(about = "Create a new project's config file.")]
    New,
    #[structopt(about = "Build a project using a config file.")]
    Build{
        #[structopt(short, long, parse(from_os_str))]
        file: Option<PathBuf>
    },
    #[structopt(about = "Updates to the latest MKProject templates.")]
    Update,
    #[structopt(about = "Shows all available MKProject templates.")]
    List,
    #[structopt(about = "Compiles a TexCreate Project")]
    Compile
}


#[tokio::main]
async fn main() -> Result<()>{
    let cli = CLI::from_args();
    match cli{
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
            dir.build().await?;
            // gets the latest repo for mkproj
            repo_update().await?;
        }
        CLI::New => {
            // for the moment creates default config
            let config = Config::new()?;
            let s = config.to_string();
            let mut file_name = String::new();
            cprint!(Color::Yellow, "Enter config file name (default: texcreate.toml): ");
            stdin().read_line(&mut file_name)?;
            let file_name = {
                let file_name = file_name.trim();
                if file_name.is_empty(){
                    "texcreate.toml"
                } else {
                    file_name
                }
            };
            let mut file = File::create(file_name).await?;
            file.write_all(s.as_bytes()).await?;
            cprint!(Color::Green, "Successfully created `{}`", file_name);
            match update_alert().await{
                Some(msg) => cprint!(Color::Red, "{}", msg),
                None => ()
            }
        }
        CLI::Build{file} => {
            // read config
            let path = file.unwrap_or(PathBuf::from("texcreate.toml"));
            let config = Config::from_file(path).await?;
            config.build().await?;
            cprint!(Color::Green, "Successfully created `{}`", config.name());
            match update_alert().await{
                Some(msg) => cprint!(Color::Red, "{}", msg),
                None => ()
            }
        }
        CLI::Update => {
            // updates to the latest repo
            repo_update().await?;
        }
        CLI::List => {
            // displays all mkproj repos
            // TODO!: Add flag to choose either mkproj or custom repo
            repo_display().await?;
            match update_alert().await{
                Some(msg) => cprint!(Color::Red, "{}", msg),
                None => ()
            }
        }
        CLI::Compile => {
            let compiler = Compiler::from_file().await?;
            compiler.compile().await?;
        }
    }
    Ok(())
}
