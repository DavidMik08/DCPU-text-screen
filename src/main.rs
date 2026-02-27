use std::env;
use std::fs;
use std::io::Write;
use std::thread;
use std::time::Duration;

fn main() {
    let home = env::var("HOME").expect("HOME not set");
    let out_path = format!(
        "{}/programming/rust/DCPU-emulator/target/debug/ports/output_port.hex",
        home
    );
    let clk_path = format!(
        "{}/programming/rust/DCPU-emulator/target/debug/ports/clk.hex",
        home
    );

    println!("Waiting for output from emulator...");
    let mut waiting_for_zero = false;

    loop {
        if let Ok(clk) = fs::read_to_string(&clk_path) {
            if clk.trim() == "01" {
                if let Ok(data) = fs::read_to_string(&out_path) {
                    let s = data.trim();
                    if s.len() >= 6 {
                        let p0 = &s[0..2];
                        let p1 = &s[2..4];
                        let p2 = &s[4..6];
                        if p0 == "02" && p1 == "00" && !waiting_for_zero {
                            if let Ok(value) = u8::from_str_radix(p2, 16) {
                                match value {
                                    0x09 => print!("\t"),
                                    0x0A => print!("\n"),
                                    0x0D => print!("\r"),
                                    0x1B => print!("\x1B[2J\x1B[H"),
                                    _ => print!("{}", value as char),
                                }

                                std::io::stdout().flush().unwrap();
                                waiting_for_zero = true;
                            }
                       } else if p0 == "00" && waiting_for_zero {
                            waiting_for_zero = false;
                        }
                    }
                }
            }
        }
        thread::sleep(Duration::from_micros(100));
    }
}
