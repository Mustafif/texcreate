# Creating a new Config

Now that our `texcreate` directory is created, we can start working on TexCreate projects and to begin that process 
is creating a new configuration file (the config is a `toml` file, with `texcreate.toml` being the default filename). 

To create a new config, we will use the `new` command which will show us a prompt to get started as shown: 

```bash
$ texcreate new 
Use default project settings? (yes/no)
no
Enter Project Name: 
Project
Enter Template Repo: 
mkproj
Enter Template Name: 
basic
Enter config file name (default: texcreate.toml): 

Successfully created `texcreate.toml`
```

Let's look inside `texcreate.toml`:
```toml
packages = []

[project]
proj_name = "Project"
template = "basic"
repo = "mkproj"

[metadata]
author = "author"
date = "date"
title = "title"
fontsize = 11
papersize = "letterpaper"
doc_class = "article"
maketitle = true
```

Let's go through each part of `texcreate.toml`, starting with the field `packages`.
The `package` field will be where you may add extra packages to be included inside the project, 
for example the `basic` template doesn't contain the `listings` package, so we may add that, as shown: 

```toml
packages = ["listings"]
[project]
...

[metadata]
...
```

Next comes the `[project]` section which contains three fields, the project name, the template and the repo where 
the template is located. In our example we have our project name as __Project__, the template we chose is __basic__ and 
this can be found from the __mkproj__ repo. 

Lastly we would like to customize the prebuilt templates to contain the metadata we would like it to have, and 
that is where the `[metadata]` section comes in. This section uses `texcore::Metadata` and contains all relevant fields 
like author, date, title of the document, etc. 

We should change the `[metadata]` section to fit our needs, for example I'll change it to the following: 
```toml
...
[metadata]
author = "Mustafif Khan"
date = "2022-2023"
title = "The Basic Template"
fontsize = 11
papersize = "a4paper"
doc_class = "article"
maketitle = true
```

Congratulations you have created a new TexCreate config, you are now one step closer to building a project!


