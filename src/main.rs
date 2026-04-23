use dotfiles_rust::fs::{AbsPath, RelPath};

fn main() {
    let tmp_dir = AbsPath::new_tmp("dotfiles_rust_example");
    let tmp_file1 = tmp_dir.join(&RelPath::from("file1.txt"));
    let tmp_file2 = tmp_dir.join(&RelPath::from("file2.txt"));
    tmp_dir.create_dir().unwrap();
    tmp_file1.create_file().unwrap();
    tmp_file2.create_file().unwrap();
    tmp_dir.purge_path(true).unwrap();
}
