mod cli;
mod crawler;
mod models;
mod output;

use crate::models::Page;

fn main() {
    let x: usize = 5;
    let y = soma(x);
    println!("{}", x);
}

fn soma(x: usize) -> usize {
    x + 1
}
