use colored::{Colorize, CustomColor};
use rand::{random, seq::SliceRandom, thread_rng};
use std::{io, thread::sleep, time::Duration};
pub static FRONT_CHARS: &[&str] = &["£", "٥", "o", "?", "J"];
pub static BACK_CHARS: &[&str] = &["3", "٥", "o", "?", "J"];
pub static MIDDLE_CHARS: &[&str] = &["٥", "o", "?", "J"];
pub static SET: [u8; 16] = [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 3, 4];

fn input_width() -> usize {
        println!("enter width");
        let mut string = String::new();
        io::stdin().read_line(&mut string).unwrap();

        string.trim_end().parse().unwrap()
}

fn input_padding() -> usize {
        println!("enter padding");
        let mut string = String::new();
        io::stdin().read_line(&mut string).unwrap();

        string.trim_end().parse().unwrap()
}

fn input_waittime() -> f64 {
        println!("enter wait time in seconds (decimals allowed)");
        let mut string = String::new();
        io::stdin().read_line(&mut string).unwrap();

        string.trim_end().parse().unwrap()
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

fn main_loop(width: usize, padding: usize) {
        let final_len = (width * 2) + 1;
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
        let padding = input_padding();
        let width = input_width();
        let waittime = input_waittime();
        print!("{esc}c", esc = 27 as char);
        loop {
                main_loop(width, padding);
                sleep(Duration::from_millis((waittime * 1000.0) as u64));
                print!("{esc}c", esc = 27 as char);
        }
}
