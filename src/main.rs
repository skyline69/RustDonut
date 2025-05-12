use rayon::prelude::*;
use std::{io::Write, sync::Mutex, thread, time};

const WIDTH: usize = 80;
const HEIGHT: usize = 22;
const BUFFER_SIZE: usize = WIDTH * HEIGHT;
const SPEED: f64 = 0.8; // Controls rotation speed: higher = faster, lower = slower

// ANSI color codes for a vibrant rainbow effect
const COLORS: [&str; 6] = [
    "\x1B[31m", // Red
    "\x1B[33m", // Yellow
    "\x1B[32m", // Green
    "\x1B[36m", // Cyan
    "\x1B[34m", // Blue
    "\x1B[35m", // Magenta
];
const COLOR_RESET: &str = "\x1B[0m";

#[derive(Clone, Copy)]
struct ColoredChar {
    ch: char,
    color_index: usize,
}

impl Default for ColoredChar {
    fn default() -> Self {
        Self {
            ch: ' ',
            color_index: 0,
        }
    }
}

fn main() {
    let mut a = 0.0_f64;
    let mut b = 0.0_f64;

    let shades = ['.', ',', '-', '~', ':', ';', '=', '!', '*', '#', '$', '@'];

    let output_buffer = Mutex::new(vec![ColoredChar::default(); BUFFER_SIZE]);
    let z_buffer = Mutex::new(vec![0.0_f64; BUFFER_SIZE]);

    // Clear terminal and hide cursor
    print!("\x1B[2J\x1B[?25l");
    std::io::stdout().flush().unwrap();

    loop {
        let frame_start = std::time::Instant::now();

        {
            let mut output = output_buffer.lock().unwrap();
            let mut z = z_buffer.lock().unwrap();
            for i in 0..BUFFER_SIZE {
                output[i] = ColoredChar::default();
                z[i] = 0.0;
            }
        }

        // Pre-calculate rotation values for performance
        let sina = a.sin();
        let cosa = a.cos();
        let sinb = b.sin();
        let cosb = b.cos();

        // Process donut in parallel with improved chunking
        (0..628).into_par_iter().step_by(5).for_each(|j| {
            // Thread-local buffers to reduce lock contention
            let mut local_output = vec![ColoredChar::default(); BUFFER_SIZE];
            let mut local_z = vec![0.0; BUFFER_SIZE];

            let j_rad = j as f64 / 100.0;
            let sinj = j_rad.sin();
            let cosj = j_rad.cos();
            let cosj2 = cosj + 2.0;

            for i in (0..628).step_by(2) {
                let i_rad = i as f64 / 100.0;
                let sini = i_rad.sin();
                let cosi = i_rad.cos();

                // 3D calculations for the torus
                let m = 1.0 / (sini * cosj2 * sina + sinj * cosa + 5.0);
                let t = sini * cosj2 * cosa - sinj * sina;

                // 2D projection to screen coordinates
                let x = (40.0 + 30.0 * m * (cosi * cosj2 * cosb - t * sinb)) as usize;
                let y = (12.0 + 15.0 * m * (cosi * cosj2 * sinb + t * cosb)) as usize;

                if y < HEIGHT && x < WIDTH {
                    let pos = x + WIDTH * y;

                    // Calculate lighting/normal value for shading
                    let n = 8.0
                        * ((sinj * sina - sini * cosj * cosa) * cosb
                            - sini * cosj * sina
                            - sinj * cosa
                            - cosi * cosj * sinb);

                    // Only update if this point is closer to viewer (z-buffer algorithm)
                    if m > local_z[pos] {
                        local_z[pos] = m;

                        // Map normal to shade character
                        let n_idx =
                            ((n + 12.0) / 24.0 * shades.len() as f64) as usize % shades.len();

                        // Dynamic color based on position and rotation for rainbow effect
                        let color_idx = ((i + j + (a * 30.0) as usize) / 100) % COLORS.len();

                        local_output[pos] = ColoredChar {
                            ch: shades[n_idx],
                            color_index: color_idx,
                        };
                    }
                }
            }

            // Merge thread-local buffer into shared buffer
            let mut output = output_buffer.lock().unwrap();
            let mut z = z_buffer.lock().unwrap();

            for pos in 0..BUFFER_SIZE {
                if local_z[pos] > z[pos] {
                    z[pos] = local_z[pos];
                    output[pos] = local_output[pos];
                }
            }
        });

        // Build the entire frame string before printing (more efficient)
        {
            let output = output_buffer.lock().unwrap();
            let mut frame = String::with_capacity(BUFFER_SIZE * 10);

            // Move cursor to home position
            frame.push_str("\x1B[H");

            let mut current_color = usize::MAX; // Invalid color index to force initial color set

            for y in 0..HEIGHT {
                for x in 0..WIDTH {
                    let pos = x + WIDTH * y;
                    let colored_char = output[pos];

                    // Only update color code when necessary to minimize escape sequences
                    if colored_char.ch != ' ' {
                        if colored_char.color_index != current_color {
                            frame.push_str(COLORS[colored_char.color_index]);
                            current_color = colored_char.color_index;
                        }
                    } else if current_color != usize::MAX {
                        frame.push_str(COLOR_RESET);
                        current_color = usize::MAX;
                    }

                    frame.push(colored_char.ch);
                }
                frame.push('\n');
            }

            print!("{}", frame);
        }

        a += 0.05 * SPEED;
        b += 0.03 * SPEED;

        // Cap frame rate for consistent animation
        let elapsed = frame_start.elapsed();
        if elapsed < time::Duration::from_millis(8) {
            thread::sleep(time::Duration::from_millis(8) - elapsed);
        }

        std::io::stdout().flush().unwrap();
    }
}
