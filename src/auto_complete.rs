
use std::fs::File;
use std::io::stdin;
use crate::error::{Error, Result};
use crate::{Cli, cprint};
use termcolor::Color::{Magenta, Green};
use structopt::clap::Shell;
use structopt::StructOpt;

pub fn prompt_shell(s: String) -> Result<Shell>{
    let result = match s.to_lowercase().trim(){
        "bash" => Shell::Bash,
        "elvish" => Shell::Elvish,
        "fish" => Shell::Fish,
        "powershell" => Shell::PowerShell,
        "zsh" => Shell::Zsh,
        _ => return Err(Error::InvalidInput(s))
    };
    Ok(result)
}

pub fn get_name(shell: &Shell) -> String{
    let extension: &str = match shell{
        Shell::Bash => "bash",
        Shell::Fish => "fish",
        Shell::Zsh => "zsh",
        Shell::PowerShell => "ps1",
        Shell::Elvish => "elv",
    };
    format!("texcreate-complete.{}", extension)
}

pub fn auto_complete(shell: String) -> Result<()> {
    let shell = prompt_shell(shell)?;
    let name = get_name(&shell);
    let mut file = File::create(&name)?;
    Cli::clap().gen_completions_to( "texcreate", shell, &mut file);
    cprint!(Green, "Successfully generated {}", &name);
    Ok(())
}
