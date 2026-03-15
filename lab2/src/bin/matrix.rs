use rayon::prelude::*;

fn run_sequential(a: &[f64], b: &[f64], n: usize) -> std::vec::Vec<f64> {
    let mut c = std::vec![0.0; n * n];
    for i in 0..n {
        for j in 0..n {
            let mut sum = 0.0;
            for k in 0..n {
                sum += a[i * n + k] * b[k * n + j];
            }
            c[i * n + j] = sum;
        }
    }
    c
}

fn run_map_reduce(a: &[f64], b: &[f64], n: usize) -> std::vec::Vec<f64> {
    (0..n).into_par_iter().flat_map(|i| {
        let mut row = std::vec![0.0; n];
        for j in 0..n {
            let mut sum = 0.0;
            for k in 0..n {
                sum += a[i * n + k] * b[k * n + j];
            }
            row[j] = sum;
        }
        row
    }).collect()
}

fn fork_join_compute(a: &[f64], b: &[f64], n: usize, row_start: usize, row_end: usize) -> std::vec::Vec<f64> {
    if row_end - row_start <= 16 {
        let mut chunk = std::vec![0.0; (row_end - row_start) * n];
        for i in row_start..row_end {
            for j in 0..n {
                let mut sum = 0.0;
                for k in 0..n {
                    sum += a[i * n + k] * b[k * n + j];
                }
                chunk[(i - row_start) * n + j] = sum;
            }
        }
        return chunk;
    }

    let mid = row_start + (row_end - row_start) / 2;
    let (mut left, right) = rayon::join(
        || fork_join_compute(a, b, n, row_start, mid),
        || fork_join_compute(a, b, n, mid, row_end)
    );
    
    left.extend(right);
    left
}

fn run_fork_join(a: &[f64], b: &[f64], n: usize) -> std::vec::Vec<f64> {
    fork_join_compute(a, b, n, 0, n)
}

fn run_worker_pool(a: &[f64], b: &[f64], n: usize, num_workers: usize) -> std::vec::Vec<f64> {
    let (job_tx, job_rx) = std::sync::mpsc::channel::<usize>();
    let (result_tx, result_rx) = std::sync::mpsc::channel::<(usize, std::vec::Vec<f64>)>();

    let job_rx = std::sync::Arc::new(std::sync::Mutex::new(job_rx));

    std::thread::scope(|s| {
        for _ in 0..num_workers {
            let rx = std::sync::Arc::clone(&job_rx);
            let tx = result_tx.clone();
            
            s.spawn(move || {
                loop {
                    let row_idx = {
                        let lock = rx.lock().unwrap();
                        match lock.recv() {
                            Ok(idx) => idx,
                            Err(_) => break,
                        }
                    };
                    
                    let mut row = std::vec![0.0; n];
                    for j in 0..n {
                        let mut sum = 0.0;
                        for k in 0..n {
                            sum += a[row_idx * n + k] * b[k * n + j];
                        }
                        row[j] = sum;
                    }
                    tx.send((row_idx, row)).unwrap();
                }
            });
        }

        std::mem::drop(result_tx);

        for i in 0..n {
            job_tx.send(i).unwrap();
        }
        
        std::mem::drop(job_tx);

        let mut c = std::vec![0.0; n * n];
        for (row_idx, row_data) in result_rx {
            let start = row_idx * n;
            c[start..start + n].copy_from_slice(&row_data);
        }
        
        c
    })
}

fn main() {
    let n = 3000; 
    let size = n * n;
    let mut a = std::vec::Vec::with_capacity(size);
    let mut b = std::vec::Vec::with_capacity(size);
    let mut val: f64 = 0.5;
    
    for _ in 0..size {
        val = (val * 1.523).fract(); 
        a.push(val * 10.0);
        
        val = (val * 1.523).fract();
        b.push(val * 10.0);
    }

    std::fs::create_dir_all("build").unwrap();
    let mut results = std::fs::File::create("build/matrix.csv").unwrap();

    let start = std::time::Instant::now();
    let res_seq = run_sequential(&a, &b, n);
    let sum_seq: f64 = res_seq.iter().sum();
    std::io::Write::write_fmt(&mut results, format_args!("sequential,{},{:.2}\n", start.elapsed().as_secs_f64(), sum_seq)).unwrap();

    let start = std::time::Instant::now();
    let res_mr = run_map_reduce(&a, &b, n);
    let sum_mr: f64 = res_mr.iter().sum();
    std::io::Write::write_fmt(&mut results, format_args!("map-reduce,{},{:.2}\n", start.elapsed().as_secs_f64(), sum_mr)).unwrap();

    let start = std::time::Instant::now();
    let res_fj = run_fork_join(&a, &b, n);
    let sum_fj: f64 = res_fj.iter().sum();
    std::io::Write::write_fmt(&mut results, format_args!("fork-join,{},{:.2}\n", start.elapsed().as_secs_f64(), sum_fj)).unwrap();

    let start = std::time::Instant::now();
    let num_cpus = std::thread::available_parallelism().unwrap().get();
    let res_wp = run_worker_pool(&a, &b, n, num_cpus);
    let sum_wp: f64 = res_wp.iter().sum();
    std::io::Write::write_fmt(&mut results, format_args!("worker-pool,{},{:.2}\n", start.elapsed().as_secs_f64(), sum_wp)).unwrap();
}
