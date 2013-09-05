use std::{os, float};

fn main() {
    let args: ~[~str] = os::args();
    let mut i = 1;
    let mut sum: float = 0.0;
    let mut nums: float = 0.0;
    while i < args.len() {

        let v = float::from_str(args[i]);
        match v {
            Some(argVal) => {
                sum = sum + argVal;
                nums = nums + 1.0;
            }
            None => {
                println("Bad input: " + args[i]);
            }
        }

        i = i + 1;
    }

    // assumption is that there exists at least one argument
    let avg = sum/nums;
    println("Average: " + avg.to_str());
}