use std::fs::File;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let tbl_tuna = [
        ("Yellowfin", 105, 15, 3),
        ("Albacore", 90, 15, 5),
        ("Skipjack", 50, 3, 4),
        ("Bigeye", 105, 25, 4),
        ("Atlantic Bonito", 50, 4, 2),
        ("Northern Bluefin", 190, 120, 11),
        ("Southern Bluefin", 190, 120, 11),
        ("Tongol", 90, 20, 4),
    ];

    let mut counter = 101;
    let mut file = File::create("tuna_data.txt")?;

    for tuna in &tbl_tuna {
        writeln!(file, "{},{},{},{},{}", counter, tuna.0, tuna.1, tuna.2, tuna.3)?;
        counter += 1;
    }

    Ok(())
}
