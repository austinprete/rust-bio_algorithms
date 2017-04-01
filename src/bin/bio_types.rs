use std::fmt;
use std::ops::{Index, Range};

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Nucleotide {
    A,
    C,
    G,
    T,
}

impl Nucleotide {
    fn complement(&self) -> Nucleotide {
        use self::Nucleotide::*;

        match *self {
            A => T,
            C => G,
            G => C,
            T => A,
        }
    }

    fn from_char(letter: char) -> Nucleotide {
        use self::Nucleotide::*;

        match letter {
            'A' => A,
            'C' => C,
            'G' => G,
            'T' => T,
            _ => panic!("Incorrect lettter"),
        }
    }
}

impl fmt::Display for Nucleotide {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Nucleotide::*;

        let letter = match *self {
            A => 'A',
            C => 'C',
            G => 'G',
            T => 'T',
        };

        write!(f, "{}", letter)
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct DNA_Pattern(pub Vec<Nucleotide>);

impl DNA_Pattern {
    #[allow(dead_code)]
    pub fn reverse_complement(&mut self) -> DNA_Pattern {

        let reverse_complement_vec = self.0
            .iter()
            .rev()
            .map(|x| x.complement())
            .collect::<Vec<Nucleotide>>();

        DNA_Pattern(reverse_complement_vec)
    }

    #[allow(dead_code)]
    pub fn from_string(dna_string: &str) -> DNA_Pattern {
        let pattern_vec = dna_string.chars()
            .map(|letter| Nucleotide::from_char(letter))
            .collect();

        DNA_Pattern(pattern_vec)
    }

    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl Index<usize> for DNA_Pattern {
    type Output = Nucleotide;

    fn index(&self, index: usize) -> &Nucleotide {
        &self.0[index]
    }
}

impl Index<Range<usize>> for DNA_Pattern {
    type Output = [Nucleotide];

    fn index(&self, range: Range<usize>) -> &[Nucleotide] {
        &self.0[range]
    }
}

impl fmt::Display for DNA_Pattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let pattern_string = self.0
            .iter()
            .fold(String::new(),
                  |acc, &dna_letter| format!("{}{}", acc, dna_letter));

        write!(f, "{}", pattern_string)
    }
}
