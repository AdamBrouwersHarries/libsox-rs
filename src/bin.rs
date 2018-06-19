#![allow(dead_code)]

extern crate libsox;
extern crate time;

use libsox::*;
use std::path::Path;
use time::PreciseTime;

pub fn main() {
    let lib_times = bench_lib(30);
    let cmd_times = bench_cmd(30);
    println!("Lib: {}", stat(lib_times));
    println!("Cmd: {}", stat(cmd_times));
}



fn read_from_sox_command() -> Vec<f32> {
    let cargo_root = env!("CARGO_MANIFEST_DIR");
    let d = Path::new(cargo_root).join("data");

    let mp3 = Path::new(&d).join("test.mp3");
    let dat = Path::new(&d).join("test.txt");
    return run_sox_and_read_file(mp3.as_path(), dat.as_path());
}

fn read_from_sox_library() -> Vec<f32> {
    let cargo_root = env!("CARGO_MANIFEST_DIR");
    let d = Path::new(cargo_root).join("data");

    let mp3 = Path::new(&d).join("test.mp3");
    return read_audio_file(mp3.as_path()).unwrap();
}

fn stat(ts: Vec<f64>) -> String {
    format!("Mean: {:?} of {} runs", ts.iter().sum::<f64>() / (ts.len() as f64), ts.len())
}

fn bench_lib(n: u32) -> Vec<f64> 
{
    let mut runs: Vec<f64> = Vec::new();
    // runs.reserve(n);
    for i in 0..n {
        let (_, t) = time(|| {
            read_from_sox_library()
        });
        println!("lib run {} took {} s", i, t);
        runs.push(t);
    }
    return runs;
}

fn bench_cmd(n: u32) -> Vec<f64> 
{
    let mut runs: Vec<f64> = Vec::new();
    // runs.reserve(n);
    for i in 0..n {
        let (_, t) = time(|| {
            read_from_sox_command()
        });
        println!("Cmd run {} took {} s", i, t);
        runs.push(t);
    }
    return runs;
}


// Timing function from : http://seenaburns.com/benchmarking-rust-with-cargo-bench/
pub fn time<F, T>(f: F) -> (T, f64)
where
    F: FnOnce() -> T,
{
    let start = PreciseTime::now();
    let res = f();
    let end = PreciseTime::now();

    let runtime_nanos = start
        .to(end)
        .num_nanoseconds()
        .expect("Benchmark iter took greater than 2^63 nanoseconds");
    let runtime_secs = runtime_nanos as f64 / 1_000_000_000.0;
    (res, runtime_secs)
}
