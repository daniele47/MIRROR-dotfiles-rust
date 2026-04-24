# dotfiles-rust

Copy-based dotfiles tracking cli, written in rust

## ideas

```
dotfiles/
├── autosaver
├── .defaults
├── configs/
│   ├── module1.conf
│   ├── module2.conf
│   ├── profile1.conf
│   └── profile2.conf
└── backups/
    ├── module1/
    └── module2/
```

- autosaver: bash wrapper script to get rust binary (downloaded/compiled) and run it
- .defaults: NOT TRACKED file to store default configurations, things like what module/profile to use by default
- configs: all modules and profile configurations, one config x file
- modules: simple list of files to track
- profiles: groups of modules to apply sequentially
- backups: each module has exactly one backup dir where to save its files, and named like the module

For example: if `neovim`, `tmux`, `plasma-desktop` are possible modules, 
`minimal-cli` or `kde-linux` are possible profiles and minimal-cli would
only have neovim and tmux, for example

Note: modules and profiles can be differenciated by a fake shebang in the file.
Aka: I could have lines start with // for comments, whilst /! or /<whatever> could 
now get a defined meaning!!!

Note: modules and profiles will be for semplicity be referred to as profiles, since modules are techinically
a special profile with a single module in it (aka itself)

Ideas:
    - shebang like first line in config files, to differenciate between profiles and modules
    - profiles could themselves include other profiles too! (decide how to handle recursion eventually!)
    - use proper subcommands instead of flags: save / restore / ... (flags only for config things, like specifying the profile to use)

## todo

- [x] in fs module, add list_files and all_files functions
- [x] add proper and comprehensive tests for fs module
- [x] make list_files and all_files return BTreeSet which is always sorted!
