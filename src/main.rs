use colored::{ColoredString, Colorize, CustomColor};
use rand::{random, rngs::ThreadRng, seq::SliceRandom, thread_rng};
use std::{io, iter::repeat_n, thread::sleep, time::Duration};
pub static FRONT_CHARS: &[&str] = &["£", "٥", "o", "?", "J"];
pub static BACK_CHARS: &[&str] = &["3", "٥", "o", "?", "J"];
pub static MIDDLE_CHARS: &[&str] = &["٥", "o"];
pub static SET: [u8; 16] = [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 4];

fn input_width() -> usize {
        // println!("enter width");
        // let mut string = String::new();
        // io::stdin().read_line(&mut string).unwrap();

        // string.trim_end().parse().unwrap()
        18
}

fn random_bright_color() -> CustomColor {
        let r = random::<u8>().saturating_add(128);
        let g = random::<u8>().saturating_add(128);
        let b = random::<u8>().saturating_add(128);

        CustomColor { r, g, b }
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
        if (SET.choose(&mut thread_rng()).unwrap() == &4) && (!prev) {
                let temp = random_middle_shape();
                print!("{}", temp);
                true
        } else {
                print!("{}", "*".green());
                false
        }
}

fn main_loop(width: usize) {
        // TODO random colors
        let final_len = (width * 2) + 1;
        let pad = (final_len - 1) / 2;
        for _ in 0..(pad - 1) {
                print!(" ")
        }

        println!("⭐");

        for i in 2..=final_len {
                if (i % 15) == 0 {
                        let mut string = String::with_capacity(final_len);
                        let pad = (final_len - 3) / 2;

                        for _ in 0..pad {
                                string.push(' ');
                        }

                        string.push_str("| |");
                        println!("{}", string.custom_color(CustomColor { r: 136, g: 91, b: 28 }));
                        continue;
                }

                if (i % 2) == 1 && i != 3 {
                        let len = i.saturating_sub(3);
                        let pad = (final_len - len) / 2;
                        let mut start = 0;
                        if ((i - 1) % 4) == 0 {
                                start = 1;
                        }
                        print!("{}", repeat_n(' ', pad - start).collect::<String>());
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

                let pad = (final_len - i) / 2;
                let mut string = String::with_capacity(final_len);
                for _ in 0..pad {
                        string.push(' ');
                }

                for _ in 0..i {
                        string.push('*');
                }

                println!("{}", string.green());
        }
        let mut string = String::with_capacity(final_len);
        let pad = (final_len - 3) / 2;

        for _ in 0..pad {
                string.push(' ');
        }

        string.push_str("| |");
        println!("{}", string.custom_color(CustomColor { r: 136, g: 91, b: 28 }));
        let mut prev = false;
        for i in 0..pad {
                if ((i % [3, 4].choose(&mut thread_rng()).unwrap()) == 0) && !prev {
                        print!("{}", "◼".red());
                        prev = true;
                } else {
                        print!(" ");
                        prev = false;
                }
        }
        print!("{}", "| |".custom_color(CustomColor { r: 136, g: 91, b: 28 }));
        let mut prev = false;
        for i in 0..pad {
                if ((i % [5, 6].choose(&mut thread_rng()).unwrap()) == 0) && !prev {
                        print!("{}", "◼".red());
                        prev = true;
                } else {
                        print!(" ");
                        prev = false;
                }
        }
        println!();
}

fn main() {
        let width = input_width();
        loop {
                main_loop(width);
                sleep(Duration::from_secs(1));
                print!("{esc}c", esc = 27 as char);
        }
}
