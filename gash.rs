use std::{io};
use internal::*;
use command::*;
mod internal;
mod command;

fn main() {
    static CMD_PROMPT: &'static str = "gash";
    let mut hist = internal::HistoryLog { history: ~[] };
    let mut reexecute = 0;
    let mut line = ~"";
    command::init();
    loop {
	line = match reexecute {
	    0 => io::stdin().read_line(),
	    _ => { 
		print(fmt!("%s: .%s > ", CMD_PROMPT, internal::currentfolder()));
		line
	    }
	};
	reexecute = 0;
	debug!(fmt!("line: %?", line));
	let mut argv: ~[~str] = (line.split_iter(' ').filter(|&x| x != "")).map(|x: &str| x.to_owned()).collect();
	debug!(fmt!("argv %?", argv));
	hist.addhistory(line.clone());
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
			Some(ref x) if x == &~"-l"   => { line = hist.selecthistory();
							  reexecute = 1;
							}
			_                            => { hist.printhistory(); }
		    }    
		}

		_           => { command::parse(program, argv); }
	    }
	}
    }
}
