# flatpak-enhanced [![CodeQL](https://github.com/JiiB1/flatpak-enhanced/actions/workflows/github-code-scanning/codeql/badge.svg)](https://github.com/JiiB1/flatpak-enhanced/actions/workflows/github-code-scanning/codeql) [![Tests](https://github.com/JiiB1/flatpak-enhanced/actions/workflows/rust.yml/badge.svg)](https://github.com/JiiB1/flatpak-enhanced/actions/workflows/rust.yml)

A lightweight wrapper for the `flatpak` tool adding some usefull features.

## Current features :

- Aliases management

## Planned features :

- (None : **feel free to request !**)

## How to use

### Aliases management - `flatpak-enhanced alias`

This feature allow you to manage aliases. These will be replaced in any `flatpak` base-command.

#### Examples:

- Creation - `flatpak-enhanced alias create <TARGET> [ALIASES]... ` 
```shell
$ flatpak-enhanced alias create org.mozilla.firefox firefox
```
```shell
$ flatpak-enhanced alias create firefox cool_browser fox-enjoyer-browser
```

- Listing - `flatpak-enhanced alias list [TARGET]`
```shell
$ flatpak-enhanced alias list
```
```shell
$ flatpak-enhanced alias list firefox
```

- Removal - `flatpak-enhanced alias remove [ALIASES]... ` 
```shell
$ flatpak-enhanced alias remove browser cool_browser fox-enjoyer-browser
```

---

Please, do not hesitate to test it, report bugs or request features
