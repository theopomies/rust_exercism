use std::collections::HashMap;

const STOP_CODON_NAME: &str = "stop codon";

pub struct CodonsInfo<'a> {
    map: HashMap<&'a str, &'a str>,
}

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        self.map.get(codon).copied()
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        rna.as_bytes()
            .chunks(3)
            .map(std::str::from_utf8)
            .map(|ref codon| self.name_for(codon.unwrap()))
            .take_while(|&name| name != Some(STOP_CODON_NAME))
            .collect::<Option<Vec<_>>>()
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    CodonsInfo {
        map: pairs.iter().copied().collect(),
    }
}
