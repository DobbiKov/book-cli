use std::process;

pub(crate) fn open_mac(path: &str) {
    process::Command::new("open")
        .args([path])
        .output()
        .expect("error while opening the file with Skim");
}
pub(crate) fn open_pdftermpy(path: &str) {
    process::Command::new("termpdf.py")
        .args([path])
        .status()
        .expect("error while opening the file with Skim");
}
