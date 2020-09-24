use encoding::all::ISO_8859_15;
use encoding::{DecoderTrap, EncoderTrap, Encoding};
use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};
use std::cmp::min;
use std::collections::HashSet;
use std::convert::TryInto;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn read_words() -> io::Result<HashSet<Vec<u8>>> {
    let mut words = HashSet::new();
    let lines = read_lines("/Users/akx/build/jatkoyhdyssana/kotus_sanat.txt")?;
    for line in lines {
        if let Ok(ip) = line {
            if ip.starts_with('-') {
                continue;
            }
            if ip.contains(' ') {
                continue;
            }
            if let Ok(enc) = ISO_8859_15.encode(&ip, EncoderTrap::Strict) {
                words.insert(enc);
            }
        }
    }
    Ok(words)
}

fn run() -> io::Result<()> {
    let words = read_words()?;
    let mut words_by_starting_char: [Vec<&Vec<u8>>; 255] = array_init::array_init(|_| Vec::new());
    for word in &words {
        let i0 = *word.get(0).unwrap();
        words_by_starting_char[i0 as usize].push(&word);
    }

    let ps = ProgressStyle::default_bar()
        .template("[{elapsed_precise}/{eta_precise}] {bar:40} {pos:>7}/{len:7} {msg}");
    let prog = ProgressBar::new(words.len().try_into().unwrap()).with_style(ps);

    for w1 in (&words).iter().progress_with(prog) {
        for n in 3..min(w1.len(), 15) {
            let start_index_byte = w1.len() - n;
            let end = &w1[start_index_byte..];
            for w2 in &words_by_starting_char[*end.get(0).unwrap() as usize] {
                if w2.starts_with(end) {
                    let start_slice = &w1[..w1.len() - end.len()];
                    let nw = [start_slice, w2].concat();
                    if &nw != w1 {
                        if let Ok(decode) = ISO_8859_15.decode(&nw, DecoderTrap::Strict) {
                            println!("{}", decode);
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

fn main() {
    run().unwrap();
}
