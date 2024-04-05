# tera-cli

Simple tera template engine CLI.

## Setup

you need cargo:
  
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

you need just to run dev tools:
```sh
brew install just
```

you need some python tools, `pre-commmit` and `commitize`.
I recommend using pipx:

```sh
brew install pipx
pipx install pre-commit commitizen
pre-commit install
```

## Develop

lint:
```sh
just lint
```

commit:
```sh
cz c
```

## Release

bump version for pre-release:
```sh
just pre-release-dry-run # check what will be changed
just pre-release
```

bump version for release:
```sh
just release-dry-run # check what will be changed
just release
```


## Install from Github Release

You can download the binary from the [github release page](https://github.com/kj-9/tera-cli/releases).

As an example, to install the binary for aarch64-apple-darwin:

```sh
curl -O -L https://github.com/kj-9/tera-cli/releases/download/v0.0.1-a2/tera-aarch64-apple-darwin.tar.gz
tar -xzf tera-aarch64-apple-darwin.tar.gz
mv tera /usr/local/bin/
tera --version
```