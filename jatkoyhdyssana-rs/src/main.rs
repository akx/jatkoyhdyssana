use encoding::all::ISO_8859_15;
use encoding::{DecoderTrap, EncoderTrap, Encoding};
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::cmp::min;
use std::collections::HashSet;
use std::convert::TryInto;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};
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

type WordsByStartingChar<'a> = [Vec<&'a Vec<u8>>; 255];

fn run() -> io::Result<()> {
    let filename =
        env::var("JATKOYHDYSSANA_DICT").unwrap_or_else(|_| "kotus_sanat.txt".to_string());
    let words = read_words(filename.as_str())?;
    let mut words_by_starting_char: WordsByStartingChar = array_init::array_init(|_| Vec::new());
    for word in &words {
        let i0 = *word.get(0).unwrap();
        words_by_starting_char[i0 as usize].push(&word);
    }

    let ps = ProgressStyle::default_bar()
        .template("[{elapsed_precise}/{eta_precise}] {bar:40} {pos:>7}/{len:7} {msg}");
    let prog = ProgressBar::new(words.len().try_into().unwrap()).with_style(ps);

    let outputs_ref = Arc::new(Mutex::new(Vec::<String>::new()));

    (&words).par_iter().progress_with(prog).for_each(|w1| {
        let mut this_output: Vec<String> = Vec::new();
        process_word(&mut this_output, w1, &words_by_starting_char);
        if !this_output.is_empty() {
            outputs_ref.lock().unwrap().append(&mut this_output);
        }
    });

    let mut outputs = outputs_ref.lock().unwrap();
    outputs.sort();
    write_outputs(&outputs)?;
    Ok(())
}

#[allow(clippy::ptr_arg)]
fn write_outputs(outputs: &Vec<String>) -> io::Result<()> {
    let stdout = io::stdout();
    let newline = &[10u8];
    let mut handle = stdout.lock();
    for s in outputs.iter() {
        handle.write_all(&[s.as_bytes(), newline].concat())?;
    }
    Ok(())
}

#[allow(clippy::ptr_arg)]
fn process_word(
    output: &mut Vec<String>,
    w1: &Vec<u8>,
    words_by_starting_char: &WordsByStartingChar,
) {
    for n in 3..min(w1.len(), 15) {
        let start_index_byte = w1.len() - n;
        let end = &w1[start_index_byte..];
        for w2 in &words_by_starting_char[*end.get(0).unwrap() as usize] {
            if w2.starts_with(end) {
                let start_slice = &w1[..w1.len() - end.len()];
                let nw = [start_slice, w2].concat();
                if w1 != &nw {
                    if let Ok(decode) = ISO_8859_15.decode(&nw, DecoderTrap::Strict) {
                        output.push(decode);
                    }
                }
            }
        }
    }
}

fn main() {
    run().unwrap();
}
