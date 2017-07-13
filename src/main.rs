use std::fs;

extern crate clap;
extern crate nix;

use clap::{Arg, App};
use nix::unistd::{fork, ForkResult, execvp};
use nix::sched::unshare;
use nix::mount;

use std::path::Path;
use std::ffi::CString;

//fn print_dir(path: &Path) -> std::io::Result<()> {
//    for entry in fs::read_dir(path)? {
//        let entry = entry?;
//        let file_name = entry.file_name();
//        println!("{:?}", file_name);
//    }
//    Ok(())
//}

fn main() {
    let matches = App::new("bucket")
                          .arg(Arg::with_name("ROOTFS_PATH")
                               .help("Path to the rootfs")
                               .required(true)
                               .index(1))
                          .arg(Arg::with_name("COMMAND")
                               .help("Command to run")
                               .required(true)
                               .index(2))
                          .get_matches();
    let rootfs_path = matches.value_of("ROOTFS_PATH").unwrap();
    //print_root().unwrap();
    //println!("\n\n***********\n");
    //print_root().unwrap();

    println!("PID: {:?}", nix::unistd::getpid());
    
    let command = matches.value_of("COMMAND").unwrap();
    println!("Cmd: {:?}", command);
    
    // TODO: Build up flags to unshare more things
    unshare(nix::sched::CLONE_NEWPID).unwrap();

    match fork() {
        Ok(ForkResult::Parent { child, .. }) => {
            println!("BIG boi. Child is {:?}", child);
            nix::sys::wait::waitpid(child, None).unwrap();
            return;
        }
        Ok(ForkResult::Child) => {
           println!("Child!"); 
        }
        Err(_) => panic!("Fork failed")
    }
    println!("outta the block");

    let proc_path = rootfs_path.to_owned() + "/proc";
    mount::mount(Some("none"), proc_path.as_str(), None::<&str>, mount::MS_PRIVATE, None::<&str>).unwrap();
    mount::mount(Some("proc"), proc_path.as_str(), Some("proc"), mount::MS_NODEV, None::<&str>).unwrap();

    //nix::unistd::chroot(rootfs_path).unwrap();

    //print_dir("/proc".as_ref());
    println!("PID: {:?}", nix::unistd::getpid());
    

    let c_string = CString::new(command).unwrap();
    let chroot_string = CString::new("chroot").unwrap();
    let rootfs_str = CString::new(rootfs_path).unwrap();

    let execv_args = vec![chroot_string.clone(), rootfs_str.clone(), c_string.clone()];
    execvp(&chroot_string, &execv_args).unwrap();


}

