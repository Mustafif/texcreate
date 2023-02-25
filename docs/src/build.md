# Building a LaTeX Project

Creating a TexCreate project has the following structure (we will use the project name `Project` as an example) 
```bash
Project/
    # a configuration to compile TexCreate projects
    compiler.toml 
    # The main source code file
    Project.tex
    # where pdfs will be stored
    out/
    # where to place files from `\input{}`
    include/
      # contains anything related to packages
      structure.tex
  
```

Let's build our project using `texcreate.toml` that was created in the last section using the `build` command: 

```bash
# to build with different filename, use the -f flag 
$ texcreate build 
Successfully created `Project`
```

We can now look at `Project/Project.tex` to see how the `basic` template looks like (spoiler alert: it's basic...)
```latex
\documentclass[11pt, a4paper]{article}
\title{The Basic Template}
\author{Mustafif Khan}
\date{2022-2023}
\input{include/structure}
\begin{document}
    \maketitle
    \pagenumbering{arabic}
    \newpage
\end{document}
```

To look at the packages that are with our document, we will need to look at `Project/include/structure.tex` as shown: 

```latex
\usepackage{amsmath}
\usepackage{listings}
```

We can see the package __listings__ added to the file, so we are now able to proceed to the next section to compile our project. 

