# flatpak-enhanced

[![flatpak-enhanced](https://img.shields.io/aur/version/flatpak-enhanced?color=1793d1&label=flatpak-enhanced&logo=arch-linux&style=for-the-badge)](https://aur.archlinux.org/packages/flatpak-enhanced/) 
[![CodeQL](https://github.com/JiiB1/flatpak-enhanced/actions/workflows/github-code-scanning/codeql/badge.svg)](https://github.com/JiiB1/flatpak-enhanced/actions/workflows/github-code-scanning/codeql)

A lightweight wrapper for the `flatpak` tool adding some usefull features.

## Current features :

- Aliases management

## Planned features :

- (None : **feel free to request !**)

## How to use

- On any Arch distro
  1. This package should be available via the **AUR**
  2. Run `yay -S flatpak-enhanced`

- Localy
  1. Clone this repo with `git clone https://github.com/JiiB1/flatpak-enhanced.git`
  2. Run `cargo --release --locked --all-features`
  3. You can find the executable in `./target/release/flatpak-enhanced`

> [!NOTE]
> You may want to create an alias : `$ alias fpe="flatpak-enhanced"`

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
