# Initializing TexCreate

Unlike in the previous TexCreate versions, TexCreate version 3 focused on template modularity and instead of 
having templates hardcoded in the program, they are instead stored locally in your system as JSON files (done using the `texcore` library). 
To allow this, we will 
need to create the layout using the `init` command. 

The layout we are talking about looks like the following: 
```bash
# where the directory is created
$HOME/.texcreate/
            # keeps metadata for mkproj templates 
            repo.toml
            # stores mkprojects first party templates
            mkproj/
                *.json
            # stores custom templates saved by texcgen 
            custom/
              *.json
```

To create this run the following command: 
```bash
$ texcreate init 
```

The `init` command will create the `.texcreate` directory and get the latest repo of templates from the 
`mkproj_texcgen` repository. 

For example, at the time of writing the current repo release is v1, and if we use the `list` command we can see 
the following output of templates: 

```bash
# to see list of custom templates use flag --repo custom 
$ texcreate list 
TexCreate Repo: v1
Number of Templates: 6
======TEMPLATES======
=> lachaise: A perfect way to write assignments
=> dictionary: Create your own dictionary!
=> basic: A Basic LaTeX Document
=> theatre: Create your own script for a play
=> code: Code Documentation Focused
=> news: Write your own newspaper article
=====================
```