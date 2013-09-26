extern mod extra;
use std::{os};
use self::	extra::{time};

pub fn changedir(pathstr: &str) {
	let path = &Path(pathstr);
	if !(os::path_exists(path) && os::path_is_dir(path) && os::change_dir(path)){
		println(fmt!("Error: %s is not a valid directory", pathstr));
	}
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
		for cmd in self.history.iter() {
			let tabs = "\t".repeat(3-cmd.command.char_len()/8);
			println(fmt!("%d: %s%s%s", counter, cmd.command, tabs, cmd.timestamp.rfc822()));
			counter += 1;
		}
	}

	pub fn clearhistory(&mut self) {
		self.history = ~[]
	}
}
