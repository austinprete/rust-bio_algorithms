use std::ascii::AsciiExt;
use std::fmt;
use std::ops::{Index, Range};

/// Represents a single nucleotide and acts as a building block
/// for the `DNA_Sequence` type.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Nucleotide {
    A,
    C,
    G,
    T,
}

impl Nucleotide {
    /// Returns the complement of the current nucleotide.
    pub fn complement(&self) -> Nucleotide {
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

        match letter.to_ascii_uppercase() {
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

/// Represents a DNA sequence as a vector of Nucleotide instances.
#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct DNA_Sequence(pub Vec<Nucleotide>);

impl DNA_Sequence {
    /// Creates and returns a new DNA sequence representing the reverse
    /// complement of the current sequence.
    #[allow(dead_code)]
    pub fn reverse_complement(&mut self) -> DNA_Sequence {

        let reverse_complement_vec = self.0
            .iter()
            .rev()
            .map(|x| x.complement())
            .collect::<Vec<Nucleotide>>();

        DNA_Sequence(reverse_complement_vec)
    }

    /// Provided a string representation of a DNA sequence (such as "AACGTACA"),
    /// returns a corresponding `DNA_Sequence` instance.
    pub fn from_string(dna_string: &str) -> DNA_Sequence {
        let pattern_vec = dna_string.chars().map(Nucleotide::from_char).collect();

        DNA_Sequence(pattern_vec)
    }

    /// Returns the length of the sequence (the number of nucleotides it contains).
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns true if the sequence is empty (length of 0).
    pub fn is_empty(&self) -> bool {
        self.0.len() == 0
    }

    /// Provided a DNA pattern, returns the number of matches found in this sequence.
    pub fn pattern_match_count(&self, pattern: DNA_Sequence) -> usize {
        let mut count = 0;

        for index in 0..(self.len() - pattern.len() + 1) {
            if pattern == DNA_Sequence(self[index..index + pattern.len()].to_vec()) {
                count += 1;
            }
        }

        count
    }

    /// Returns an integer value corresponding to the current sequence for use with
    /// frequency array algorithms. **Note:** sequences longer than 32 nucleotides might
    /// result in an integer overflow.
    pub fn pattern_to_number(&self) -> u64 {

        let mut pattern_number = 0;

        for (index, nucleotide) in self.0.iter().rev().enumerate() {
            let multiplier = 4u64.pow(index as u32);

            let nucleotide_number = match *nucleotide {
                Nucleotide::A => 0,
                Nucleotide::C => 1,
                Nucleotide::G => 2,
                Nucleotide::T => 3,
            };

            pattern_number += nucleotide_number * multiplier;
        }

        pattern_number
    }
}

impl Index<usize> for DNA_Sequence {
    type Output = Nucleotide;

    fn index(&self, index: usize) -> &Nucleotide {
        &self.0[index]
    }
}

impl Index<Range<usize>> for DNA_Sequence {
    type Output = [Nucleotide];

    fn index(&self, range: Range<usize>) -> &[Nucleotide] {
        &self.0[range]
    }
}

impl fmt::Display for DNA_Sequence {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let pattern_string = self.0
            .iter()
            .fold(String::new(),
                  |acc, &dna_letter| format!("{}{}", acc, dna_letter));

        write!(f, "{}", pattern_string)
    }
}
