# mascara <img alt="GitHub release (latest by date including pre-releases)" src="https://img.shields.io/github/v/release/ethgallucci/mascara?color=orange&include_prereleases&style=flat-square"> <img alt="Commit activity" src="https://img.shields.io/github/commit-activity/m/ethgallucci/mascara?style=flat-square" /> <img alt="Lines of code" src="https://img.shields.io/tokei/lines/github/ethgallucci/mascara?color=green&style=flat-square"> <img alt="GitHub issues" src="https://img.shields.io/github/issues/ethgallucci/mascara?color=white&style=flat-square">

An experimental package manager/config initializer tool for system hoppers.

## Version History

* 0.0.8-pre_release
    * Initial Release

## mascara.toml
```toml
[mascara]
feature = "Debian"
logs = { stdout = "blue", stderr = "red" }

[packages.defaults.curl]
[packages.defaults.cmake]

[packages.defaults.neovim]
cfg.after = { bin = "sudo", args = ["add-apt-repository", "ppa:neovim-ppa/stable"] }

[packages.defaults.git]
cfg.after = { bin = "git", args = ["--version"] }

[packages.defaults.zsh]
cfg.after = { bin = "git", args = ["clone", "me/dotfiles", "&&", "cp", "dotfiles/.zshrc", ".zshrc"] }

[packages.fallbacks.rust]
cfg.after = { bin = "cargo", args = ["install", "exa"] }
fallback = "curl"
cmd = { bin = "curl", args = ["--proto", "'=https'", "--tlsv1.2", "-sSf", "https://sh.rustup.rs", "|", "sh"] }
```
