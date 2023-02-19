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
            Commands::Gen { .. } => {
                "gen".to_string()
            }
            Commands::GenAll { .. } => {
                "gen-all".to_string()
            }
            Commands::Save { .. } => {
                "save".to_string()
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
                .args(&self.arguments())
                .spawn()
                .unwrap();
            }
        }
    }
    fn arguments(&self) -> Vec<String>{
        let mut vec = Vec::new();
        vec.push("run".to_string());
        vec.push("--".to_string());
        match &self{
            Commands::New => {
                unimplemented!("not implemented for new")
            }
            Commands::Init => {
                vec.push("init".to_string())
            }
            Commands::Gen { level } => {
                let level = level.unwrap_or(1);
                vec.push("gen".to_string());
                vec.push("-l".to_string());
                vec.push(level.to_string());
            }
            Commands::GenAll { level } => {
                let level = level.unwrap_or(1);
                vec.push("gen-all".to_string());
                vec.push("-l".to_string());
                vec.push(level.to_string());
            }
            Commands::Save { name } => {
                vec.push("save".to_string());
                vec.push("-n".to_string());
                vec.push(name.to_string());
            }
        }
        vec
    }
}

