use std::env;
use std::process;
use std::fs;
use std::io;

fn buildstub(name: String){
	
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
	println!("Usage: rustyvault <payload>");
	process::exit(1);
    }
    println!("Basic rust packer 0x1337");
    let payload = args[1];
    let packed = format!("{}_packed.exe", payload);
}
