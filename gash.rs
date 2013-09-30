use std::{io};
use internal::*;
use command::*;
use std::libc::*;
mod internal;
mod command;

static SA_MASK_SIZE: c_int = 16;

struct sigaction_t {
    handler: extern "C" fn(c_int, *c_void, *c_void),
    sa_mask: [c_ulong, ..16],
    sa_flags: c_int,
    sa_restorer: extern "C" fn()
}

static SIG_ERR: c_int = -1;
static SIG_DFL: c_int = 0;
static SIG_IGN: c_int = 1;
static SIG_HOLD: c_int = 2;

#[no_link]
extern "C" {
    fn sigaction(sig: c_int, in_: *sigaction_t, out: *sigaction_t) -> c_int;
}

extern "C" fn handle(number: c_int, info: *c_void, userdata: *c_void) {
    let s = "Why does this exit?".to_c_str();
    unsafe {
	s.with_ref(|x| puts(x));
    }
}

#[fixed_stack_segment]
fn main() {
    let sa = sigaction_t {
	handler: handle,
	sa_mask: [0, ..16],
	sa_flags: 0,
	sa_restorer: unsafe { std::cast::transmute(0) }
    };

    unsafe {
	sigaction(2, &sa, SIG_DFL as *sigaction_t);
    }

    static CMD_PROMPT: &'static str = "gash";
    let mut hist = internal::HistoryLog { history: ~[] };
    command::init();
    loop {
	print(fmt!("%s: .%s > ", CMD_PROMPT, internal::currentfolder()));
	let line = io::stdin().read_line();
	debug!(fmt!("line: %?", line));
	let mut argv: ~[~str] = (line.split_iter(' ').filter(|&x| x != "")).map(|x: &str| x.to_owned()).collect();
	debug!(fmt!("argv %?", argv));
	hist.addhistory(line);     
	if argv.len() > 0 {
	    let program = argv.remove(0);
	    match program {
		~"exit"     => { 
		    command::kill_children();
		    break; 
		}

		~"cd"       => { 
		    if argv.len() > 0 {
			changedir(argv.remove(0));
		    }
		}

		~"history"  => { 
		    match argv.shift_opt() {
			Some(ref x) if x == &~"-c"   => { hist.clearhistory(); }
			_                            => { hist.printhistory(); }
		    }    
		}

		_           => { command::parse(program, argv); }
	    }
	}
    }
}
