use std::{env, process};

use crate::utils::minify_bufname;

mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage: {} <bufname> <comma_separated_buflist>", args[0]);
        process::exit(1);
    }

    let bufname = &args[1];
    let buflist = args[2]
        .split(',')
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    print!("{}", minify_bufname(bufname, &buflist));
}
