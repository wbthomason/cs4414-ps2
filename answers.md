Title: Problem Set 2 Answers
Authors: Wil Thomason, Zeming Lin

1. Wil: You can run "ps auxf" to display an ASCII art representation of the parent-child relationships between processes in a tree structure. What I found interesting about this is that it seems that the resources being used by a parent process include only those resources being used by exactly that process, and not by its children.

Zeming: Using ps aux, I saw a few "users" of Ubuntu that turns out to be the message bus daemon and the error  reporting daemon by Ubuntu. They had cute names like "whoopsie" and "102", which turns out to be the user id because the actual username, "messagebus" is too long for ps aux to display using its default settings.

2. Wil: I find it interesting that even while compiling rust 0.8, with rustc using 100% of cpu, the %cpu for user is only about 25. Given that I have four cores, this seems to indicate that only one is being fully utilized. While I understand that this is due to the way the application in question (here, rustc) was written, it seems as though my other cores are largely lying underutilized, and I would be interested in knowing if there's a way to design an OS capable of using them more effectively, essentially parallelizing code written for a single core. 

Zeming: The VIRT column is how much memory the process has access to in aggregate, including RAM, hard drive memory, GPU memory, etc. It sometimes hits more RAM than my computer has, in which case I seem to slow down a little while my computer does what I assume is switching to use swap. The RES column shows how much RAM the process has access to, and corresponds fairly well with the %MEM column. RES never goes above my computers RAM value, and the %MEM column doesn't go above 1 in total. SHR is how much memory the process shares with other processes. This usually is a very small value, because not many processes need to share memory with other processes.

3. Wil: cat *.rs | grep fn | wc -l > fns.txt
This counts the number of functions in rust files in a directory and outputs them to a file. Useful for looking at fragmentation and growth of code.

Zeming: tr -s "\n" < gash.rs | wc -l | figlet

gives you an awesome ascii art printout of the number of non-newline lines in your file, which is "gash.rs" in this example.


