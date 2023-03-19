mod index;

use crate::error::Result;
use crate::repo::{get_latest_num, gh_link, repo_link};
use reqwest::Client;
use rocket::form::Form;
use rocket::fs::NamedFile;
use rocket::{get, post, routes, Build, Config, FromForm, Rocket};
use std::path::PathBuf;
use texcore::template::Template;
use texcore::{Input, Level, Metadata};
use texcreate_repo::Repo;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::spawn;
use zip::write::FileOptions;
use zip::{CompressionMethod, ZipWriter};
use texc_v3_compiler_conf::Compiler;
/// A web version of the config that is simplified and only
/// uses mkproj templates
#[derive(Debug, Clone, FromForm)]
pub struct WebConfig {
    // Project fields
    proj_name: String,
    template: String,
    // metadata fields
    author: String,
    date: String,
    title: String,
    fontsize: u8,
    papersize: String,
    doc_class: String,
}

impl WebConfig {
    // sends a request to get a template's data
    async fn get_template(&self) -> Template {
        // get the current version number
        let curr_num = get_latest_num().await;
        // get the name of the template
        // since we are spawning a thread, we will need to clone self, then access template
        let name = self.clone().template;
        // spawn a task that returns a `Template`
        spawn(async move {
            // create client
            let client = Client::new();
            // send a get request and get the response
            let resp = client.get(gh_link(curr_num, &name)).send().await.unwrap();
            // turn the response into a stream of bytes
            let bytes = resp.bytes().await.unwrap();
            // get a UTF8 string using the bytes as Vec<u8>
            let s = String::from_utf8(bytes.to_vec()).unwrap();
            // return a template using the `from_string()` method
            Template::from_string(&s)
        })
        .await
        .expect("Failed to retrieve Template")
    }
    // returns `Metadata` using the config's metadata fields
    // used when zipping the project
    fn metadata(&self) -> Metadata {
        Metadata::new(
            &self.author,
            &self.date,
            &self.title,
            self.fontsize,
            &self.papersize,
            &self.doc_class,
            true,
        )
    }
    pub async fn zip(&self) -> Result<String> {
        use std::fs;
        use std::io::Write;
        let zip_name = format!("{}.zip", &self.proj_name);
        // create zip writer
        let mut writer = ZipWriter::new(fs::File::create(&zip_name)?);
        // set compression method to stored
        let option = FileOptions::default().compression_method(CompressionMethod::Stored);
        // create directories in zip
        writer.add_directory("out", option).unwrap();
        writer.add_directory("include", option).unwrap();

        let main_path = format!("{}.tex", &self.proj_name);
        let str_path = PathBuf::from("include").join("structure.tex");
        // get template and change metadata
        let mut template = self.get_template().await;
        template.change_metadata(self.metadata());
        // create input for template
        let input = Input::new(str_path.clone(), Level::Meta);
        // split template into main and structure latex strings
        let (main_data, str_data) = template.to_latex_split_string(input).await;
        // create compiler
        let compiler = Compiler::new(&self.proj_name);
        // convert compiler to string
        let compiler_data = compiler.to_string();

        // write to main.tex in zip
        writer
            .start_file(&main_path, option)
            .expect("Couldn't start main file");
        writer
            .write_all(main_data.as_bytes())
            .expect("Couldn't write to main file");

        // write to structure.tex in zip
        writer
            .start_file(str_path.to_str().unwrap(), option)
            .expect("Couldn't start structure.tex");
        writer
            .write_all(str_data.as_bytes())
            .expect("Couldn't write to structure.tex");

        // write compiler.toml in zip
        writer
            .start_file("compiler.toml", option)
            .expect("Couldn't start compiler.toml");
        writer
            .write_all(compiler_data.as_bytes())
            .expect("Couldn't write to compiler.toml");
        // finish writing to zip
        let _ = writer.finish().unwrap();
        // return the zip name
        Ok(zip_name)
    }
}
/// Returns the latest repo by sending a request to the repo link with the latest version number
pub async fn read_repo(n: u64) -> Repo {
    let client = Client::new();
    let resp = client.get(&repo_link(n)).send().await.unwrap();
    let bytes = resp.bytes().await.unwrap();
    let s = String::from_utf8(bytes.to_vec()).unwrap();
    Repo::from_string(&s)
}
/// To ensure to display latest mkproj templates as `<option></option>`
pub async fn template_html_options() -> String {
    let curr_num = get_latest_num().await;
    let repo = read_repo(curr_num).await;
    let mut vec = Vec::new();
    for (name, _) in repo.into_iter() {
        let s = format!("<option value='{}'>{}</option>", &name, &name);
        vec.push(s)
    }
    vec.join("\n")
}
/// Builds the index.html file
pub async fn build_index() -> Result<()> {
    let mut file = File::create("index.html").await?;
    let mut index = index::INDEX.to_string();
    index = index.replace("{templates}", &template_html_options().await);
    file.write_all(index.as_bytes()).await?;
    Ok(())
}

#[get("/")]
async fn texc_index() -> Option<NamedFile> {
    match build_index().await {
        Ok(()) => (),
        Err(_) => return None,
    }
    NamedFile::open("index.html").await.ok()
}

#[post("/", data = "<input>")]
async fn texc_post(input: Form<WebConfig>) -> Option<NamedFile> {
    let input = input.into_inner();
    let file_name = match input.zip().await {
        Ok(s) => s,
        Err(_) => return None,
    };
    NamedFile::open(&file_name).await.ok()
}
/// Builds the rocket instance
pub fn web() -> Rocket<Build> {
    // create a figment with the debug configuration
    let config = Config::figment()
        .merge(("cli_color", true))
        .merge(("port", 8000))
        .merge(("log_level", "debug"))
        .merge(("keep_alive", 5));
    // build the rocket instance
    rocket::build()
        .configure(config)
        .mount("/", routes![texc_index, texc_post])
}
