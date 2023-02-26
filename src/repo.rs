use crate::cprint;
use crate::dir::DIR;
use crate::error::*;
use reqwest::Client;
use termcolor::Color;
use tokio::spawn;
// The web address of texcreate to send requests to
const ADDRESS: &str = "https://texcreate.mkproj.com";
/// Returns the github link to download a template file given a version number and template name
pub fn gh_link(num: u64, name: &str) -> String {
    format!("https://github.com/MKProj/mkproj_texcgen/releases/download/v{num}/{name}.json")
}
/// Returns the repo github link to download `repo.toml` given a version number
pub fn repo_link(num: u64) -> String {
    format!("https://github.com/MKProj/mkproj_texcgen/releases/download/v{num}/repo.toml")
}
/// Sends a request to get the latest mkproj template repo version number
pub async fn get_latest_num() -> u64 {
    let client = Client::new();
    let link = format!("{ADDRESS}/repo/latest");
    let resp = client.get(&link).send().await.unwrap();
    let b = resp.bytes().await.unwrap();
    let s = String::from_utf8(b.to_vec()).unwrap();
    let num = s.trim().parse::<u64>().unwrap();
    num
}
/// Returns a vector of bytes of a template given a version number and template name
async fn get_template_data(num: u64, name: &str) -> Vec<u8> {
    let link = gh_link(num, name);
    let client = Client::new();
    let resp = client.get(&link).send().await.unwrap();
    let bytes = resp.bytes().await.unwrap();
    bytes.to_vec()
}
// Gets the latest repo and saves it
async fn get_latest_repo() -> Result<()> {
    let num = get_latest_num().await;
    let repo_link = repo_link(num);
    DIR.save_repo(&repo_link).await?;
    Ok(())
}
/// Updates the mkprojects directory to the latest release
pub async fn repo_update() -> Result<()> {
    // gets the latest repo in a separate thread
    spawn(async move { get_latest_repo().await })
        .await
        .unwrap()?;
    // clear mkproj directory
    DIR.clear().await?;
    // read the repo so we can get all of the templates name to download
    let repo = DIR.read_repo().await?;
    // get the latest version number
    let num = get_latest_num().await;
    // stores a tuple of `(template name, template bytes join handle)`
    let mut tasks = Vec::new();
    // iterate through `repo` for all template names in the release
    for (name, _) in repo.into_iter() {
        // to use the name after, we will need to create a new owner n that we will use when pushing
        let n = name.clone();
        // create a new task that gets the template data with the version number and template name
        // each task will have type `JoinHandle<Vec<u8>>` so when we join it back to main thread we
        // get the template's bytes
        let task = spawn(async move { get_template_data(num, &name).await });
        // push the name of the template and task to `tasks`
        tasks.push((n, task))
    }

    // iterate through `tasks` so we can save each template to the `mkproj` directory
    for (name, handle) in tasks {
        // get the template's bytes by joining the handle
        let bytes = handle.await.expect("Join handle failed in update!");
        // get the file name by adding the JSON extension
        let file_name = format!("{name}.json");
        // save the template
        DIR.save_mkproj(&file_name, &bytes).await?;
    }
    cprint!(Color::Green, "Successfully updated to repo v{num}!");
    Ok(())
}
/// Displays all mkproject templates and their description
pub async fn repo_display() -> Result<()> {
    // to make sure that we print white, we will make stdout white
    cprint!(Color::White, "\r");
    // read `repo.toml`
    let repo = DIR.read_repo().await?;
    // use the `display()` method to print out the repo information
    repo.display();
    Ok(())
}
/// Checks to see if there is a new repo, if so a message will be returned
pub async fn update_alert() -> Option<String> {
    // to check if there is any new repo
    // we will need to know what the current version is from our `repo.toml`
    let repo = DIR.read_repo().await;
    // handle the `Result` using a match statement
    match repo {
        // if we get a repo, which we should unless TexCreate isn't initialized
        Ok(r) => {
            // to know what version we have we will use the `version()` method and store it in `v`
            let v = r.version();
            // to get the current version, we will use the `get_latest_num()` function that sends a
            // request to get the latest release version.
            let current = get_latest_num().await;
            // we will check if the current version is greater than `v`
            if current > v {
                // The goal of our message is to look like the following:
                // For example if the latest version was v2:
                /*
                   ----------------------------------
                   | Update to the latest repo: v2! |
                   ----------------------------------
                */

                let mut vec = Vec::new();

                let msg = format!("| Update to the latest repo: v{current}! |");
                let pattern = {
                    let mut p = String::new();
                    for _ in 0..msg.trim().len() {
                        p.push('-')
                    }
                    p
                };
                vec.push(pattern.to_string());
                vec.push(msg);
                vec.push(pattern);
                Some(vec.join("\n"))
            }
            // if the current version is the same (can't be less logically), then we have no message
            else {
                None
            }
        }
        // the only error to occur is if TexCreate isn't initialized, if so there is no repo to update
        Err(_) => None,
    }
}
