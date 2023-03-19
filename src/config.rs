use crate::cprint;
use crate::dir::DIR;
use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use std::io::{stdin, Write};
use std::path::PathBuf;
use termcolor::Color::Cyan;
use texc_v3_compiler_conf::*;
use texcore::{Any, Element, Input, Level, Metadata, Package};
use tokio::fs::{create_dir, read_to_string};
use toml::{from_str, to_string_pretty};
use zip::write::FileOptions;
use zip::{CompressionMethod, ZipWriter};

/// The configuration used to create TexCreate projects
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    // handles fields related to building the project
    project: Project,
    // uses `texcore::Metadata` to change the default metadata in a template
    metadata: Metadata,
    // extra packages to put in a template
    packages: Vec<String>,
}

// The default for Config, used when the user would like to use default settings
impl Default for Config {
    fn default() -> Self {
        let project = Project::default();
        let metadata = Metadata::default();
        Self {
            project,
            metadata,
            packages: vec![],
        }
    }
}

/// The Project section provides the user to declare the following:
/// - Project Name (Used for main directory, main source file, and `compiler.toml`)
/// - The Template Name (Used to build the project using a particular template)
/// - The Repo Name (Used to search which repo to find the template)
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Project {
    proj_name: String,
    template: String,
    repo: String,
}

// The default for Project, used when the user would like to use default settings
impl Default for Project {
    fn default() -> Self {
        Project::new("Project", "basic", "mkproj")
    }
}

impl Project {
    /// Creates a new project using a project, template and repo name
    pub fn new(proj_name: &str, template: &str, repo: &str) -> Self {
        Self {
            proj_name: proj_name.to_string(),
            template: template.to_string(),
            repo: repo.to_string(),
        }
    }
    /// Prompts the user for Project settings
    pub fn prompt_user() -> Result<Project> {
        // a mutable string buffer
        let mut input = String::new();
        // ask the user if they would like to use default settings
        cprint!(Cyan, "Use default project settings? (yes/no)");
        stdin().read_line(&mut input)?;
        match input.to_lowercase().trim() {
            // We will need to get the fields from them
            "no" => {
                // when asking these questions we will need to read the buffer
                // create a binding and use `input`, to avoid cloning we will use `.to_string()`
                // after to prompt the next field, we will need to clear the buffer and do the
                // same steps again
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
            }
            // We can use default settings
            "yes" => Ok(Project::default()),
            // anything other than yes or no will return an invalid input error
            _ => Err(Error::InvalidInput(input)),
        }
    }
    /// Returns the paths for the layout of a TexCreate Project.
    ///
    /// When a project is built, it is meant to have the following layout:
    ///
    /// ```
    /// Project/
    ///     compiler.toml
    ///     include/
    ///       structure.tex
    ///     Project.tex
    ///     out/
    ///       Project.pdf
    /// ```
    pub fn paths(&self) -> (PathBuf, PathBuf, PathBuf) {
        let main_path = PathBuf::from(&self.proj_name);
        let incl_path = main_path.join("include");
        let out_path = main_path.join("out");
        (main_path, incl_path, out_path)
    }
    /// Creates the layout of a project
    pub async fn create_layout(&self) -> Result<()> {
        // get the main, include and out path from the `paths()` method
        let (main_path, incl_path, out_path) = self.paths();
        // create a new `Compiler` type using the project name
        let compiler = Compiler::new(&self.proj_name);
        // first create the main project path
        create_dir(&main_path).await?;
        // create the include and out directories
        create_dir(&incl_path).await?;
        create_dir(&out_path).await?;
        // create `compiler.toml`
        compiler.create_file().await?;
        Ok(())
    }
}

impl Config {
    /// Create a new `Config` by prompting the user
    pub fn new() -> Result<Self> {
        // Create new `Project` using the method `prompt_user()`
        let project = Project::prompt_user()?;
        // Use default `Metadata`
        let metadata = Metadata::default();
        // return new `Config` wrapped in `Ok()` since the function returns a `Result`
        Ok(Self {
            project,
            metadata,
            packages: vec![],
        })
    }
    /// Creates a new `Config` by reading a file
    pub async fn from_file(p: PathBuf) -> Result<Self> {
        let s = read_to_string(p).await?;
        let config = from_str(&s).unwrap();
        Ok(config)
    }
    /// Returns `Config` as a TOML string
    pub fn to_string(&self) -> String {
        to_string_pretty(&self).unwrap()
    }
    /// Returns the project name
    pub fn name(&self) -> String {
        self.project.proj_name.to_string()
    }
    // Returns the template name
    fn template(&self) -> String {
        self.project.template.to_string()
    }
    // Returns the repo name
    fn repo(&self) -> String {
        self.project.repo.to_string()
    }
    // Returns a vector of `Element<Any>` from self.packages
    fn packages(&self) -> Vec<Element<Any>> {
        let mut packages = Vec::new();
        for i in &self.packages {
            let p = Package::new(i);
            packages.push(Element::from(p))
        }
        packages
    }
    /// Builds a TexCreate project
    pub async fn build(&self) -> Result<()> {
        // Create the project layout
        self.project.create_layout().await?;
        // Get the main and include directory path
        let (main_path, incl_path, _) = self.project.paths();
        // Get the template by using the template and repo names
        let mut template = DIR.search(&self.template(), &self.repo()).await?;
        // Change the template's metadata to config's
        template.change_metadata(self.metadata.clone());
        // Push the array of packages from config
        template.push_element_array(self.packages()).await;
        // Path for the main source file
        let main_path = main_path.join(format!("{}.tex", self.name()));
        // Path to write the structure file
        let incl_path = incl_path.join("structure.tex");
        // Path for the source file's `\input{}` entry
        let str_path = PathBuf::from("include").join("structure");
        // Create a new input for our source file
        let input = Input::new(str_path, Level::Meta);
        // Write the tex files using the main and include path, as well as using `input`
        template
            .write_tex_files(main_path, incl_path, input)
            .await?;
        Ok(())
    }
    /// Zips a TexCreate Project
    pub async fn zip(&self) -> Result<String> {
        // need to use standard library `fs::File` since `ZipWriter` doesn't support `tokio::fs::File`
        use std::fs;
        let zip_name = format!("{}.zip", &self.name());
        // create a new zip writer using an inner value of `File` which contains `{proj_name}.zip`
        let mut writer = ZipWriter::new(fs::File::create(&zip_name)?);
        // We will use default file options to the compressed files and use the Stored method
        let option = FileOptions::default().compression_method(CompressionMethod::Stored);
        // add out and include directory
        writer
            .add_directory("out", option)
            .expect("Couldn't add out directory");
        writer
            .add_directory("include", option)
            .expect("Couldn't add include directory");
        // create the paths for the main and structure file
        let main_path = format!("{}.tex", &self.name());
        let str_path = PathBuf::from("include").join("structure.tex");
        // get the template by using the template and repo name
        let mut template = DIR.search(&self.template(), &self.repo()).await?;
        // change the metadata to the config's
        template.change_metadata(self.metadata.clone());
        // Push the array of packages from the config
        template.push_element_array(self.packages()).await;
        // Create an input by cloning `str_path` since it's just `include/structure.tex`
        let input = Input::new(str_path.clone(), Level::Meta);
        // get the split latex strings using input
        let (main_data, str_data) = template.to_latex_split_string(input).await;
        // create a new compiler config using the project name
        let compiler = Compiler::new(&self.name());
        // get the compiler toml string
        // need to assign it to a binding or we create a temporary value then borrow it
        // if we directly use compiler.to_string().as_bytes()
        let compiler_data = compiler.to_string();

        // start the main file using the file options
        writer
            .start_file(&main_path, option)
            .expect("Couldn't start main file");
        // write to the main file converting the data into bytes
        writer
            .write_all(main_data.as_bytes())
            .expect("Couldn't write to main file");

        // start the structure file using the file options
        writer
            .start_file(str_path.to_str().unwrap(), option)
            .expect("Couldn't start structure.tex");
        // write to the structure file converting the data into bytes
        writer
            .write_all(str_data.as_bytes())
            .expect("Couldn't write to structure.tex");

        // start on `compiler.toml`
        writer
            .start_file("compiler.toml", option)
            .expect("Couldn't start compiler.toml");
        // write to `compiler.toml` converting the data into bytes
        writer
            .write_all(compiler_data.as_bytes())
            .expect("Couldn't write to compiler.toml");
        // finish the last file and drop the writer using the `finish()` method
        // we will ignore the returned `File`, recommended not to append anymore data to it
        let _ = writer.finish().unwrap();
        // return the zip file name
        Ok(zip_name)
    }
}
