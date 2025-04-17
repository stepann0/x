use std::io;
use std::panic;
use std::{env, num::ParseIntError};

#[derive(Debug, PartialEq)]
struct Config {
    num: i64,
    base: u32,
    width: u32,
}

fn usage() -> String {
    "Usage: x number[.base][.width]
    number can be binary, octal, decimal or hexademical
    [base]: 
        b|bin| 2| (default)
        o|oct| 8|
        d|dec|10|
        x|hex|16|
    [width]: valid positive integer

$ x 0xc0de.b.20
0b001100000011011110

Options:
    -h --help Show this page
"
    .to_string()
}

fn main() {
    // Turn off standard panic!() behavior and make it print usage() with error message
    panic::set_hook(Box::new(|panic_info| {
        if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
            println!("{s}\n{}", usage());
        } else if let Some(s) = panic_info.payload().downcast_ref::<String>() {
            println!("{s}\n{}", usage());
        } else {
            println!("{}", usage())
        }
    }));

    let args: Vec<String> = env::args().collect();
    let expr: String = match args.get(1) {
        Some(val) => val.clone(),
        None => panic!(""),
    };
    match expr.as_ref() {
        "-h" | "--help" => {
            panic!("")
        }
        _ => do_stuff(expr),
    }
}

fn do_stuff(expr: String) {
    // stdin case
    if expr.starts_with(".") {
        let lines = io::stdin().lines();
        for line in lines {
            let input = line.unwrap();
            let full_expr = format!("{input}{expr}");
            println!("{}", convert(parse_expr(full_expr)));
        }
        return;
    }
    println!("{}", convert(parse_expr(expr)));
}

fn parse_expr(expr: String) -> Config {
    let parts: Vec<&str> = expr.split(".").collect();
    let mut conf = Config {
        num: 0,
        base: 2,
        width: 0,
    };

    assert!(!parts.is_empty());
    conf.num = parse_number(parts[0]);

    if parts.len() > 1 {
        conf.base = parse_base(parts[1]);
    }
    if parts.len() > 2 {
        conf.width = match parts[2].parse::<u32>() {
            Ok(n) => n,
            Err(_) => panic!("Error: Can not parse width"),
        }
    }
    conf
}

fn try_parse(num: &str, pref: &str, radix: u32) -> Result<i64, ParseIntError> {
    let no_prefix = num.strip_prefix(pref).unwrap_or("");
    i64::from_str_radix(no_prefix, radix)
}

fn parse_number(num: &str) -> i64 {
    for pair in [("0x", 16), ("0b", 2), ("", 10), ("0o", 8)] {
        if let Ok(n) = try_parse(num, pair.0, pair.1) {
            return n;
        }
    }
    panic!("Error: Can not parse number `{}`", num);
}

fn parse_base(base: &str) -> u32 {
    match base {
        "b" | "bin" | "2" => 2,
        "o" | "oct" | "8" => 8,
        "d" | "dec" | "10" => 10,
        "x" | "hex" | "16" => 16,
        _ => panic!("Error: Unknown base {}", base),
    }
}

fn convert(cfg: Config) -> String {
    match cfg.base {
        2 => format!("{:#0width$b}", cfg.num, width = cfg.width as usize),
        8 => format!("{:#0width$o}", cfg.num, width = cfg.width as usize),
        10 => format!("{:#0width$}", cfg.num, width = cfg.width as usize),
        16 => format!("{:#0width$x}", cfg.num, width = cfg.width as usize),
        _ => panic!("Error: Unknown base {}", cfg.base),
    }
}

#[cfg(test)]
mod tests {
    use crate::{Config, convert, parse_expr};

    #[test]
    fn parse() {
        assert_eq!(
            parse_expr("0x3333333333333333.10".to_string()),
            Config {
                num: 3689348814741910323,
                base: 10,
                width: 0
            }
        );
        assert_eq!(
            parse_expr("0xf0f0f0f0f0f0f0f".to_string()),
            Config {
                num: 1085102592571150095,
                base: 2,
                width: 0
            }
        );
        assert_eq!(
            parse_expr("0xff00ff00ff00ff.o.32".to_string()),
            Config {
                num: 71777214294589695,
                base: 8,
                width: 32
            }
        );
        assert_eq!(
            parse_expr("0b111000111000111.d".to_string()),
            Config {
                num: 29127,
                base: 10,
                width: 0
            }
        );
        assert_eq!(
            parse_expr("0b1011001110011001011001.o.25".to_string()),
            Config {
                num: 2942553,
                base: 8,
                width: 25
            }
        );
        assert_eq!(
            parse_expr("0b101101111001010111".to_string()),
            Config {
                num: 187991,
                base: 2,
                width: 0
            }
        );
        assert_eq!(
            parse_expr("0o1726354.dec.33".to_string()),
            Config {
                num: 503020,
                base: 10,
                width: 33
            }
        );
        assert_eq!(
            parse_expr("0o233323332.2".to_string()),
            Config {
                num: 40740570,
                base: 2,
                width: 0
            }
        );
    }

    #[test]
    fn convertion() {
        assert_eq!(
            convert(parse_expr("0x5555555555555555.dec".to_string())),
            "6148914691236517205"
        );
        assert_eq!(
            convert(parse_expr("0xffff0000ffff.bin.64".to_string())),
            "0b00000000000000111111111111111100000000000000001111111111111111"
        );
        assert_eq!(
            convert(parse_expr("0b11000000111111.hex".to_string())),
            "0x303f"
        );
        assert_eq!(
            convert(parse_expr("0o555555555.x.16".to_string())),
            "0x00000005b6db6d"
        );
        assert_eq!(
            convert(parse_expr("28384922342543.b.64".to_string())),
            "0b00000000000000000110011101000011100001001101101001010010001111"
        );
        assert_eq!(
            convert(parse_expr("0b111111000000.b.32".to_string())),
            "0b000000000000000000111111000000"
        );
        assert_eq!(
            convert(parse_expr("0o77001133440011".to_string())),
            "0b111111000000001001011011100100000000001001"
        );
    }
}
