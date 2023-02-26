# Compiling a LaTeX Project

To compile a TexCreate project you will need to be in the root of the project and use the `compile` command. 
The `compile` command reads the `compiler.toml` configuration and will run the appropriate LaTeX compiler and flags. 

Let's take a look at `compiler.toml`: 
```toml
compiler = "pdflatex"
proj_name = "Project"
flags = []
clean = true
mode = "Output"
```

Here's a breakdown of each of the fields: 
- `compiler`: The name of the LaTeX compiler to use (default: `pdflatex`)
- `proj_name`: The project name __(do not change or project will not compile)__
- `flags`: Any extra flags to add to the compiler command
- `clean`: Whether to remove the `aux` and `log` file from the `out` directory after compiling
- `mode`: Whether to run the process using `output()` or `spawn()`
  - `"Output"`: Executes the child process and collects the output _(doesn't see task)_
  - `"Spawn"`: Executes the child process returning a handle to it _(able to see task)_

If we run `texcreate compile` with the current configuration , it would be equivalent to running the following: 
```bash
$ pdflatex -out-directory=out Project
```


Let say we also wanted to use the `xelatex` compiler instead,add the flag `-interaction=nonstopmode` and change the mode to 
`"Spawn"`, then the configuration would look like the following: 

```toml
compiler = "xelatex"
proj_name = "Project"
flags = ["-interaction=nonstopmode"]
clean = true
mode = "Spawn"
```

Let's run `texcreate compile` in the root directory of our project: 
```bash
$ texcreate compile 
This is XeTeX, Version 3.141592653-2.6-0.999993 (TeX Live 2022/dev/Debian) (preloaded format=xelatex)
 restricted \write18 enabled.
entering extended mode
(./Project.tex
LaTeX2e <2021-11-15> patch level 1
L3 programming layer <2022-01-21>
(/usr/share/texlive/texmf-dist/tex/latex/base/article.cls
Document Class: article 2021/10/04 v1.4n Standard LaTeX document class
(/usr/share/texlive/texmf-dist/tex/latex/base/size11.clo))
(./include/structure.tex
(/usr/share/texlive/texmf-dist/tex/latex/amsmath/amsmath.sty
For additional information on amsmath, use the `?' option.
(/usr/share/texlive/texmf-dist/tex/latex/amsmath/amstext.sty
(/usr/share/texlive/texmf-dist/tex/latex/amsmath/amsgen.sty))
(/usr/share/texlive/texmf-dist/tex/latex/amsmath/amsbsy.sty)
(/usr/share/texlive/texmf-dist/tex/latex/amsmath/amsopn.sty)))
(/usr/share/texlive/texmf-dist/tex/latex/listings/listings.sty
(/usr/share/texlive/texmf-dist/tex/latex/graphics/keyval.sty)
(/usr/share/texlive/texmf-dist/tex/latex/listings/lstmisc.sty)
(/usr/share/texlive/texmf-dist/tex/latex/listings/listings.cfg))
(/usr/share/texlive/texmf-dist/tex/latex/l3backend/l3backend-xetex.def
(|extractbb --version))
No file Project.aux.
(/usr/share/texlive/texmf-dist/tex/latex/base/ts1cmr.fd) [1] (out/Project.aux) 
)
Output written on out/Project.pdf (1 page).
Transcript written on out/Project.log.
The project `Project` successfully compiled!
```