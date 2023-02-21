# TexCreate by Mustafif Khan
## Version 3.0.0-beta.6

> This project is under the [MIT License](LICENSE)

TexCreate is a LaTeX Project creator that allows users to easily create projects using a configuration file that will 
define a template, project name, and metadata. Compared to version 2 which last saw an update 10 months ago, version 3 
comes with the focus on template modularity and stability. 

## Template Modularity? 
In previous versions the templates are in someway embedded in the programs code, such as in version 2 we saw that being done 
using `texc_latex` crate. However, this limited the amount of templates that could be available to users as they would have to 
wait for a minor update to see a template addition. So...how was this fixed? This was solved in 2 parts, first was by creating 
an opensource template generator `texcgen` (note: TexCreate templates are built with a fork `mkproj_texcgen`), and the second part 
is done in a web service `texcweb`. The ability to put a template into a JSON file using the handy `texcore` library I developed 
allows it to be easily downloaded in the latest releases and TexCreate is able to know about these updates because of `texcweb`. 


## How to get started?

### Installing

```bash
$ cargo install texcreate --version 3.0.0.beta.6
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


## Changes in Beta 2 

- Added the `compile` command 
  - Required creating the `Compiler` type and making changes to `Project`
- Added a `-r/--repo` flag to the `list` command 
  - Allows to specify the `mkproj` or `custom` repo

## Changes in Beta 3 
- Added the `texcgen` command
  - Allows users to create/interact with `texc_gen` using `texcreate`

## Changes in Beta 4 
- Fixed the `texcgen` command 
- Altered the `update_alert()` to show a prettier message 

## Changes in Beta 5 
- Added the `zip` command 
  - Allows users to create a zip file of a TexCreate project
- Added the `open` command 
  - Opens [texcreate.mkproj.com](https://texcreate.mkproj.com) in their default browser
- Added the `web` command
  - Runs a web service to create TexCreate projects 

## Changes in Beta 6 
- Updated to `texcore 0.4.18`
- Fixed breaking in `3.0.0-beta.5`
- Fixed logging in `web` command 
- Added `papersize` option to `WebConfig` because of update to `texcore`