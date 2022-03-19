use std::{
    collections::HashMap,
    mem,
    thread::{self, JoinHandle},
};

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    match input.len() {
        0 => HashMap::new(),
        0..=500 => char_counter(input),
        _ => {
            let mut handles = Vec::with_capacity(worker_count);

            for input in input.chunks(input.len() / worker_count + 1) {
                let input: &'static [&str] =
                    unsafe { mem::transmute::<&[&str], &'static [&str]>(input) };
                handles.push(thread::spawn(move || char_counter(input)));
            }

            handles
                .into_iter()
                .map(JoinHandle::join)
                .map(Result::unwrap)
                .fold(HashMap::new(), thread_reducer)
        }
    }
}

fn char_counter(input: &[&str]) -> HashMap<char, usize> {
    let mut frequencies = HashMap::new();

    for line in input {
        for c in line
            .chars()
            .filter(|c| c.is_alphabetic())
            .map(|c| c.to_ascii_lowercase())
        {
            *frequencies.entry(c).or_default() += 1;
        }
    }

    frequencies
}

fn thread_reducer(
    mut res: HashMap<char, usize>,
    frequencies: HashMap<char, usize>,
) -> HashMap<char, usize> {
    for (c, freq) in frequencies {
        *res.entry(c).or_default() += freq;
    }

    res
}
