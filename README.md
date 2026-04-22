# dotfiles-rust

Copy-based dotfiles tracking cli, written in rust

## todo

- [ ] modules and profile (modules aka neovim, tmux, kde configs, ...; profile aka kde-linux, fedora-kde, minimal-cli, ...)
    - [ ] modules must be allowed to track same files. for example neovim and neovim-raspberrypi both will track .config/nvim, but will have different files
    - [ ] profiles are just a list of modules to apply in order
- [ ] proper error system
- [ ] proper docs and help message

## ideas

```
dotfiles/
+--- modules/              # settings (files to track)
|    |--- module1.conf
|    +--- module2.conf
+--- profiles/             # settings (list of modules for each profile)
|    |--- profile1.conf
|    +--- profile2.conf
+--- backups/              # managed automatically
     |--- module1/
     +--- module2/
```
