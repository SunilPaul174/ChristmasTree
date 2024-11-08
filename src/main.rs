use colored::{Colorize, CustomColor};
use rand::{random, seq::SliceRandom, thread_rng};
use std::{env, io, thread::sleep, time::Duration};
use terminal_size::{terminal_size, Height, Width};
pub static FRONT_CHARS: &[&str] = &["£", "٥", "o", "?", "J"];
pub static BACK_CHARS: &[&str] = &["3", "٥", "o", "?", "J"];
pub static MIDDLE_CHARS: &[&str] = &["٥", "o", "?", "J"];
pub static SET: [u8; 16] = [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 3, 4];

fn input_parameters() -> (usize, usize, f64) {
        let mut final_len;
        let mut padding;
        let mut waittime;
        let mut args = env::args();
        args.next();

        if let Some((Width(term_width), Height(term_height))) = terminal_size() {
                final_len = (((15 * term_height) / 16) - 1) as usize;
                padding = (term_width as usize - 1 - final_len) / 2;
                waittime = 1.0;

                if let Some(arg) = args.next() {
                        if arg.contains("h") {
                                println!("Enter command line arguments in this order: time interval, the width of the tree, and size padding on the left and right\nall are optional, but if you want to provide width or padding, the ones before in the list also have to be provided\n(eg, to provide width, you need to also give the argument time interval)");
                                sleep(Duration::from_secs(10));
                                return (15, 15, 1.0);
                        }
                        waittime = arg.trim().parse().unwrap_or(waittime);
                }
                if let Some(arg) = args.next() {
                        final_len = arg.trim().parse().unwrap_or(final_len);
                }
                if let Some(arg) = args.next() {
                        padding = arg.trim().parse().unwrap_or(padding);
                }

                (final_len, padding, waittime)
        } else {
                println!("enter width");
                let mut width = String::new();
                io::stdin().read_line(&mut width).unwrap();
                println!();

                println!("enter padding");
                let mut padding_buf = String::new();
                io::stdin().read_line(&mut padding_buf).unwrap();
                println!();

                final_len = (width.trim_end().parse::<usize>().unwrap() * 2) + 1;
                padding = padding_buf.trim_end().parse().unwrap();

                println!("enter wait time in seconds (decimals allowed)");
                let mut waittime_buf = String::new();
                io::stdin().read_line(&mut waittime_buf).unwrap();
                waittime = waittime_buf.trim().parse().unwrap();
                (final_len, padding, waittime)
        }
}

pub static BATCH: &[CustomColor] = &[
        CustomColor { r: 255, g: 0, b: 0 },
        CustomColor { r: 255, g: 191, b: 0 },
        CustomColor { r: 255, g: 255, b: 0 },
        CustomColor { r: 0, g: 255, b: 0 },
        CustomColor { r: 0, g: 0, b: 255 },
        CustomColor { r: 255, g: 0, b: 255 },
        CustomColor { r: 0, g: 255, b: 255 },
        CustomColor { r: 255, g: 255, b: 255 },
        CustomColor { r: 227, g: 154, b: 255 },
];

fn random_bright_color() -> CustomColor {
        *BATCH.choose(&mut thread_rng()).unwrap()
}

fn random_front_shape() -> &'static str {
        FRONT_CHARS.choose(&mut thread_rng()).unwrap()
}
fn random_back_shape() -> &'static str {
        BACK_CHARS.choose(&mut thread_rng()).unwrap()
}
fn random_middle_shape() -> &'static str {
        MIDDLE_CHARS.choose(&mut thread_rng()).unwrap()
}

fn body(prev: bool) -> bool {
        let temp = SET.choose(&mut thread_rng()).unwrap() == &4;
        if temp && (!prev) {
                let temp = random_middle_shape();
                print!("{}", temp);
                true
        } else if temp | (SET.choose(&mut thread_rng()).unwrap() == &3) {
                print!("{}", "@".custom_color(CustomColor { r: 136, g: 91, b: 28 }));
                false
        } else if SET.choose(&mut thread_rng()).unwrap() == &2 {
                print!("{}", "@".white());
                false
        } else {
                print!("{}", "@".green());
                false
        }
}

fn main_loop(final_len: usize, padding: usize) {
        let pad = ((final_len - 1) / 2) + padding;
        for _ in 0..(pad - 1) {
                empty_space();
        }

        print!("⭐");
        for _ in 0..(pad - 1) {
                empty_space();
        }
        println!();

        for i in 2..=final_len {
                if (i % 15) == 0 {
                        let filled = i - 4;
                        let unfilled_one_side = (((final_len - filled) / 2) - 1) + padding;
                        let filled_one_side = filled / 2;

                        for _ in 0..unfilled_one_side {
                                empty_space();
                        }
                        for _ in 0..filled_one_side {
                                print!("{}", "_".white());
                        }

                        print!("{}", "| |".custom_color(CustomColor { r: 136, g: 91, b: 28 }));

                        for _ in 0..filled_one_side {
                                print!("{}", "_".white());
                        }
                        for _ in 0..unfilled_one_side {
                                empty_space();
                        }

                        println!();
                        continue;
                }

                if (i % 2) == 1 && i != 3 {
                        let len = i.saturating_sub(3);
                        let pad = ((final_len - len) / 2) + padding;
                        let mut start = 0;
                        if ((i - 1) % 4) == 0 {
                                start = 1;
                        }
                        for _ in 0..(pad - start) {
                                empty_space();
                        }
                        if (start == 1) && (i != final_len) {
                                print!("{}", random_front_shape().custom_color(random_bright_color()))
                        };
                        let mut prev = true;
                        for _ in 0..len {
                                prev = body(prev);
                        }
                        let mut start = 0;
                        if ((i - 1) % 4) != 0 {
                                start = 1
                        }
                        if (start == 1) && (i != final_len) {
                                print!("{}", random_back_shape().custom_color(random_bright_color()))
                        };

                        println!();
                        continue;
                }

                let pad = ((final_len - i) / 2) + padding;
                for _ in 0..pad {
                        empty_space();
                }
                print!("{}", "/".white());
                for _ in 1..(i - 1) {
                        print!("{}", "@".green());
                }
                print!("{}", "\\".green());
                for _ in 0..pad {
                        empty_space();
                }

                println!();
        }
        let pad = (final_len - 3) / 2;

        for _ in 0..(pad + padding) {
                empty_space()
        }

        print!("{}", "| |".custom_color(CustomColor { r: 136, g: 91, b: 28 }));
        for _ in 0..(pad + padding) {
                empty_space()
        }
        println!();

        for _ in 0..padding {
                print!("{}", "_".white());
        }

        let mut prev = false;
        for i in 0..pad {
                if ((i % [3, 4].choose(&mut thread_rng()).unwrap()) == 0) && !prev {
                        print!("{}", "◼".red());
                        prev = true;
                } else {
                        print!("{}", "_".white());
                        prev = false;
                }
        }
        print!("{}", "| |".custom_color(CustomColor { r: 136, g: 91, b: 28 }));
        let mut prev = false;
        for i in 0..pad {
                if ((i % [3, 4].choose(&mut thread_rng()).unwrap()) == 0) && !prev {
                        print!("{}", "◼".red());
                        prev = true;
                } else {
                        print!("{}", "_".white());
                        prev = false;
                }
        }
        for _ in 0..padding {
                print!("{}", "_".white());
        }
        println!();
}

fn empty_space() {
        if random::<bool>() {
                print!(" ")
        } else {
                print!("{}", "`".white());
        }
}

fn main() {
        let (final_len, padding, waittime) = input_parameters();
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        loop {
                main_loop(final_len, padding);
                sleep(Duration::from_millis((waittime * 1000.0) as u64));
                print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        }
}
