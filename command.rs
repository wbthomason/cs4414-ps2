use std::{cast, os, run, task, libc};

static SA_MASK_SIZE: libc::c_int = 16;

struct sigaction_t {
    handler: extern "C" fn(libc::c_int, *libc::c_void, *libc::c_void),
    sa_mask: [libc::c_ulong, ..16],
    sa_flags: libc::c_int,
    sa_restorer: extern "C" fn()
}

static SIG_ERR: libc::c_int = -1;
static SIG_DFL: libc::c_int = 0;
static SIG_IGN: libc::c_int = 1;
static SIG_HOLD: libc::c_int = 2;

#[no_link]
extern "C" {
    fn sigaction(sig: libc::c_int, in_: *sigaction_t, out: *sigaction_t) -> libc::c_int;
}

extern "C" fn handle(number: libc::c_int, info: *libc::c_void, userdata: *libc::c_void) {
    let s = "Got SIGINT!".to_c_str();
    unsafe {
	s.with_ref(|x| libc::puts(x));
    }
}


pub fn parse(command : &str, argv: &[~str]) {
	match argv.last_opt() {
          Some(y) if y == &~"&" => { let freezeArg = argv.to_owned().clone();
                     let freezeCmd = command.to_owned().clone();
                     do task::spawn_supervised {
                              task::deschedule();
                              execute(freezeCmd, freezeArg.slice_to(freezeArg.len() - 1), None);
                            }
                            }
           None                => { execute(command, argv, Some(0)); }
           _                   => { execute(command, argv, Some(0)); }
                    }
}

enum RunType {
    FileIn,
    OutFile,
    ResultIn,
    Normal
}

#[fixed_stack_segment]
fn execute(command: &str, args: &[~str], instream: Option<i32>) {
    #[fixed_stack_segment]; #[inline(never)];
    let sa = sigaction_t {
	handler: handle,
	sa_mask: [0, ..16],
	sa_flags: 0,
	sa_restorer: unsafe { cast::transmute(0) }
    };

    unsafe {
	sigaction(2, &sa, 0 as *sigaction_t);
    }

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
                     libc::open(pathbuf, libc::consts::os::posix88::O_RDONLY, 
                        (libc::consts::os::posix88::S_IWUSR | libc::consts::os::posix88::S_IRUSR) as libc::c_int)
                };
            }
            if fd > 0 {
                let mut proc = run::Process::new(command, 
                                  left, 
                                  run::ProcessOptions {
                                       env: None,
                                       dir: None,
                                       in_fd: Some(fd),
                                       out_fd: Some(1),
                                       err_fd: Some(2)
                                       });
                proc.finish();
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
                     libc::open(pathbuf, (libc::consts::os::posix88::O_WRONLY | libc::consts::os::posix88::O_CREAT) as libc::c_int, 
                        (libc::consts::os::posix88::S_IWUSR | libc::consts::os::posix88::S_IRUSR) as libc::c_int)
                };
            }
            if fd > 0 {
                let mut proc = run::Process::new(command, 
                                  left, 
                                  run::ProcessOptions {
                                       env: None,
                                       dir: None,
                                       in_fd: instream,
                                       out_fd: Some(fd),
                                       err_fd: Some(2)
                                       });
                proc.finish();
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
            second.finish();
                       
        }
        _       =>  { let mut proc = run::Process::new(command, 
                                  args, 
                                  run::ProcessOptions {
                                       env: None,
                                       dir: None,
                                       in_fd: instream,
                                       out_fd: Some(1),
                                       err_fd: Some(2)
                                       }); 
                        proc.finish();
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
