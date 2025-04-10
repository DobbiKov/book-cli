use std::process;

pub(crate) fn open_mac(path: &str) {
    process::Command::new("open")
        .args([path])
        .output()
        .expect("error while opening the file with Skim");
}
