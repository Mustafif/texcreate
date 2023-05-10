# TexCreate by Mustafif Khan

![Crates.io](https://img.shields.io/crates/v/texcreate)
![Lines of code](https://img.shields.io/tokei/lines/github/MKProj/texcreate)
![Crates.io](https://img.shields.io/crates/d/texcreate)
![GitHub top language](https://img.shields.io/github/languages/top/MKProj/texcreate)

[!["Buy Me A Coffee"](https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png)](https://www.buymeacoffee.com/mustafif09Q)


> This project is under the [MIT License](LICENSE)

TexCreate is a LaTeX Project creator that allows users to easily create projects using a configuration file that will 
define a template, project name, and metadata. Compared to version 2 which last saw an update 10 months ago, version 3 comes with a focus on template modularity and stability. 

## Template Modularity? 
In previous versions the templates are in someway embedded in the programs code, such as in version 2 we saw that being done 
using `texc_latex` crate. However, this limited the amount of templates that could be available to users as they would have to 
wait for a minor update to see a template addition. So...how was this fixed? This was solved in 2 parts, first was by creating 
an opensource template generator `texcgen` (note: TexCreate templates are built with a fork `mkproj_texcgen`), and the second part 
is done in a web service `texcweb`. The ability to put a template into a JSON file using the handy `texcore` library I developed 
allows it to be easily downloaded in the latest releases and TexCreate is able to know about these updates because of `texcweb`. 

> If you are updating from an earlier beta (before `Beta 9`) to stable, please make sure to update to the latest template repo (currently v2). 
> Due to the update of TexCore `0.6.0` from `0.4.17`, earlier templates are broken. To update to the latest 
> template use the command `texcreate update`. 

## How to get started?

### Installing

#### Cargo Method 

```bash
# make sure to have rust installed 
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# install via Cargo 
$ cargo install texcreate
```

#### Debian Method 

```bash
# this method is only for Debian amd64 Linux systems
# will automatically install autocomplete 
$ curl --proto '=https' https://texcreate.mkproj.com/sh | sudo sh
```

### Setting up Autocomplete 
This example will show how to do auto complete for the bash shell: 

```bash
$ apt-get install bash-completion -y
$ texcreate gen-complete --shell bash
$ mv texcreate.bash /usr/share/bash-completion/completions/
```

The new thing with TexCreate is that templates are locally stored in the directory `$HOME/.texcreate` where the structure 
looks like the following: 

```bash
$HOME/.texcreate/
                # where MKProj first party templates are saved
                mkproj/
                # where custom generated templates are saved
                custom/
```

To setup this directory structure you will need to run the following command: 

```bash
$ texcreate init 
```

To create a new config file, you will need to use the `new` command. The difference between v2 and v3 is the default use of 
`texcreate.toml` instead of `config.toml` and the ability to use different file names. Let's create a new project: 

```bash
$ texcreate new 
Use default project settings? (yes/no)
no
Enter Project Name: 
name
Enter Template Repo: 
mkproj
Enter Template Name: 
basic
Enter config file name (default: texcreate.toml): 
myfirst.toml
Successfully created `myfirst.toml`
```

If we look at `myfirst.toml` we can see the following: 

```toml
packages = []

[project]
proj_name = "name"
template = "basic"
repo = "mkproj"

[metadata]
author = "author"
date = "date"
title = "title"
fontsize = 11
doc_class = "article"
maketitle = true
```

Lastly to build the project use the `build` command: 
```bash
# when using build, if name isn't `texcreate.toml`, use flag -f 
$ texcreate build -f myfirst.toml
Successfully created `name`
```

If we use `ls` in the project, we can see the following structure: 

```bash
$ ls 
include  name.tex compiler.toml out
```

To compile we can use the `compile` command, do note you may change the LaTeX compiler used in the `compiler.toml` file. 
The default is `pdflatex` when a project is created. 

```bash
# make sure to be in project root
$ texcreate compile 
$ ls out 
name.pdf 
```

> Make sure to not change the `proj_name` field in `compiler.toml` or the command will not be able to compile. 


