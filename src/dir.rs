// Creates the directory to save all templates

use std::path::PathBuf;
use dirs::home_dir;
use texcore::template::Template;
use tokio::io::AsyncWriteExt;
use tokio::fs::{create_dir, File, read_to_string, remove_dir_all};
use texcreate_repo::Repo;
use walkdir::WalkDir;
use crate::error::*;


pub struct Dir{
    pub main_dir: PathBuf,
    pub mkproj: PathBuf,
    pub custom: PathBuf
}

impl Dir{
    pub fn new() -> Self{
        let main_dir = home_dir().unwrap().join(".texcreate");
        let mkproj = main_dir.join("mkproj");
        let custom = main_dir.join("custom");
        Self{main_dir, mkproj, custom}
    }
    pub async fn build(&self) -> Result<()> {
        create_dir(&self.main_dir).await?;
        create_dir(&self.mkproj).await?;
        create_dir(&self.custom).await?;
        Ok(())
    }
    pub async fn save_mkproj(&self, file_name: &str, data: &[u8]) -> Result<()>{
        let path  = self.mkproj.join(file_name);
        let mut file = File::create(&path).await?;
        file.write_all(data).await?;
        Ok(())
    }
    pub async fn search(&self, name: &str, repo: &str) -> Result<Template>{
        let file_name = format!("{name}.json");
        let path = {
            match repo{
                "custom" => self.custom.join(file_name),
                _ => self.mkproj.join(file_name),
            }
        };
        if !path.exists(){
            return Err(Error::InvalidTemplate(name.to_string()))
        }
        let template = Template::from_file(path)?;
        Ok(template)
    }
    pub async fn read_repo(&self) -> Result<Repo>{
        let path = self.main_dir.join("repo.toml");
        let s = read_to_string(&path).await?;
        let repo = Repo::from_string(&s);
        Ok(repo)
    }
    pub async fn read_custom_repo(&self) -> Result<()>{
        for entry in WalkDir::new(&self.custom){
            let entry = entry.unwrap();
            if entry.path().is_dir(){
                continue
            }
            println!("{}", entry.file_name().to_str().unwrap())
        }
        Ok(())
    }
    pub async fn save_repo(&self, url: &str) -> Result<()>{
        let repo = Repo::get_repo(url).await;
        let s = repo.to_string();
        let path = self.main_dir.join("repo.toml");
        let mut file = File::create(&path).await?;
        file.write_all(s.as_bytes()).await?;
        Ok(())
    }
    // Clears the mkproj repo directory
    pub async fn clear(&self) -> Result<()>{
        remove_dir_all(&self.mkproj).await?;
        create_dir(&self.mkproj).await?;
        Ok(())
    }
}