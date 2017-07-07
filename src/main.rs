use std::fs;

extern crate nix;

fn print_root() -> std::io::Result<()> {
    for entry in fs::read_dir("/")? {
        let entry = entry?;
        let file_name = entry.file_name();
        println!("{:?}", file_name);
    }
    Ok(())
}

fn main() {
    print_root().unwrap();
    nix::unistd::chroot("rootfs");
    print_root().unwrap();
}
