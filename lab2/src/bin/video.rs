use image::{GenericImage, Rgba, GenericImageView};
use std::io::Write;

fn apply_watermark(mut img: image::DynamicImage) -> image::DynamicImage {
    let (width, height) = img.dimensions();
    let watermark_size = 20;
    
    for x in 0..watermark_size {
        for y in 0..watermark_size {
            if x < width && y < height {
                img.put_pixel(x, y, Rgba([255, 0, 0, 255]));
            }
        }
    }
    img
}

fn generate_dummy_frames(input_dir: &str, count: usize) -> Vec<std::path::PathBuf> {
    std::fs::create_dir_all(input_dir).unwrap();
    let mut paths = Vec::new();
    
    for i in 0..count {
        let path = std::path::PathBuf::from(format!("{}/frame_{:03}.png", input_dir, i));
        let img = image::ImageBuffer::from_pixel(100, 100, Rgba([(i % 255) as u8, 100, 100, 255]));
        img.save(&path).unwrap();
        paths.push(path);
    }
    
    paths
}

fn run_producer_consumer(paths: &[std::path::PathBuf], output_dir: &str, num_workers: usize) {
    let (job_tx, job_rx) = std::sync::mpsc::channel::<std::path::PathBuf>();
    let job_rx = std::sync::Arc::new(std::sync::Mutex::new(job_rx));
    let out_dir = output_dir.to_string();

    std::thread::scope(|s| {
        for _ in 0..num_workers {
            let rx = std::sync::Arc::clone(&job_rx);
            let out = out_dir.clone();
            
            s.spawn(move || {
                loop {
                    let path = {
                        let lock = rx.lock().unwrap();
                        match lock.recv() {
                            Ok(p) => p,
                            Err(_) => break, // Breaks when job_tx is dropped
                        }
                    };
                    
                    if let Ok(img) = image::open(&path) {
                        let filtered = img.grayscale();
                        let watermarked = apply_watermark(filtered);
                        
                        let name = path.file_name().unwrap().to_string_lossy().into_owned();
                        let out_path = std::path::Path::new(&out).join(name);
                        watermarked.save(out_path).unwrap();
                    }
                }
            });
        }

        for path in paths {
            job_tx.send(path.clone()).unwrap();
        }
        
        // FIX: Tell the workers we are done sending jobs
        std::mem::drop(job_tx); 
    });
}

fn run_pipeline(paths: &[std::path::PathBuf], output_dir: &str) {
    let (tx1, rx1) = std::sync::mpsc::sync_channel::<std::path::PathBuf>(10);
    let (tx2, rx2) = std::sync::mpsc::sync_channel::<(String, image::DynamicImage)>(10);
    let (tx3, rx3) = std::sync::mpsc::sync_channel::<(String, image::DynamicImage)>(10);

    let out_dir = output_dir.to_string();

    std::thread::scope(|s| {
        s.spawn(move || {
            for path in rx1 {
                if let Ok(img) = image::open(&path) {
                    let name = path.file_name().unwrap().to_string_lossy().into_owned();
                    tx2.send((name, img)).unwrap();
                }
            }
        });

        s.spawn(move || {
            for (name, img) in rx2 {
                let filtered = img.grayscale();
                tx3.send((name, filtered)).unwrap();
            }
        });

        s.spawn(move || {
            for (name, img) in rx3 {
                let watermarked = apply_watermark(img);
                let out_path = std::path::Path::new(&out_dir).join(name);
                watermarked.save(out_path).unwrap();
            }
        });

        for path in paths {
            tx1.send(path.clone()).unwrap();
        }
        
        std::mem::drop(tx1);
    });
}

fn main() {
    let input_dir = "build/frames_in";
    let output_dir_pc = "build/frames_out_pc";
    let output_dir_pipe = "build/frames_out_pipe";

    std::fs::create_dir_all(output_dir_pc).unwrap();
    std::fs::create_dir_all(output_dir_pipe).unwrap();

    println!("generating dummy frames...");
    let paths = generate_dummy_frames(input_dir, 100000);
    let mut results = std::fs::File::create("build/image_processing.csv").unwrap();

    let num_cpus = std::thread::available_parallelism().unwrap().get();

    println!("running producer-consumer worker pool...");
    let start = std::time::Instant::now();
    run_producer_consumer(&paths, output_dir_pc, num_cpus);
    writeln!(results, "producer-consumer,{}\n", start.elapsed().as_secs_f64()).unwrap();

    println!("running 3-stage pipeline...");
    let start = std::time::Instant::now();
    run_pipeline(&paths, output_dir_pipe);
    writeln!(results, "pipeline,{}\n", start.elapsed().as_secs_f64()).unwrap();
    
    println!("done...");
}
