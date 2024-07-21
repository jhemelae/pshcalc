use std::time::Instant;

#[inline(always)]
fn next(x: &mut [usize], n: usize) {
    for i in 0..x.len() {
        // Unsafe block to avoid bounds checks within the loop
        unsafe {
            let elem = x.get_unchecked_mut(i);
            *elem += 1;
            if *elem == n {
                *elem = 0;
            } else {
                break;
            }
        }
    }
}

#[inline(always)]
fn get(s: &[usize], n: usize, i: usize, j: usize) -> usize {
        s[n * i + j]
}

#[inline(always)]
fn is_associative(s: &[usize], n: usize) -> bool {
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                let left = get(s, n, get(s, n, i, j), k);
                let right = get(s, n, i, get(s, n, j, k));
                if left != right {
                    return false;
                }
            }
        }
    }
    true
}

fn main() {
    let start = Instant::now();
    let n: usize = 4;
    let size = n * n;
    
    let mut array = vec![0; size];

    let mut count = 0;
    for _i in 0..n.pow(size as u32) {
        if is_associative(&array, n) {
            count += 1;
        }
        next(&mut array, n);
    }
    println!("Count = {:?}", count);

    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
}
