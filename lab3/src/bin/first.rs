struct Prng {
    state: u64,
}

impl Prng {
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }
    
    fn next_range(&mut self, max: usize) -> usize {
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        ((self.state >> 32) as usize) % max
    }
}

fn run_race_condition(num_accounts: usize, num_threads: usize, transfers_per_thread: usize) {
    let mut prng = Prng::new(42);
    let mut accounts: std::vec::Vec<i32> = (0..num_accounts).map(|_| (prng.next_range(1000) + 100) as i32).collect();
    
    let total_before: i32 = accounts.iter().sum();
    std::println!("--- RACE CONDITION ---");
    std::println!("Before: {}", total_before);

    let ptr_addr = accounts.as_mut_ptr() as usize;

    std::thread::scope(|s| {
        for t_id in 0..num_threads {
            s.spawn(move || {
                let mut local_prng = Prng::new((t_id as u64) + 12345);
                let ptr = ptr_addr as *mut i32;
                
                for _ in 0..transfers_per_thread {
                    let from = local_prng.next_range(num_accounts);
                    let mut to = local_prng.next_range(num_accounts);
                    while from == to { to = local_prng.next_range(num_accounts); }
                    
                    let amount = (local_prng.next_range(50) + 1) as i32;

                    unsafe {
                        let from_ptr = ptr.add(from);
                        let to_ptr = ptr.add(to);

                        let current_from = std::ptr::read_volatile(from_ptr);
                        if current_from >= amount {
                            std::ptr::write_volatile(from_ptr, current_from - amount);
                            let current_to = std::ptr::read_volatile(to_ptr);
                            std::ptr::write_volatile(to_ptr, current_to + amount);
                        }
                    }
                }
            });
        }
    });

    let total_after: i32 = accounts.iter().sum();
    std::println!("After: {}", total_after);
    std::println!("Diff: {}\n", total_after - total_before);
}

fn run_resolved(num_accounts: usize, num_threads: usize, transfers_per_thread: usize) {
    let mut prng = Prng::new(42);
    let accounts: std::vec::Vec<std::sync::Mutex<i32>> = (0..num_accounts)
        .map(|_| std::sync::Mutex::new((prng.next_range(1000) + 100) as i32))
        .collect();

    let total_before: i32 = accounts.iter().map(|acc| *acc.lock().unwrap()).sum();
    std::println!("--- RESOLVED ---");
    std::println!("Before: {}", total_before);

    std::thread::scope(|s| {
        for t_id in 0..num_threads {
            let accounts_ref = &accounts; 
            s.spawn(move || {
                let mut local_prng = Prng::new((t_id as u64) + 9999);
                for _ in 0..transfers_per_thread {
                    let from = local_prng.next_range(num_accounts);
                    let mut to = local_prng.next_range(num_accounts);
                    while from == to { to = local_prng.next_range(num_accounts); }
                    
                    let amount = (local_prng.next_range(50) + 1) as i32;

                    let first = std::cmp::min(from, to);
                    let second = std::cmp::max(from, to);

                    let mut lock1 = accounts_ref[first].lock().unwrap();
                    let mut lock2 = accounts_ref[second].lock().unwrap();

                    let (from_bal, to_bal) = if from < to {
                        (&mut *lock1, &mut *lock2)
                    } else {
                        (&mut *lock2, &mut *lock1)
                    };

                    if *from_bal >= amount {
                        *from_bal -= amount;
                        *to_bal += amount;
                    }
                }
            });
        }
    });

    let total_after: i32 = accounts.iter().map(|acc| *acc.lock().unwrap()).sum();
    std::println!("After: {}", total_after);
    std::println!("Diff: {}\n", total_after - total_before);
}

fn run_deadlock(num_accounts: usize, num_threads: usize, transfers_per_thread: usize) {
    let mut prng = Prng::new(42);
    let accounts: std::vec::Vec<std::sync::Mutex<i32>> = (0..num_accounts)
        .map(|_| std::sync::Mutex::new((prng.next_range(1000) + 100) as i32))
        .collect();

    std::println!("--- DEADLOCK ---");
    std::println!("Hanging...");

    std::thread::scope(|s| {
        for t_id in 0..num_threads {
            let accounts_ref = &accounts;
            s.spawn(move || {
                let mut local_prng = Prng::new((t_id as u64) + 777);
                for _ in 0..transfers_per_thread {
                    let from = local_prng.next_range(num_accounts);
                    let mut to = local_prng.next_range(num_accounts);
                    while from == to { to = local_prng.next_range(num_accounts); }

                    let _lock_from = accounts_ref[from].lock().unwrap();
                    std::thread::yield_now(); 
                    let _lock_to = accounts_ref[to].lock().unwrap();
                }
            });
        }
    });
    
    std::println!("Done.");
}

fn main() {
    let num_accounts = 200;
    let num_threads = 1500;
    let transfers_per_thread = 1000;

    run_race_condition(num_accounts, num_threads, transfers_per_thread);
    run_resolved(num_accounts, num_threads, transfers_per_thread);
    run_deadlock(num_accounts, num_threads, transfers_per_thread);
}
