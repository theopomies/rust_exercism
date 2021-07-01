use std::collections::HashMap;

const DNA_NUCLEOTIDES: &[char; 4] = &['A', 'C', 'G', 'T'];
const RNA_NUCLEOTIDES: &[char; 4] = &['U', 'G', 'C', 'A'];

#[derive(Debug, PartialEq)]
pub struct Dna(String);

#[derive(Debug, PartialEq)]
pub struct Rna(String);

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        match dna.chars().position(|n| !DNA_NUCLEOTIDES.contains(&n)) {
            Some(idx) => Err(idx),
            _ => Ok(Dna(dna.into())),
        }
    }
    pub fn into_rna(self) -> Rna {
        let Dna(dna) = self;
        let dna_to_rna: HashMap<char, char> = DNA_NUCLEOTIDES
            .iter()
            .copied()
            .zip(RNA_NUCLEOTIDES.iter().copied())
            .collect();
        Rna(dna.chars().map(|n| dna_to_rna[&n]).collect::<String>())
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        match rna.chars().position(|n| !RNA_NUCLEOTIDES.contains(&n)) {
            Some(idx) => Err(idx),
            _ => Ok(Rna(rna.into())),
        }
    }
}
