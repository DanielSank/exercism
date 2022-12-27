use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    match nucleotide {
            'G'|'A'|'T'|'C' => (),
            other => return Err(other),
    }
    let mut counts = 0;
    for c in dna.chars() {
        println!("{}", c);
        match c {
            'G'|'A'|'T'|'C' => {
                if c == nucleotide {
                    counts += 1;
                }
            },
            other => return Err(other)
        }
    }
    Ok(counts)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut counts = HashMap::new();
    counts.insert('G', 0);
    counts.insert('A', 0);
    counts.insert('T', 0);
    counts.insert('C', 0);

    for c in dna.chars() {
        match c {
            'G'|'A'|'T'|'C' => {
                let num = counts.entry(c).or_insert(0);
                *num += 1;
            }
            other => return Err(other)
        }
    }
    Ok(counts)
}
