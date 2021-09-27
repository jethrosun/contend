//! Allocate large vector with heap so we can cause memory contention.
//! https://stackoverflow.com/questions/53691012/is-there-any-way-to-allocate-a-standard-rust-array-directly-on-the-heap-skippin
use std::collections::HashMap;
use std::env;
use std::process;
use std::thread;
use std::time::{Duration, Instant};

const GB_SIZE: usize = 1073741824;

/// Map different setup to memory resource intensiveness. We are mapping setup into size of u128,
/// which is the largest size we can use
/// setup: 10GB, 20GB, 50GB
fn read_setup(setup: &usize) -> Option<usize> {
    let mut map = HashMap::new();
    map.insert(1, 10 * GB_SIZE); // 10GB
    map.insert(2, 20 * GB_SIZE); // 20GB
    map.insert(3, 30 * GB_SIZE); // 50GB

    map.remove(&setup)
}

fn main() {
    // get the list of ports from cmd args and cast into a Vec
    let params: Vec<String> = env::args().collect();

    // len of params will be number of args +1
    // if params.len() == 2 {
    //     println!("Parse 2 args");
    //     println!("Setup: {:?}", params[1],);
    // } else {
    //     println!("More or less than 1 args are provided. Run it with *setup*");
    //     process::exit(0x0100);
    // }
    // let setup = params[1].parse::<usize>().unwrap();

    // counting the iterations
    let mut counter = 0;
    let now = Instant::now();

    // read setup and translate to vector size
    // let vec_size = read_setup(&setup).unwrap() / 16;
    // let large_vec = vec![42u128; vec_size];

    // let _sleep_time = Duration::from_millis(500);
    let _sleep_time = Duration::from_millis(900);

    // loop {
    //     thread::sleep(_sleep_time);
    //     for i in 0..vec_size / 256 {
    //         let _ = large_vec[i * 256];
    //         counter += 1;
    //         // println!("current value: {:?}", t);
    //     }
    //     if counter % 1_000 == 0 {
    //         println!("{} * k since {:?}", counter, now.elapsed());
    //     }
    // }

    // const LENGTH: usize = 10_000_000;
    const LENGTH: usize = 10 * GB_SIZE / 16;

    let mut large_vec = {
        let mut v: Vec<u128> = Vec::with_capacity(LENGTH);

        // Explicitly set length which is safe since the allocation is
        // sized correctly.
        unsafe {
            v.set_len(LENGTH);
        };

        // While not required for this particular example, in general
        // we want to initialize elements to a known value.
        let mut slice = v.into_boxed_slice();
        for i in &mut slice[..] {
            *i = 42;
        }

        let raw_slice = Box::into_raw(slice);

        // Using `from_raw` is safe as long as the pointer is
        // retrieved using `into_raw`.
        unsafe { Box::from_raw(raw_slice as *mut [u128; LENGTH]) }
    };

    // This is the micro benchmark from the question.
    // for j in 0..LENGTH {
    //     for i in 0..LENGTH {
    //         a[i] = j as u128;
    //     }
    // }

    loop {
        thread::sleep(_sleep_time);
        for i in 0..LENGTH / 256 {
            let _ = large_vec[i * 256];
            counter += 1;
            // println!("current value: {:?}", t);
        }

        // if counter % 1_000 == 0 {
        //     println!("{} * k since {:?}", counter, now.elapsed());
        // }
        if counter % 100 == 0 {
            println!("{} * 100 since {:?}", counter, now.elapsed());
        }
    }
}
