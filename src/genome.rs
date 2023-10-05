use crate::definitions::*;
use rand::Rng;
use std::mem;
use std::cmp;
use std::sync::atomic;

static COUNTER: atomic::AtomicUsize = atomic::AtomicUsize::new(0);

fn bump_counter() {
    // Add one using the most conservative ordering.
    COUNTER.fetch_add(1, atomic::Ordering::SeqCst);
}

fn draw_counter() -> usize {
    let curr_counter: usize = COUNTER.load(atomic::Ordering::SeqCst);
    bump_counter();
    curr_counter
}

pub fn get_counter() -> usize {
    COUNTER.load(atomic::Ordering::SeqCst)
}

// Main structures
struct Gene {
    source: u8,
    weight: u8,
    bias: u8,
    sink: u8,
    value: u32,
}

struct Genome {
    id: u32,
    adn: Vec<Gene>
}

// Traits overloading
impl std::fmt::Display for Gene {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}.{}", self.source, self.weight, self.bias, self.sink)
    }
}

impl cmp::PartialEq for Gene {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    } 
    
    fn ne(&self, other: &Self) -> bool {
        self.value != other.value
    } 
}

// Custom traits
impl Gene {
    // TODO: Docstring
    pub fn new_from_bytes(bytes: [u8; 4]) -> Self {
        unsafe {
            Gene {
                source: bytes[0],
                weight: bytes[1],
                bias: bytes[2],
                sink: bytes[3],
                value: mem::transmute(bytes),
            }
        }
    }

    // TODO: Docstring
    // TODO: UnitTest
    pub fn new_random() -> Self {
        let max_value_u8: u16 = u8::MAX as u16 + 1;
        unsafe {
            let mut gene: Gene = Gene {
                source: rand::thread_rng().gen_range(0..max_value_u8) as u8,
                weight: rand::thread_rng().gen_range(0..max_value_u8) as u8,
                bias: rand::thread_rng().gen_range(0..max_value_u8) as u8,
                sink: rand::thread_rng().gen_range(0..max_value_u8) as u8,
                value: 0,
            };
            gene.value = mem::transmute([gene.source, gene.weight, gene.bias, gene.sink]);
            gene
        }
    }

    // TODO: Docstring
    // TODO: UnitTest
    pub fn rebuild(&mut self) {
        unsafe {
            let bytes: [u8; 4] = mem::transmute(self.value);
            self.source = bytes[0];
            self.weight = bytes[1];
            self.bias = bytes[2];
            self.sink = bytes[3];
        }
    }
    
    // TODO: Docstring
    // TODO: UnitTest
    fn to_bytes(&self) -> [u8; 4] {
        let bytes: [u8; 4] = [self.source, self.weight, self.bias, self.sink];
        bytes
    }

    // TODO: Docstring
    // TODO: UnitTest
    fn mutate_random(&mut self) {
        let draw_random = rand::thread_rng().gen_range(0..GENOME_MUTATION_TRIES);
        if draw_random < GENOME_MUTATION_RATE {
            let mutation_mask: u32 = 1;
            self.value ^= mutation_mask << rand::thread_rng().gen_range(0..32);
            self.rebuild();
        }
    }

    // TODO: Docstring
    // TODO: UnitTest
    fn mutate_on_rate(&mut self, rate: u16) {
        let draw_random = rand::thread_rng().gen_range(0..GENOME_MUTATION_TRIES);
        if draw_random < rate {
            let mutation_mask: u32 = 1;
            self.value ^= mutation_mask << rand::thread_rng().gen_range(0..32);
            self.rebuild();
        }
    }

    // TODO: Docstring
    // TODO: UnitTest
    fn mutate_deterministic(&mut self) {
        let mutation_mask: u32 = 1;
        self.value ^= mutation_mask << rand::thread_rng().gen_range(0..32);
        self.rebuild();
    }

    fn print_binary(&self) {
        println!("{:0<8b}{:0<8b}{:0<8b}{:0<8b}", self.source, self.weight, self.bias, self.sink);
    }

    fn print_bytes(&self) {
        println!("{}.{}.{}.{}", self.source, self.weight, self.bias, self.sink);
    }

}

impl Genome {
    // TODO: Docstring
    // TODO: UnitTest
    pub fn new_random() -> Self {
        let genome_id: u32 = draw_counter() as u32;
        let mut adn: Vec<Gene> = Vec::new();
        for _gene_idx in 0..GENOME_SIZE {
            let gene = Gene::new_random();
            adn.push(gene);
        }
        Genome {id: genome_id, adn: adn}
    }

    // TODO: Docstring
    // TODO: UnitTest
    pub fn mutate_random(&mut self) {
        for gene in self.adn.iter_mut() {
            gene.mutate_random();
        }
    }

    // TODO: Docstring
    // TODO: UnitTest
    pub fn mutate_deterministic(&mut self) {
        for gene in self.adn.iter_mut() {
            gene.mutate_deterministic();
        }
    }

    // TODO: Docstring
    // TODO: UnitTest
    pub fn mutate_on_rate(&mut self, rate: u16) {
        for gene in self.adn.iter_mut() {
            gene.mutate_on_rate(rate);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_genome_to_bytes() {
        let gene: Gene = Gene::new_from_bytes([0b11111111, 0b01111110, 0b11100111, 0b00000001]);
        let bytes: [u8; 4] = gene.to_bytes();
        assert_eq!(0b11111111, bytes[0]);
        assert_eq!(0b01111110, bytes[1]);
        assert_eq!(0b11100111, bytes[2]);
        assert_eq!(0b00000001, bytes[3]);
    }

}