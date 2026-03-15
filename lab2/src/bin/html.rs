use rayon::prelude::*;

fn process_file(path: &std::path::PathBuf, pattern: &regex::Regex) -> std::collections::HashMap<String, usize> {
    let mut counts = std::collections::HashMap::new();
    let content = std::fs::read_to_string(path).unwrap();
    for cap in pattern.captures_iter(&content) {
        let tag = cap[1].to_lowercase();
        *counts.entry(tag).or_insert(0) += 1;
    }
    counts
}

fn merge_maps(mut map1: std::collections::HashMap<String, usize>, map2: std::collections::HashMap<String, usize>) -> std::collections::HashMap<String, usize> {
    for (key, val) in map2 {
        *map1.entry(key).or_insert(0) += val;
    }
    map1
}

fn run_sequential(paths: &[std::path::PathBuf], pattern: &regex::Regex) -> std::collections::HashMap<String, usize> {
    let mut total_counts = std::collections::HashMap::new();
    for path in paths {
        let file_counts = process_file(path, pattern);
        total_counts = merge_maps(total_counts, file_counts);
    }
    total_counts
}

fn run_map_reduce(paths: &[std::path::PathBuf], pattern: &regex::Regex) -> std::collections::HashMap<String, usize> {
    paths.par_iter()
        .map(|path| process_file(path, pattern))
        .reduce( || std::collections::HashMap::new(), |map1, map2| merge_maps(map1, map2))
}

fn run_fork_join(paths: &[std::path::PathBuf], pattern: &regex::Regex) -> std::collections::HashMap<String, usize> {
    if paths.len() <= 8 {
        return run_sequential(paths, pattern);
    }
    let mid = paths.len() / 2;
    let (left, right) = paths.split_at(mid);
    let (left_result, right_result) = rayon::join(
        || run_fork_join(left, pattern),
        || run_fork_join(right, pattern)
    );
    merge_maps(left_result, right_result)
}

fn run_worker_pool(paths: &[std::path::PathBuf], num_workers: usize, pattern: &regex::Regex) -> std::collections::HashMap<String, usize> {
    let (job_tx, job_rx) = std::sync::mpsc::channel::<std::path::PathBuf>();
    let (result_tx, result_rx) = std::sync::mpsc::channel::<std::collections::HashMap<String, usize>>();

    let job_rx = std::sync::Arc::new(std::sync::Mutex::new(job_rx));

    std::thread::scope(|s| {
        for _ in 0..num_workers {
            let rx = std::sync::Arc::clone(&job_rx);
            let tx = result_tx.clone();
            
            s.spawn(move || {
                loop {
                    let path = {
                        let lock = rx.lock().unwrap();
                        match lock.recv() {
                            Ok(p) => p,
                            Err(_) => break,
                        }
                    };
                    
                    let counts = process_file(&path, pattern);
                    tx.send(counts).unwrap();
                }
            });
        }

        std::mem::drop(result_tx); 

        for path in paths {
            job_tx.send(path.clone()).unwrap();
        }
        
        std::mem::drop(job_tx);

        let mut total_counts = std::collections::HashMap::new();
        
        for result in result_rx {
            total_counts = merge_maps(total_counts, result);
        }
        
        total_counts
    })
}

use std::io::Write;

fn main() {
    let pattern = regex::Regex::new(r"<([a-zA-Z0-9]+)[^>]*>").unwrap();
    let paths = std::fs::read_dir("build/wikipedia").unwrap().map(|f| f.unwrap().path()).collect::<Vec<_>>();
    let mut results = std::fs::File::create("build/html.csv").unwrap();

    let start = std::time::Instant::now();
    let res = run_sequential(&paths, &pattern);

    writeln!(results, "sequential,{},{}", start.elapsed().as_secs_f64(), res.len()).unwrap();

    let start = std::time::Instant::now();
    let res = run_map_reduce(&paths, &pattern);
    writeln!(results, "map-reduce,{},{}", start.elapsed().as_secs_f64(), res.len()).unwrap();

    let start = std::time::Instant::now();
    let res = run_fork_join(&paths, &pattern);
    writeln!(results, "fork-join,{},{}", start.elapsed().as_secs_f64(), res.len()).unwrap();

    let start = std::time::Instant::now();
    let num_cpus = std::thread::available_parallelism().unwrap().get();
    let res = run_worker_pool(&paths, num_cpus, &pattern);
    writeln!(results, "worker-pool,{},{}", start.elapsed().as_secs_f64(), res.len()).unwrap();
}
