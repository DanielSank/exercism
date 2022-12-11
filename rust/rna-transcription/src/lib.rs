#[derive(Debug, PartialEq, Eq)]
pub struct Dna {
    nucleotides: Vec<char>
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rna {
    nucleotides: Vec<char>
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        let mut nucleotides: Vec<char> = Vec::new();
        for (idx, c) in dna.chars().enumerate() {
            if c == 'G' || c == 'C' || c == 'T' || c == 'A' {
                nucleotides.push(c)
            } else {
                return Err(idx)
            }
        }
        Ok(Dna {nucleotides})
    }

    pub fn into_rna(self) -> Rna {
        let nucleotides = self.nucleotides.iter().map(
            |n| match n {
                'G' => 'C',
                'C' => 'G',
                'T' => 'A',
                'A' => 'U',
                _ => panic!(),
            }
        ).collect();
        Rna{nucleotides}
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        let mut nucleotides: Vec<char> = Vec::new();
        for (idx, c) in rna.chars().enumerate() {
            if c == 'G' || c == 'C' || c == 'U' || c == 'A' {
                nucleotides.push(c)
            } else {
                return Err(idx)
            }
        }
        Ok(Rna {nucleotides})
    }
}
