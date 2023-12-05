extern crate libtest_mimic;

use std::env;
use std::{thread, time};
use libtest_mimic::{Arguments, Trial, Failed};
use log::*;


fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();


    // Get all command-line arguments, including the program name
    let mut args: Vec<String> = env::args().collect();
    let program_name = args[0].clone();
    let elf = args[1].clone(); // get elf (first positional arg).
    info!("Flashing elf file {}", elf);


    // Create an iterator from the remaining arguments, skipping the first argument
    let mut args_for_libtest_mimic = vec![program_name];
    args_for_libtest_mimic.extend( args.into_iter().skip(2));
    let args = Arguments::from_iter(args_for_libtest_mimic);

    let tests = vec![
        Trial::test("check_toph", check_toph),
        Trial::test("check_sokka", check_sokka),
        Trial::test("long_computation", long_computation).with_ignored_flag(true),
        Trial::test("foo", compile_fail_dummy).with_kind("compile-fail"),
        Trial::test("check_katara", check_katara),
    ];

    libtest_mimic::run(&args, tests).exit();
}


// Tests

fn check_toph() -> Result<(), Failed> {
    Ok(())
}
fn check_katara() -> Result<(), Failed> {
    Ok(())
}
fn check_sokka() -> Result<(), Failed> {
    Err("Sokka tripped and fell :(".into())
}
fn long_computation() -> Result<(), Failed> {
    thread::sleep(time::Duration::from_secs(1));
    Ok(())
}
fn compile_fail_dummy() -> Result<(), Failed> {
    Ok(())
}