use std::collections::HashMap;

mod cpu;
mod fmt;
mod lorom;



fn main() {
    let mut args = std::env::args().skip(1);
    let rom = args.next().expect("need rom name");

    let data_map = format!("{}.map", &rom[..rom.find(".").unwrap_or(rom.len())]);
    let symbols  = format!("{}.sym", &rom[..rom.find(".").unwrap_or(rom.len())]);

    let rom = std::fs::read(&rom).expect("can't read rom");
    let data_map = std::fs::read(&data_map).expect("can't read data");
    let symbols = std::fs::read(&symbols).expect("can't read symbols");

    let mut labels = HashMap::new();

    let symbols = String::from_utf8(symbols).unwrap();
    let mut iter = symbols.lines();
    while iter.next() != Some("[labels]") {};
    for i in iter.by_ref() {
        if i.starts_with("[") { break; }
        if i.trim() == "" { continue; }
        let mut iter = i.split(" ");
        let addr = iter.next().unwrap();
        let label = iter.next().unwrap();
        let addr = (u32::from_str_radix(&addr[..2], 16).unwrap() << 16) + (u32::from_str_radix(&addr[3..], 16).unwrap());
        labels.insert(addr, label.trim_start_matches(':').to_string());
    }



    let mut buf = String::new();
    for (i,map) in data_map.iter().enumerate() {
        let m_flag = map & 0x80 != 0;
        let x_flag = map & 0x40 != 0;
        let addr = lorom::to_address(i).unwrap();
        if addr & 0x7FFF == 0 { println!("org ${:06X}", addr); }
        if let Some(c) = labels.get(&addr) {
            println!("{}:", c);
        }
        match map & 0x1F {
            0x00 => println!("    db ${:02X}     ; {:06X} unk", rom[i], addr),
            0x01 => {},
            0x02 => if let Some((_,x)) = cpu::parse_instr(&rom[i..], m_flag, x_flag) {
                buf.clear();
                x.display(None, &mut buf);
                print!("    {}{}; {:06X}",
                         buf,
                         " ".repeat(24usize.saturating_sub(buf.len())),
                         addr);
                for off in 0..4 {
                    if off < x.size() + 1 {
                        print!(" {:02X}", rom[i+off]);
                    } else {
                        print!("   ");
                    }
                }
                println!(" |");

                for off in 0..x.size() {
                    if data_map[i + off + 1] != 1 { eprintln!("{:06X} {}", addr, x); }
                }

            } else { eprintln!("Error at {:06X}", addr); },
            0x03|0x06 => println!("    db ${:02X}                  ; {:06X} {0:02X}          |", rom[i], addr),
            0x04|0x07 => println!("    dw ${:02X}{:02X}                ; {:06X} {1:02X} {0:02X}       |", rom[i+1], rom[i], addr),
            0x05|0x08 => println!("    dl ${:02X}{:02X}{:02X}              ; {:06X} {2:02X} {1:02X} {0:02X}    |", rom[i+2], rom[i+1], rom[i], addr),
            c    => println!("    db ${:02X}    ; unknown type {}", rom[i], c),
        }
    }
    println!();
}
