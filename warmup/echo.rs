use std::{os};

fn main() {
    let args: ~[~str] = os::args();
    let mut i = 1;
    while i < args.len() {
        print(args[i]);
        
        if i != args.len()-1 {
            print(" ");
        } else {
            print("\n");
        }

        i = i + 1;
    }
}