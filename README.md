# WRESTIC

<div align="middle">
  <img src="misc/wr_light.svg#gh-dark-mode-only" height="350" />
  <img src="misc/wr_dark.svg#gh-light-mode-only" height="350" />
</div>

:star: Star me up!

Wrestic is a backup tool built in Rust that provides a wrapper around Restic, a popular backup program. With Wrestic, you can easily configure and run backups of your files and directories, and take advantage of Restic's powerful features such as deduplication, encryption, and compression. Whether you need to back up your personal files or your organization's data, Wrestic can help you automate the process and ensure your data is safe and secure.

Wrestic has support for all the restic backends, including:

> - LOCAL
> - SFTP
> - REST
> - AMAZON S3
> - AZURE
> - BACKBLAZE B2
> - WASABI
> - MINIO
> - GOOGLE CLOUD STORAGE

## TABLE OF CONTENTS[![](https://raw.githubusercontent.com/aregtech/areg-sdk/master/docs/img/pin.svg)](#table-of-contents)
- [WRESTIC](#wrestic)
  - [TABLE OF CONTENTS](#table-of-contents)
  - [INSTALLATION](#installation)
    - [WITH CARGO](#with-cargo)
    - [DOWNLOAD BINARY](#download-binary)
    - [BUILD FROM SOURCE](#build-from-source)
  - [CONFIGURATION](#configuration)
  - [USAGE](#usage)
  - [COMPLETIONS](#completions)



## INSTALLATION

### WITH CARGO
```sh
cargo install wrestic
```
### DOWNLOAD BINARY

```sh
curl -sL $(curl -s https://api.github.com/repos/alvaro17f/wrestic/releases/latest | grep browser_download_url | cut -d '"' -f 4) | sudo tar zxf - -C /usr/bin --overwrite
```

### BUILD FROM SOURCE
Requirements:
- [git](https://git-scm.com/)
- [rust](https://rust-lang.org/)

```sh
git clone https://github.com/alvaro17f/wrestic.git
cd wrestic
cargo build --release
sudo cp target/release/wrestic /usr/bin
```

## CONFIGURATION

Copy `wrestic.toml` to `/home/$USER/.config/wrestic/wrestic.toml` and modify the content for your needs.

## USAGE

<div align="middle">
  <img src="misc/wrestic_mockup.png" height="350" /> 
</div>

Simply run `sudo wrestic`.

```sh
$ wrestic help

Restic wrapper built in Rust

Usage: wrestic [OPTIONS] [COMMAND]

Commands:
  backup, -b     Make a backup of all your repositories
  restore, -r    Restore a snapshot
  snapshots, -s  List snapshots
  delete, -d     Delete a snapshot
  init, -i       Initialize all of your repositories
  check          Check repository health
  repair         Fix any issue
  cache          Clean cache
  update, -u     Update Wrestic
  custom, -c     Custom command
  help           Print this message or the help of the given subcommand(s)

Options:
      --generate <GENERATOR>  [possible values: bash, elvish, fish, powershell, zsh]
  -h, --help                  Print help
  -V, --version               Print version

```

## COMPLETIONS

> if your shell is `bash` you'll also need the `bash-completion` package installed.

To get `<TAB>` completions run `sudo wrestic --generate <your shell>`
