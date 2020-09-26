use encoding::all::ISO_8859_15;
use encoding::{DecoderTrap, EncoderTrap, Encoding};
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressIterator, ProgressStyle};
use logging_timer::{executing, timer};
use rayon::prelude::*;
use simple_logger::SimpleLogger;
use std::cmp::min;
use std::collections::{BTreeSet, HashSet};
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufWriter, Write};
use std::iter::FromIterator;
use std::sync::{Arc, Mutex};

fn read_words(filename: &str) -> io::Result<HashSet<Vec<u8>>> {
    let mut words = HashSet::new();
    let file = File::open(filename)?;
    for line in io::BufReader::new(file).lines() {
        if let Ok(ip) = line {
            if ip.starts_with('-') {
                continue;
            }
            match ISO_8859_15.encode(&ip, EncoderTrap::Strict) {
                Ok(enc) => {
                    words.insert(enc);
                }
                Err(..) => {
                    eprintln!("Unable to represent input word {}", ip);
                }
            }
        }
    }
    Ok(words)
}

const PROGRESS_DELTA_DIVISOR: usize = 100;

type WordsByStartingChar<'a> = [Vec<&'a Vec<u8>>; 256];

fn get_progress_style() -> indicatif::ProgressStyle {
    ProgressStyle::default_bar()
        .template("[{elapsed_precise}/{eta_precise}] {bar:40} {pos:>7}/{len:7} {msg}")
}

fn run() -> io::Result<()> {
    let tmr = timer!("JYS");
    let filename = env::var("JATKOYHDYSSANA_DICT").unwrap_or_else(|_| "sanat.txt".to_string());
    let words = read_words(filename.as_str())?;
    let mut words_by_starting_char: WordsByStartingChar = array_init::array_init(|_| Vec::new());
    for word in &words {
        let i0 = *word.get(0).unwrap();
        words_by_starting_char[i0 as usize].push(&word);
    }
    executing!(tmr, "Words read");

    let prog = ProgressBar::new(words.len() as u64).with_style(get_progress_style());
    prog.set_draw_delta((words.len() / PROGRESS_DELTA_DIVISOR) as u64);

    let outputs_ref = Arc::new(Mutex::new(BTreeSet::<String>::new()));

    (&words).par_iter().progress_with(prog).for_each(|w1| {
        let mut this_output: Vec<String> = Vec::new();
        process_word(&mut this_output, w1, &words_by_starting_char);
        if !this_output.is_empty() {
            let mut outputs = outputs_ref.lock().unwrap();
            for v in &this_output {
                outputs.insert(v.to_string());
            }
        }
    });

    executing!(tmr, "JYSes created");
    let output_set = outputs_ref.lock().unwrap();
    eprintln!("Sorting {} unique words...", output_set.len());
    let mut outputs = Vec::from_iter(output_set.iter());
    outputs.sort_unstable();

    executing!(tmr, "Sort done");
    eprintln!("Writing...");
    write_outputs(&outputs)?;

    executing!(tmr, "Writes done");
    Ok(())
}

#[allow(clippy::ptr_arg)]
fn write_outputs(outputs: &Vec<&String>) -> io::Result<()> {
    let newline = &[10u8];
    let file = File::create("jatkoyhdyssanat.txt")?;
    let mut handle = BufWriter::new(file);

    let prog = ProgressBar::new(outputs.len() as u64).with_style(get_progress_style());
    prog.set_draw_delta((outputs.len() / PROGRESS_DELTA_DIVISOR) as u64);

    for s in outputs.iter().progress_with(prog) {
        handle.write_all(s.as_bytes())?;
        handle.write_all(newline)?;
    }
    handle.flush()?;
    Ok(())
}

#[allow(clippy::ptr_arg)]
fn process_word(
    output: &mut Vec<String>,
    w1: &Vec<u8>,
    words_by_starting_char: &WordsByStartingChar,
) {
    let max_offset = w1.len() - 1;
    let p1 = max_offset - min(max_offset, 15);
    let p2 = max_offset - min(max_offset, 1);
    for start_index_byte in p1..p2 {
        let end = &w1[start_index_byte..];
        assert!(!end.is_empty());
        for w2 in &words_by_starting_char[*end.get(0).unwrap() as usize] {
            if (&w1).eq(w2) {
                continue;
            }
            if w2.starts_with(end) {
                let start_slice = &w1[..w1.len() - end.len()];
                let nw = [start_slice, w2].concat();
                if w1 != &nw && (&&nw) != (w2) {
                    if let Ok(decode) = ISO_8859_15.decode(&nw, DecoderTrap::Strict) {
                        output.push(decode);
                    }
                }
            }
        }
    }
}

fn main() {
    SimpleLogger::new().init().unwrap();
    run().unwrap();
}
