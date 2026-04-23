# dotfiles-rust

Copy-based dotfiles tracking cli, written in rust

## todo

- [x] in fs module, add list_files and all_files functions
- [ ] add proper and comprehensive tests for fs module

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
