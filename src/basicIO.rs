//ignores errors thrown due to unused variables
#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn basicIO() {

    //Macros are identified by a '!' at the end
    println!("What is your name?");

    //Varibles defined in Rust are immutable by default
    //use the 'mut' keyword to make them mutable
    let mut name : String = String::new();

    let greeting : &str = "Nice to meet you";
    
    //takes input from the user alongside built-in error handling
    io::stdin().read_line(&mut name).expect("Didn't receive input!");

    //{} is used to place a variable inside of a print statement
    println!("Hello {}! {}!", name.trim_end(), greeting);

}