use std::path::PathBuf;
use structopt::StructOpt;
use tokio::process::Command;
use crate::cprint;
use termcolor::Color::Green;
// the gh link to clone the `texcgen` project
const LINK: &str = "https://github.com/MKProj/texcgen.git";

/// All commands from the `texcgen` project with the inclusion of the `new` command
/// This can be found [here](https://github.com/MKProj/texcgen/blob/main/src/main.rs)
#[derive(StructOpt)]
pub enum Commands {
    #[structopt(about = "Create a new TexcGen Project.")]
    New,
    #[structopt(about = "Initialize Output Directory Structure")]
    Init,
    #[structopt(about = "Refreshes `src/template.rs` using the default template.")]
    Refresh,
    #[structopt(about = "Generate template")]
    Gen {
        #[structopt(short, long)]
        level: Option<u8>,
    },
    #[structopt(about = "Generate all templates in `src/generated`")]
    GenAll {
        #[structopt(short, long)]
        level: Option<u8>,
    },
    #[structopt(about = "Saves template to TexCreate custom directory")]
    Save {
        #[structopt(short, long)]
        name: String,
    },
}

impl ToString for Commands {
    fn to_string(&self) -> String {
        match self {
            Commands::Init => "init".to_string(),
            Commands::Refresh => "refresh".to_string(), 
            Commands::Gen { .. } => "gen".to_string(),
            Commands::GenAll { .. } => "gen-all".to_string(),
            Commands::Save { .. } => "save".to_string(),
            _ => {
                unimplemented!("This command is unimplemented")
            }
        }
    }
}

impl Commands {
    /// Runs the command depending on the variant
    pub async fn run_command(&self) {
        match self {
            // the new command will clone the repository in the root directory it's in
            Commands::New => {
                let _ = git2::Repository::clone(LINK, PathBuf::from("texcgen"))
                    .expect("Couldn't clone repository");
                cprint!(Green, "Successfully created TexCGen!");
            }
            // any other command will be ran, using the `args()` method to iterate through an array of arguments
            _ => {
                let _ = Command::new("cargo")
                    .args(&self.arguments())
                    .spawn()
                    .unwrap();
            }
        }
    }
    // Used to define the arguments for the given command
    fn arguments(&self) -> Vec<String> {
        let mut vec = Vec::new();
        // we will be running cargo run -- {command} {flags}
        vec.push("run".to_string());
        vec.push("--".to_string());
        vec.push(self.to_string());
        match &self {
            Commands::Gen { level } => {
                // if no level is defined, we will default to 1
                let level = level.unwrap_or(1);
                // push -l <level>
                vec.push("-l".to_string());
                vec.push(level.to_string());
            }
            Commands::GenAll { level } => {
                // same as the gen command ^
                let level = level.unwrap_or(1);
                vec.push("-l".to_string());
                vec.push(level.to_string());
            }
            Commands::Save { name } => {
                // push -n <name>
                vec.push("-n".to_string());
                vec.push(name.to_string());
            }
            // should be unreachable
            // new command is handled in `run_command()`
            // init command doesn't have any flags
            _ => {}
        }
        vec
    }
}
