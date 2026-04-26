# versions

Use SemVer!

## how to release a new version

This will be done mostly manually. There are 2 relevant processes involved in updating the version:
- update `Cargo.toml` version, this way rust binary will be able to correct detect the current version
- trigger github `.github/workflows/release.yml` action to actually have binaries compiled and a release made,
    by creating a tag called `v*.*.*`, 

There is no other system involved! Versions will be manually updated manually, making sure the 2 different
versioning system use the same versions

## when to release new versions
