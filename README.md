# tera-cli

Simple tera template engine CLI.

## Setup

install rustup and cargo:
  
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```


install dev tools:

```sh
pipx install pre-commit commitizen
pre-commit install
```

## Develop

lint:
```sh
pre-commit run --all-files
```

commit:
```sh
cz c
```

## Release

bump version:
```sh
cz bump --prerelease alpha --changelog # for alpha version

# or
cz bump --changelog # for release version
```

then push tag to trigger release:
```sh
git push --follow-tags
```
