use std::{io, run};
use internal::*;
mod internal;

fn main() {
    static CMD_PROMPT: &'static str = "gash > ";
    let mut hist = internal::HistoryLog { history: ~[] };
    loop {
        print(CMD_PROMPT);
        let line = io::stdin().read_line();
        debug!(fmt!("line: %?", line));
        let mut argv: ~[~str] = (line.split_iter(' ').filter(|&x| x != "")).map(|x: &str| x.to_owned()).collect();
        debug!(fmt!("argv %?", argv));
        hist.history = hist.addhistory(line);
        if argv.len() > 0 {
            let program = argv.remove(0);
            match program {
                ~"exit"     => {return; }
                ~"cd"       => {
                    if argv.len() > 0 {
                        changedir(argv.remove(0));
                    }
                }
                ~"history"  => {
                    if argv.len() > 0 {
                        let arg = argv.remove(0); 
                        match arg {
                            ~"-c" => { hist.history = hist.clearhistory(); }
                            _     => { println(fmt!("Error: %s is not a valid option for history.", arg)); }
                        }
                    }
                    else {
                        hist.printhistory();
                    }
                }
                _           => {run::process_status(program, argv);}
            }
        }
    }
}
