use std::path::PathBuf;
use structopt::StructOpt;
use tokio::process::Command;

const LINK: &str = "https://github.com/MKProj/texcgen.git";

#[derive(StructOpt)]
pub enum Commands{
    #[structopt(about = "Create a new TexcGen Project.")]
    New,
    #[structopt(about = "Initialize Output Directory Structure")]
    Init,
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

impl ToString for Commands{
    fn to_string(&self) -> String {
        match self{
            Commands::Init => {
                "init".to_string()
            }
            Commands::Gen { level} => {
                match level{
                    Some(l) => format!("gen -l {l}"),
                    None => "gen".to_string()
                }
            }
            Commands::GenAll { level } => {
                match level {
                    Some(l) => format!("gen-all -l {l}"),
                    None => "gen-all".to_string()
                }
            }
            Commands::Save { name } => {
                format!("save -n {name}")
            }
            _ => {
                unimplemented!("This command is unimplemented")
            }
        }
    }
}

impl Commands{
    pub async fn run_command(&self) {
        match self{
            Commands::New => {
                let _ = git2::Repository::clone(LINK, PathBuf::from(".")).expect("Couldn't clone repository");
            }
            _ => {
                let _ = Command::new("cargo")
                .args(["run", "--", &self.to_string()])
                .spawn()
                .unwrap();
            }
        }

    }
}

