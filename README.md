# flatpak-enhanced

[![flatpak-enhanced](https://img.shields.io/aur/version/flatpak-enhanced?color=1793d1&label=flatpak-enhanced&logo=arch-linux&style=for-the-badge)](https://aur.archlinux.org/packages/flatpak-enhanced/) 
[![CodeQL](https://github.com/JiiB1/flatpak-enhanced/actions/workflows/github-code-scanning/codeql/badge.svg)](https://github.com/JiiB1/flatpak-enhanced/actions/workflows/github-code-scanning/codeql)

A lightweight wrapper for the `flatpak` tool adding some usefull features.

## Current features :

- **Simple run** : Use `$ flatpak-enhanced <APP>` instead of `$ flatpak run <APP>`
- **Application aliases** : Use `obs` instead of `com.obsproject.Studio`

## Planned features :

- Performances improvement with cache for frequently used aliases (`V0.2.1`, **WIP**)
- Add support for user or custom insstalled application (`V0.2.1`)
- (None : **feel free to request !**)

## Installation

- On any Arch distro
  1. This package should be available via the **AUR**
  2. Run `yay -S flatpak-enhanced`

- Localy
  1. Clone this repo with `git clone https://github.com/JiiB1/flatpak-enhanced.git`
  2. Run `cargo --release --locked --all-features`
  3. You can find the executable in `./target/release/flatpak-enhanced`

> [!NOTE]
> You may want to create an alias : `$ alias fpe="flatpak-enhanced"`

## Documentation and changelogs

Please, read the wiki [here](https://github.com/JiiB1/flatpak-enhanced/wiki).

---

Please, do not hesitate to test it, report bugs or request features
