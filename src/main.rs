#![allow(unused)]

use std::{
    env::{self, Args},
    iter::Skip,
    process::exit,
};

fn main() {
    let args = env::args();

    without_vec(args);
}

fn with_vec(args: Args) {
    let mut args = args.skip(1);

    let args = args.collect::<Vec<String>>();

    if args.is_empty() {
        eprintln!("Usage: echo <your text>");
        exit(1);
    }

    println!("{}", args.join(" "));
}

fn without_vec(mut args: Args) {
    let mut args = args.skip(1);

    if args.len() < 1 {
        eprintln!("Usage: echo <your text>");
        exit(1);
    }

    let mut text = args.next().unwrap();
    for txt in args {
        text.push(' ');
        text.push_str(&txt);
    }
    println!("{text}")
}
