# Installing TexCreate

## Install Rust 

To install TexCreate you will need the `cargo` package manager (on Debian `amd64` Linux systems you can use alternative method) which means you will need to install 
`rust`. The recommended way of installation is using `rustup`, which can be found [here](https://rustup.rs/) and 
will allow you to install Rust for your particular operating system. 

## Install TexCreate 

#### Cargo Method

```bash
# install via Cargo 
$ cargo install texcreate
```

#### Debian Method

```bash
# this method is only for Debian amd64 Linux systems
$ curl --proto '=https' https://texcreate.mkproj.com/sh | sudo sh
```

With TexCreate installed, it is also recommended to have a LaTeX compiler installed as well, you may install 
`texlive` with their install instructions found [here](https://tug.org/texlive/doc/texlive-en/texlive-en.html#installation). 