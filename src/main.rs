use std::env;

use autosaver::core::{
    error::Result,
    fs::{AbsPath, LineWriter},
    profile::{Profile, composite::HashMapProfileLoader},
};

fn purge_path_even_on_panic(tmpdir: &AbsPath) -> impl Drop {
    struct Guard(AbsPath);
    impl Drop for Guard {
        fn drop(&mut self) {
            let _ = self.0.purge_path(true);
        }
    }
    Guard(tmpdir.clone())
}

fn main() -> Result<()> {
    println!("Binary version: {}", env!("CARGO_PKG_VERSION"));

    let tmpdir = AbsPath::new_tmp("rust_example");
    tmpdir.create_dir()?;
    let _guard = purge_path_even_on_panic(&tmpdir);

    // create first module
    let tmpfile1 = tmpdir.joins(&["neovim.conf"]);
    let mut writer = tmpfile1.line_writer()?;
    writer.write_all_lines([
        "/! type module",
        "",
        "// just testing with neovim configuration as an example",
        ".config/nvim",
        "",
        "/! policy ignore",
        ".config/nvim/lazy-lock.json",
    ])?;

    // create second module
    let tmpfile2 = tmpdir.joins(&["tmux.conf"]);
    let mut writer = tmpfile2.line_writer()?;
    writer.write_all_lines([
        "/! type module",
        "",
        "// just testing with neovim configuration as an example",
        ".config/tmux",
    ])?;

    // create profile with both
    let tmpfile3 = tmpdir.joins(&["tools.conf"]);
    let mut writer = tmpfile3.line_writer()?;
    writer.write_all_lines(["/! type composite", "", "neovim", "tmux"])?;

    // load all profiles
    let mut profile_loader = HashMapProfileLoader::new();
    let profiles = profile_loader.profiles();
    for (p, f) in [
        ("neovim", &tmpfile1),
        ("tmux", &tmpfile2),
        ("tools", &tmpfile3),
    ] {
        profiles.insert(p.into(), Profile::parse(p.into(), f.line_reader()?)?);
    }

    for profile in profiles {
        println!("{profile:#?}\n\n");
    }

    // let reader = tmpfile.line_reader()?;
    // let profile = Profile::parse("neovim".to_string(), reader)?;
    //
    // println!("\nPARSED:\n{profile:#?}");
    //
    // match profile.ptype() {
    //     autosaver::core::profile::ProfileType::Composite(_composite) => todo!(),
    //     autosaver::core::profile::ProfileType::Module(module) => {
    //         let resolved_profile = module.resolve(&AbsPath::from(
    //             env::var("HOME").expect("HOME not set!").as_str(),
    //         ))?;
    //         println!("\nRESOLVED:\n{resolved_profile:#?}");
    //     }
    // }

    Ok(())
}
