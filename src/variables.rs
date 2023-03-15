#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn variables() {

    //Varibles are defined as <variable_scope> <variable_name> : <data_type> = <value>
    const ONE_MIL : u32 = 1_000_000;
    const PI : f32 = 3.141592;
    let age : &str = "47";
    let mut age : u32 = age.trim().parse().expect("Age wasn't converted!");

    age = age + 1;

    println!("I'm {} and I have ${}", age, ONE_MIL);

}
