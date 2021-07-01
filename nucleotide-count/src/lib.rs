use std::collections::HashMap;
use std::iter::repeat;

const NUCLEOTIDES: &str = "ACGT";
const NUMBER_OF_NUCLEOTIDES: usize = NUCLEOTIDES.len();

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    nucleotide_counts(dna)?
        .remove(&nucleotide)
        .ok_or(nucleotide)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    dna.chars().try_fold(
        NUCLEOTIDES
            .chars()
            .zip(repeat(0).take(NUMBER_OF_NUCLEOTIDES))
            .collect(),
        |mut map: HashMap<_, _>, c| map.get_mut(&c).map(|c| *c += 1).ok_or(c).and(Ok(map)),
    )
}
