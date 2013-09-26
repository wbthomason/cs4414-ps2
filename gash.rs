use std::{io, run, task, os};
use internal::*;
mod internal;

fn main() {
    static CMD_PROMPT: &'static str = "gash";
    let mut hist = internal::HistoryLog { history: ~[] };
    loop {
        print(fmt!("%s: %s > ", CMD_PROMPT, os::getcwd().to_str()));
        let line = io::stdin().read_line();
        debug!(fmt!("line: %?", line));
        let mut argv: ~[~str] = (line.split_iter(' ').filter(|&x| x != "")).map(|x: &str| x.to_owned()).collect();
        debug!(fmt!("argv %?", argv));
        hist.addhistory(line);     

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
                    match argv.shift_opt() {
                           Some(y)      => { if y == ~"-c" {
                                                hist.clearhistory(); 
                                             }
                                           }
                           _            => { hist.printhistory(); }
                       }
                }
                _           => {
                    match argv.iter().last() {
                        Some(y) if y == &~"&" => {
                            let freezeArg = argv.clone();
                            do task::spawn_supervised {
                                run::process_status(program, freezeArg.slice_to(freezeArg.len() - 1));
                            }
                        }
                        _                => { run::process_status(program, argv); }
                    }
                }
            }
        }
    }
}
