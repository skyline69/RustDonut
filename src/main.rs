use rayon::prelude::*;
use std::{sync::Mutex, time, thread};


fn main() {
    let mut a = 0.0_f64;
    let mut b = 0.0_f64;

    let q: Mutex<Vec<char>> = Mutex::new(vec![' '; 1760]);
    let z: Mutex<Vec<f64>> = Mutex::new(vec![0.0_f64; 1760]);


    let cos_table = (0..628).map(|i| f64::cos(i as f64 * 0.01)).collect::<Vec<f64>>();
    let sin_table = (0..628).map(|i| f64::sin(i as f64 * 0.01)).collect::<Vec<f64>>();
    let shades = "▁▂▂▃▄▄▅▆▆▇██";

    loop {
        a += 0.07;
        b += 0.03;

        let g = a.sin_cos();
        let m = b.sin_cos();

        q.lock().unwrap().fill(' ');

        for z_elem in z.lock().unwrap().iter_mut() {
            *z_elem = 0.0_f64;
        }

        (0..628).into_par_iter().filter(|&j| j % 10 == 0).for_each(|j| {
            let v = sin_table[j] + 2.0;
            let u = cos_table[j];
            for i in 0..628 {
                let w = cos_table[i];
                let c = sin_table[i];
                let h = w * v;

                let d = 1.0 / (h * g.1 + u * m.1 + 5.0);
                let t = h * m.0 - u * g.0;

                let x = (40.0 + 30.0 * d * (c * v * m.0 - t * g.0)) as usize;
                let y = (12.0 + 15.0 * d * (c * v * g.0 + t * m.0)) as usize;

                let o = x + 80 * y;

                if y < 22 && x < 79 {
                    let n = 8.0 * ((u * g.1 - w * sin_table[j] * m.1) * m.0 - w * sin_table[j] * g.1 - u * m.1 - c * sin_table[j] * g.0);

                    let mut z = z.lock().unwrap();
                    let mut q = q.lock().unwrap();
                    if o < z.len() && d > z[o] {
                        z[o] = d;
                        let shade_char = unsafe { shades.char_indices().nth_unchecked(n as usize).1 };
                        q[o] = shade_char;
                    }
                }
            }
        });

        print!("\x1B[H");
        {
            let q = q.lock().unwrap();
            for chunk in q.chunks(80) {
                let s: String = chunk.iter().collect();
                println!("{}", s);
            }
        }
        thread::sleep(time::Duration::from_millis(16));
    }
}

trait UncheckedIndexExt<T> {
    unsafe fn nth_unchecked(&self, index: usize) -> T;
}

impl<'a> UncheckedIndexExt<(usize, char)> for std::str::CharIndices<'a> {
    unsafe fn nth_unchecked(&self, index: usize) -> (usize, char) {
        let mut iter = self.clone();
        for _ in 0..index {
            iter.next();
        }
        iter.next().unwrap()
    }
}