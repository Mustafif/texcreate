use std::path::PathBuf;
use toml::{to_string_pretty, from_str};
use serde::{Deserialize, Serialize};
use texcore::{Input, Level, Metadata};
use tokio::fs::{create_dir, File, read_to_string, remove_file};
use crate::dir::Dir;
use crate::error::{Error, Result};
use crate::cprint;
use termcolor::Color::Cyan;
use std::io::stdin;
use tokio::io::AsyncWriteExt;
use tokio::process::Command;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config{
    project: Project,
    metadata: Metadata,
    packages: Vec<String>
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Project{
    proj_name: String,
    template: String,
    repo: String
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Compiler{
    compiler: String,
    proj_name: String,
    flags: Vec<String>
}

impl Default for Project{
    fn default() -> Self {
        Project::new("Project", "basic", "mkproj")
    }
}

impl Default for Config{
    fn default() -> Self {
        let project = Project::default();
        let metadata = Metadata::default();
        Self{project, metadata, packages: vec![]}
    }
}

impl Project{
    pub fn new(proj_name: &str, template: &str, repo: &str) -> Self{
        Self{
            proj_name: proj_name.to_string(),
            template: template.to_string(),
            repo: repo.to_string()
        }
    }
    pub fn prompt_user() -> Result<Project>{
        let mut input = String::new();
        cprint!(Cyan, "Use default project settings? (yes/no)");
        stdin().read_line(&mut input)?;
        match input.to_lowercase().trim(){
            "no" => {
                let mut input = String::new();
                cprint!(Cyan, "Enter Project Name: ");
                stdin().read_line(&mut input)?;
                let proj_name = input.to_string();
                input.clear();
                cprint!(Cyan, "Enter Template Repo: ");
                stdin().read_line(&mut input)?;
                let repo = input.to_string();
                input.clear();
                cprint!(Cyan, "Enter Template Name: ");
                stdin().read_line(&mut input)?;
                let template = input.to_string();
                Ok(Self::new(proj_name.trim(), template.trim(), repo.trim()))
            },
            "yes" => Ok(Project::default()),
            _ => Err(Error::InvalidInput(input))
        }
    }
    pub fn paths(&self) -> (PathBuf, PathBuf, PathBuf){
        let main_path = PathBuf::from(&self.proj_name);
        let incl_path = main_path.join("include");
        let out_path = main_path.join("out");
        (main_path, incl_path, out_path)
    }
    pub async fn create_layout(&self) -> Result<()>{
        let (main_path, incl_path, out_path) = self.paths();
        let compiler = Compiler::new(&self);
        create_dir(&main_path).await?;
        create_dir(&incl_path).await?;
        create_dir(&out_path).await?;
        compiler.create_file().await?;
        Ok(())
    }
}

impl Config{
    pub fn new() -> Result<Self>{
        let project = Project::prompt_user()?;
        let metadata = Metadata::default();
        Ok(Self{
            project,
            metadata,
            packages: vec![]
        })
    }
    pub async fn from_file(p: PathBuf) -> Result<Self>{
        let s = read_to_string(p).await?;
        let config = from_str(&s).unwrap();
        Ok(config)
    }
    pub fn to_string(&self) -> String{
        to_string_pretty(&self).unwrap()
    }
    pub fn name(&self) -> String{
        self.project.proj_name.to_string()
    }
    fn template(&self) -> String{
        self.project.template.to_string()
    }
    fn repo(&self) -> String{
        self.project.repo.to_string()
    }
    pub async fn build(&self) -> Result<()>{
        self.project.create_layout().await?;
        let (main_path, incl_path, _) = self.project.paths();
        let dir = Dir::new();
        let template = dir.search(&self.template(), &self.repo()).await?;
        template.change_metadata(self.metadata.clone());
        let main_path = main_path.join(&format!("{}.tex", self.name()));
        let incl_path = incl_path.join("structure.tex");
        let str_path = PathBuf::from("include").join("structure");
        let input = Input::new(str_path, Some(Level::Meta));
        template.write_tex_files(main_path, incl_path, input)?;
        Ok(())
    }
}

impl Compiler{
    // Create a new default compiler with pdflatex as compiler
    pub fn new(project: &Project) -> Self{
        Self{
            compiler: "pdflatex".to_string(),
            proj_name: project.proj_name.to_string(),
            flags: vec![],
        }
    }
    // path will always be `compiler.toml`
    pub async fn from_file() -> Result<Self>{
        let s = read_to_string("compiler.toml").await?;
        Ok(from_str(&s).unwrap())
    }
   // pub fn to_string(&self) -> String{ to_string_pretty(&self).unwrap() }
    pub async fn create_file(&self) -> Result<()>{
        let s = to_string_pretty(&self).unwrap();
        let path = PathBuf::from(&self.proj_name).join("compiler.toml");
        let mut file = File::create(path).await?;
        file.write_all(s.as_bytes()).await?;
        Ok(())
    }
    pub async fn compile(&self) -> Result<()>{
        let _ = Command::new(&self.compiler)
            .arg("-output-directory=out")
            .args(&self.flags)
            .arg(&self.proj_name)
            .output()
            .await
            .expect("Couldn't compile LaTeX document");
        // remove aux and log files
        let out = PathBuf::from("out");
        let aux = out.join(&format!("{}.aux", &self.proj_name));
        let log = out.join(&format!("{}.log", &self.proj_name));
        remove_file(aux).await?;
        remove_file(log).await?;
        println!("The project {} successfully compiled!", &self.proj_name);
        Ok(())
    }
}