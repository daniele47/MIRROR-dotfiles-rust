use dotfiles_rust::core::{errors::Result, fs::AbsPath};

fn main() -> Result<()> {
    let abs = AbsPath::from("/etc/passwd");
    abs.create_file(true).unwrap();
    for line in abs.read_lines().unwrap() {
        let line = line?;
        println!("{line}");
    }
    Ok(())
}
