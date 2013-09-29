use std::{os, run, task, libc};

use openWriteCreate = (libc::consts::os::posix88::O_WRONLY | libc::consts::os::posix88::O_CREAT) as libc::c_int;
use openRead = libc::consts::os::posix88::O_RDONLY;
use modeReadWrite = (libc::consts::os::posix88::S_IWUSR | libc::consts::os::posix88::S_IRUSR) as libc::c_int;
pub fn parse(command : &str, argv: &[~str]) {
	match argv.iter().last() {
          Some(y) if y == &~"&" => {
                  	 let freezeArg = argv.to_owned().clone();
                     let freezeCmd = command.to_owned().clone();
                     do task::spawn_supervised {
                              execute(freezeCmd, freezeArg.slice_to(freezeArg.len() - 1), None);
                            }
                        }
           _                => { execute(command, argv, Some(0)); }
                    }
}

enum RunType {
    FileIn,
    OutFile,
    ResultIn,
    Normal
}

fn execute(command: &str, args: &[~str], instream: Option<i32>) {
    #[fixed_stack_segment]; #[inline(never)];
    let (runtype, index) = parse_pipes(args);
    match runtype {
        FileIn  =>  {
            let left = args.slice_to(index);
            let right = args.slice_from(index+1);
            let file : &str = right.head().to_owned();
            let path = &Path(file);
            if !os::path_exists(path) {
                println(fmt!("%s isn't a valid file!", file));
                return;
            }

            if os::path_is_dir(path) {
                println("Can't read from a directory!");
                return;
            }

            let mut fd : libc::c_int;
            unsafe {
                fd = do path.with_c_str |pathbuf| {
                     libc::open(pathbuf, openRead, modeReadWrite)
                };
            }
            if fd > 0 {
                run::Process::new(command, 
                                  left, 
                                  run::ProcessOptions {
                                       env: None,
                                       dir: None,
                                       in_fd: Some(fd),
                                       out_fd: Some(1),
                                       err_fd: Some(2)
                                       });
                unsafe {
                    libc::close(fd as i32);
                }
            }
            else {
                println(os::last_os_error());
            }
        }
        OutFile =>  {
            let left = args.slice_to(index);
            let right = args.slice_from(index+1);
            let file : &str = right.head().to_owned();
            let path = &Path(file);
            if os::path_is_dir(path) {
                println("Can't write to a directory!");
                return;
            }

            let mut fd : libc::c_int;
            unsafe {
                fd = do path.with_c_str |pathbuf| {
                     libc::open(pathbuf, openWriteCreate, modeReadWrite)
                };
            }
            if fd > 0 {
                run::Process::new(command, 
                                  left, 
                                  run::ProcessOptions {
                                       env: None,
                                       dir: None,
                                       in_fd: instream,
                                       out_fd: Some(fd),
                                       err_fd: Some(2)
                                       });
                unsafe {
                    libc::close(fd as i32);
                }
            }
            else {
                println(os::last_os_error());
            }
        }
        ResultIn=>  { 
            let left = args.slice_to(index);
            let right = args.slice_from(index+1);
            let results = run::process_output(command, left);
            let secondCommand: &str = right.head().to_owned();
            let mut second = run::Process::new(secondCommand, 
                                  right.tail(), 
                                  run::ProcessOptions {
                                       env: None,
                                       dir: None,
                                       in_fd: None,
                                       out_fd: Some(1),
                                       err_fd: Some(2)
                                       });
            let secondIn = second.input();
            secondIn.write(results.output);
                       
        }
        _       =>  { run::Process::new(command, 
                                  args, 
                                  run::ProcessOptions {
                                       env: None,
                                       dir: None,
                                       in_fd: instream,
                                       out_fd: Some(1),
                                       err_fd: Some(2)
                                       }); 
                    }
    } 
}

fn parse_pipes(args: &[~str]) -> (RunType, uint) {
    match args.position_elem(&~"<") {
        Some(i) =>  { return (FileIn, i); }
        None    =>  { }
    }
    match args.position_elem(&~">") {
        Some(i) =>  { return (OutFile, i); }
        None    =>  { }
    }
    match args.position_elem(&~"|") {
        Some(i) =>  { return (ResultIn, i); }
        None    =>  { }
    }
    (Normal, 0)
}

pub fn init() {

}

pub fn kill_children() {

}