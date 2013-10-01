extern mod extra;
use std::{os, io};
use self::extra::{time};
mod command;

pub fn changedir(pathstr: &str) {
    let path = &Path(pathstr);
    if !(os::path_exists(path) && os::path_is_dir(path) && os::change_dir(path)){
	println(fmt!("Error: %s is not a valid directory", pathstr));
    }
}

pub fn currentfolder() -> ~str {
    let dir = os::getcwd().to_str();
    let index = match(dir.rfind('/')) {
	Some(f) => { f }
	None => { 5 }
    };
    dir.slice_from(index).to_owned()
}

#[deriving(Clone)]
pub struct Record {
    command : ~str,
    timestamp: time::Tm
}

pub struct HistoryLog {
    history: ~[Record]
}

impl HistoryLog {

    pub fn addhistory(&mut self, cmd : ~str) {
	let temp = Record {command: cmd, timestamp:time::now()};
	self.history.push(temp);
    }

    pub fn printhistory(&self) {
	let mut counter = 0;
	println("\nHistory:\n");
	for cmd in self.history.rev_iter() {
	    let tabs = "\t".repeat(3-cmd.command.char_len()/8+1);
	    println(fmt!("%d: %s%s%s", (self.history.len() - counter) as int, cmd.command, tabs, cmd.timestamp.rfc822()));
	    counter += 1;
	}
    }

    pub fn selecthistory(&self) -> ~str {
	self.printhistory();
	print("Select a previous command: ");
	match from_str::<int>(io::stdin().read_line()) {
	    Some(k)	=> { self.history[k-1].command.clone() }
	    None	=> { println(fmt!("Bad Input!") ); ~"" }
	}

    }

    pub fn clearhistory(&mut self) {
	self.history = ~[]
    }
}
