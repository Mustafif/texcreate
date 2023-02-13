const ADDRESS: &str = "https://texcreate.mkproj.com";
use reqwest::Client;
use crate::error::*;
use tokio::spawn;
use crate::dir::Dir;
use crate::cprint;
use termcolor::Color;

fn gh_link(num: u64, name: &str) -> String{
    format!("https://github.com/MKProj/mkproj_texcgen/releases/download/v{num}/{name}.json")
}

fn repo_link(num: u64) -> String{
    format!("https://github.com/MKProj/mkproj_texcgen/releases/download/v{num}/repo.toml")
}

async fn get_latest_num() -> u64{
    let client = Client::new();
    let link = format!("{ADDRESS}/repo/latest");
    let resp = client.get(&link).send().await.unwrap();
    let s = resp.text().await.unwrap();
    let num: u64= s.trim().parse().unwrap();
    num
}

async fn get_template_data(num: u64, name: &str) -> Vec<u8>{
    let link = gh_link(num, name);
    let client = Client::new();
    let resp = client.get(&link).send().await.unwrap();
    let bytes = resp.bytes().await.unwrap();
    bytes.to_vec()
}

async fn get_latest_repo() -> Result<()>{
    let dir = Dir::new();
    let num = get_latest_num().await;
    let repo_link = repo_link(num);
    dir.save_repo(&repo_link).await?;
    Ok(())
}

pub async fn repo_update() -> Result<()>{
    // gets the latest repo in a separate thread
    spawn(async move {
        get_latest_repo().await
    }).await.unwrap()?;
    let dir = Dir::new();
    // clear mkproj directory
    dir.clear().await?;
    let repo = dir.read_repo().await?;
    let num = get_latest_num().await;
    let mut tasks = Vec::new();
    for (name, _) in repo.into_iter(){
        let n = name.clone();
        let task = spawn(async move {
            get_template_data(num, &name).await
        });
        tasks.push((n, task))
    }
    for (name, handle) in tasks{
        let bytes = handle.await.unwrap();
        let file_name = format!("{name}.json");
        dir.save_mkproj(&file_name, &bytes).await?;
    }
    cprint!(Color::Green, "Successfully updated to repo v{num}!");
    Ok(())
}

pub async fn repo_display()  -> Result<()>{
    let dir = Dir::new();
    let repo = dir.read_repo().await?;
    repo.display();
    Ok(())
}

pub async fn update_alert() -> Option<String>{
    let dir = Dir::new();
    let repo = dir.read_repo().await.ok();
    match repo{
        Some(r) => {
            let v = r.version();
            let current = get_latest_num().await;
            if current > v{
                let msg = format!("Update to the latest repo: v{current}...");
                Some(msg)
            } else {
                None
            }
        }
        None => return None
    }
}
