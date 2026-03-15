use rayon::prelude::*;
use std::io::Write;

fn process_chunk(chunk: &[f64]) -> (f64, f64, f64, usize) {
    let mut min = std::f64::INFINITY;
    let mut max = std::f64::NEG_INFINITY;
    let mut sum = 0.0;
    let count = chunk.len();

    for &val in chunk {
        if val < min { min = val; }
        if val > max { max = val; }
        sum += val;
    }

    (min, max, sum, count)
}

fn merge_stats(a: (f64, f64, f64, usize), b: (f64, f64, f64, usize)) -> (f64, f64, f64, usize) {
    (
        a.0.min(b.0),
        a.1.max(b.1),
        a.2 + b.2,
        a.3 + b.3,
    )
}

fn get_median_sequential(data: &[f64]) -> f64 {
    let mut sorted = data.to_vec();
    sorted.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
    let mid = sorted.len() / 2;
    if sorted.len() % 2 == 0 {
        (sorted[mid - 1] + sorted[mid]) / 2.0
    } else {
        sorted[mid]
    }
}

fn get_median_parallel(data: &[f64]) -> f64 {
    let mut sorted = data.to_vec();
    sorted.par_sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
    let mid = sorted.len() / 2;
    if sorted.len() % 2 == 0 {
        (sorted[mid - 1] + sorted[mid]) / 2.0
    } else {
        sorted[mid]
    }
}

fn run_sequential(data: &[f64]) -> (f64, f64, f64, f64) {
    let stats = process_chunk(data);
    let mean = stats.2 / stats.3 as f64;
    let median = get_median_sequential(data);
    (stats.0, stats.1, median, mean)
}

fn run_map_reduce(data: &[f64]) -> (f64, f64, f64, f64) {
    let (min, max, sum, count) = data.par_chunks(10000)
        .map(|chunk| process_chunk(chunk))
        .reduce(|| (std::f64::INFINITY, std::f64::NEG_INFINITY, 0.0, 0), merge_stats);
    
    let mean = sum / count as f64;
    let median = get_median_parallel(data);
    (min, max, median, mean)
}

fn fork_join_stats(data: &[f64]) -> (f64, f64, f64, usize) {
    if data.len() <= 10000 {
        return process_chunk(data);
    }
    let mid = data.len() / 2;
    let (left, right) = data.split_at(mid);
    let (left_result, right_result) = rayon::join(
        || fork_join_stats(left),
        || fork_join_stats(right)
    );
    merge_stats(left_result, right_result)
}

fn run_fork_join(data: &[f64]) -> (f64, f64, f64, f64) {
    let stats = fork_join_stats(data);
    let mean = stats.2 / stats.3 as f64;
    let median = get_median_parallel(data);
    (stats.0, stats.1, median, mean)
}

fn run_worker_pool(data: &[f64], num_workers: usize) -> (f64, f64, f64, f64) {
    let (job_tx, job_rx) = std::sync::mpsc::channel::<&[f64]>();
    let (result_tx, result_rx) = std::sync::mpsc::channel::<(f64, f64, f64, usize)>();

    let job_rx = std::sync::Arc::new(std::sync::Mutex::new(job_rx));

    let final_stats = std::thread::scope(|s| {
        for _ in 0..num_workers {
            let rx = std::sync::Arc::clone(&job_rx);
            let tx = result_tx.clone();
            
            s.spawn(move || {
                loop {
                    let chunk = {
                        let lock = rx.lock().unwrap();
                        match lock.recv() {
                            Ok(c) => c,
                            Err(_) => break,
                        }
                    };
                    
                    let stats = process_chunk(chunk);
                    tx.send(stats).unwrap();
                }
            });
        }

        std::mem::drop(result_tx); 

        let chunk_size = (data.len() + num_workers - 1) / num_workers;
        for chunk in data.chunks(chunk_size) {
            job_tx.send(chunk).unwrap();
        }
        
        std::mem::drop(job_tx);

        let mut total_stats = (std::f64::INFINITY, std::f64::NEG_INFINITY, 0.0, 0);
        
        for result in result_rx {
            total_stats = merge_stats(total_stats, result);
        }
        
        total_stats
    });

    let mean = final_stats.2 / final_stats.3 as f64;
    let median = get_median_sequential(data); 
    
    (final_stats.0, final_stats.1, median, mean)
}

fn main() {
    let size = 100_000_000;
    let mut data = Vec::with_capacity(size);
    let mut val: f64 = 0.5;
    
    for _ in 0..size {
        val = (val * 1.523).fract(); 
        data.push(val * 1000.0);
    }

    std::fs::create_dir_all("build").unwrap();
    let mut results = std::fs::File::create("build/numbers.csv").unwrap();

    let start = std::time::Instant::now();
    let res_seq = run_sequential(&data);
    writeln!(results, "sequential,{},{},{},{},{}", start.elapsed().as_secs_f64(), res_seq.0, res_seq.1, res_seq.2, res_seq.3).unwrap();

    let start = std::time::Instant::now();
    let res_mr = run_map_reduce(&data);
    writeln!(results, "map-reduce,{},{},{},{},{}", start.elapsed().as_secs_f64(), res_mr.0, res_mr.1, res_mr.2, res_mr.3).unwrap();

    let start = std::time::Instant::now();
    let res_fj = run_fork_join(&data);
    writeln!(results, "fork-join,{},{},{},{},{}", start.elapsed().as_secs_f64(), res_fj.0, res_fj.1, res_fj.2, res_fj.3).unwrap();

    let start = std::time::Instant::now();
    let num_cpus = std::thread::available_parallelism().unwrap().get();
    let res_wp = run_worker_pool(&data, num_cpus);
    writeln!(results, "worker-pool,{},{},{},{},{}", start.elapsed().as_secs_f64(), res_wp.0, res_wp.1, res_wp.2, res_wp.3).unwrap();
}
