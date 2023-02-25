// Creates the directory to save all templates

use crate::error::*;
use dirs::home_dir;
use lazy_static::lazy_static;
use std::path::PathBuf;
use texcore::template::Template;
use texcreate_repo::Repo;
use tokio::fs::{create_dir, read_to_string, remove_dir_all, File};
use tokio::io::AsyncWriteExt;
use walkdir::WalkDir;

lazy_static! {
    // A global Dir value that will be lazily evaluated
    // Reason to use DIR is to avoid creating a new Dir in every function
    // Dir doesn't change values with all methods immutably borrowing it, hence a
    // lazily static value will be more efficient
    pub static ref DIR: Dir = Dir::new();
}

/// A type to navigate the `.texcreate` directory
pub struct Dir {
    /// The main directory will be located in `$HOME/.texcreate`
    pub main_dir: PathBuf,
    /// The mkproject repo directory will be located `main_dir/mkproj`
    pub mkproj: PathBuf,
    /// The custom repo directory will be located in `main_dir/custom`
    pub custom: PathBuf,
}

impl Dir {
    pub fn new() -> Self {
        // Use the function `home_dir()` to get `$HOME` value
        // If there is no `$HOME` path then we cannot create the directory and we should panic
        // Which makes `unwrap()` call valid to use, if the path exists then we will join it with `.texcreate`
        let main_dir = home_dir().unwrap().join(".texcreate");
        // With the main path we can join it with `mkproj` to get the mkproject repo directory path
        let mkproj = main_dir.join("mkproj");
        // We can do the same as above but for the `custom` repo directory
        let custom = main_dir.join("custom");
        Self {
            main_dir,
            mkproj,
            custom,
        }
    }
    /// Create the layout of `.texcreate`
    pub async fn build(&self) -> Result<()> {
        // Begin by creating the main directory
        create_dir(&self.main_dir).await?;
        // After we can build the directory for `mkproj` and `custom` directory respectively
        create_dir(&self.mkproj).await?;
        create_dir(&self.custom).await?;
        Ok(())
    }
    /// Saves an mkproject template given the filename and its data
    pub async fn save_mkproj(&self, file_name: &str, data: &[u8]) -> Result<()> {
        // To create the proper path, we will join the filename to the `mkproj` directory path
        let path = self.mkproj.join(file_name);
        // With the proper path, we can create the template
        let mut file = File::create(&path).await?;
        // after we can write to the file with the given bytes
        file.write_all(data).await?;
        Ok(())
    }
    /// Searches for a template given a name and repository to look in, and will return a `Template`
    pub async fn search(&self, name: &str, repo: &str) -> Result<Template> {
        // to get the filename we will need to add the JSON extension
        let file_name = format!("{name}.json");
        // to get the proper path we will use a match statement on the parameter, `repo`
        // if the repo isn't `custom` or `mkproj`, then we have an invalid repo and we will return an error
        let path = {
            match repo {
                "custom" => self.custom.join(file_name),
                "mkproj" => self.mkproj.join(file_name),
                _ => return Err(Error::InvalidRepo(repo.to_string())),
            }
        };
        // if the path doesn't exist, then we have an invalid template and we will return an error
        if !path.exists() {
            return Err(Error::InvalidTemplate(name.to_string()));
        }
        // the template should exist after these checks and we can use the method `Template_from_file()`
        // to get the value `Template`
        let template = Template::from_file(path)?;
        // return the template back wrapped in `Ok` since the function returns `Result<Template>`
        Ok(template)
    }
    /// Reads from `main_dir/repo.toml` and returns Repo
    pub async fn read_repo(&self) -> Result<Repo> {
        // get the proper path by joining `main_dir`
        let path = self.main_dir.join("repo.toml");
        // read the file using `path`
        let s = read_to_string(&path).await?;
        // get the `Repo` value using the `from_string()` method
        let repo = Repo::from_string(&s);
        Ok(repo)
    }
    /// Walks trough the `custom` directory and prints out the templates in it
    pub async fn read_custom_repo(&self) -> Result<()> {
        for entry in WalkDir::new(&self.custom) {
            let entry = entry.unwrap();
            if entry.path().is_dir() {
                continue;
            }
            println!("{}", entry.file_name().to_str().unwrap())
        }
        Ok(())
    }
    /// Saves `repo.toml` given a url to send a get request to
    pub async fn save_repo(&self, url: &str) -> Result<()> {
        let repo = Repo::get_repo(url).await;
        let s = repo.to_string();
        let path = self.main_dir.join("repo.toml");
        let mut file = File::create(&path).await?;
        file.write_all(s.as_bytes()).await?;
        Ok(())
    }
    /// Clears the `mkproj` repo directory, used when updating to latest repo
    pub async fn clear(&self) -> Result<()> {
        remove_dir_all(&self.mkproj).await?;
        create_dir(&self.mkproj).await?;
        Ok(())
    }
}
