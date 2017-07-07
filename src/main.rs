use std::fs;

extern crate clap;
extern crate nix;

use clap::App;

fn print_root() -> std::io::Result<()> {
    for entry in fs::read_dir("/")? {
        let entry = entry?;
        let file_name = entry.file_name();
        println!("{:?}", file_name);
    }
    Ok(())
}

fn main() {
    let matches = App::new("bucket")
                          .arg(Arg::with_name("ROOTFS_PATH")
                               .help("Path to the rootfs")
                               .required(true)
                               .index(1)
                          .get_matches();
    let rootfs_path = matches.value_of("ROOTFS_PATH").unwrap();
    println!("rootfs path: {:?}", rootfs_path);
    print_root().unwrap();
    nix::unistd::chroot(rootfs_path).unwrap();
    println!("\n\n***********\n");
    print_root().unwrap();
}

