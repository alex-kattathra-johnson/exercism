use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub struct Dna {
    dna: String,
    rna: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rna(String);

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        let mapping = HashMap::from([('G', 'C'), ('C', 'G'), ('T', 'A'), ('A', 'U')]);

        if let Some(i) = dna.chars().position(|c| !mapping.contains_key(&c)) {
            return Err(i);
        }

        let rna = dna.chars().filter_map(|c| mapping.get(&c)).collect();

        Ok(Dna {
            dna: dna.to_string(),
            rna,
        })
    }

    pub fn into_rna(self) -> Rna {
        Rna(self.rna)
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        let mapping = HashMap::from([('C', 'G'), ('G', 'C'), ('A', 'T'), ('U', 'A')]);
        if let Some(i) = rna.chars().position(|c| !mapping.contains_key(&c)) {
            return Err(i);
        }
        Ok(Rna(rna.to_string()))
    }
}
