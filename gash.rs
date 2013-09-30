use std::{io};
use internal::*;
use command::*;
mod internal;
mod command;

fn main() {
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
