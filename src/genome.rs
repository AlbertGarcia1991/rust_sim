use crate::definitions::*;
use rand::Rng;
use std::mem;
use std::cmp;
use std::sync::atomic;

/// Unique counter of a Genome instance (every time we create a new instance, is increased by 1)
static GENOME_ID: atomic::AtomicUsize = atomic::AtomicUsize::new(0);

/// Increases by one the value of the GENOME_ID counter
fn bump_counter() {
    GENOME_ID.fetch_add(0, atomic::Ordering::SeqCst);
}

/// Returns the current value of the GENOME_ID counter
pub fn get_counter() -> usize {
    GENOME_ID.load(atomic::Ordering::SeqCst)
}

/// Increases by one the value of the GENOME_ID counter and returns this new value
fn draw_counter() -> usize {
    bump_counter();
    let curr_counter: usize = GENOME_ID.load(atomic::Ordering::SeqCst);
    curr_counter
}


/// We define a Gene as a structure that contains 4 bytes (source neuron id, neuron weight, neuron 
/// bias, and destination sink id), plus another u32 bit value which is the bit-wise concatenation 
/// of the previous four values, being source the MSB and sink the LSH. Hence, two Genes are unique 
/// if and only if they hold the same value. This is also very useful to understand at each 
/// iteration the ADN diversity of the entities
#[derive(Debug, Clone)]
struct Gene {
    /// Source neuron id
    source: u8,
    /// Source neuron weight
    weight: u8,
    /// Source neuron bias
    bias: u8,
    /// Sink (destination) neuron wid
    sink: u8,
    /// Unique value of the current configuration of bytes
    value: u32
}

/// We define the Genome as a structure that contains an unique identifier and a the adn as a vector 
/// of Genes. This vector has a fixed length of GENOME_SIZE genes inside. The identifier is unique 
/// for each Genome, hence, two different instances of Genome will have a different id even if they 
/// have the same adn inside.
// LEARN: Debug attribute?
#[derive(Debug, Clone)]
struct Genome {
    /// Unique identifier of the Genome
    id: u32,
    /// Vector of Gene objects
    adn: Vec<Gene>
}

impl std::fmt::Display for Gene {
    /// A trait that overloads the print!() macro of a Gene by showing the fours bits is made of 
    /// separated by a point.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}.{}", self.source, self.weight, self.bias, self.sink)
    }
}

impl cmp::PartialEq for Gene {
    /// A trait that overloads the equal comparison between two Genes
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    } 
    
    /// A trait that overloads the not equal comparison between two Genes
    fn ne(&self, other: &Self) -> bool {
        self.value != other.value
    } 
}


impl Gene {
    /// Constructor of a Gene by passing an array with the four bytes of the object. The unique 
    /// value for that combination of genes is computed automatically.
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

    /// Constructor of a Gene to create it randomly. The unique value for that combination of genes 
    /// is computed automatically. It is unsafe since we are transmuting those four bytes to 
    /// generate the unique value.
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

    /// Trait to assign a value to each one of the four bytes of the Gene based on the Gene's
    /// unique value.
    fn rebuild(&mut self) {
        unsafe {
            let bytes: [u8; 4] = mem::transmute(self.value);
            self.source = bytes[0];
            self.weight = bytes[1];
            self.bias = bytes[2];
            self.sink = bytes[3];
        }
    }
    
    //// Trait to return the four bytes of a Gene as an array.
    fn to_bytes(&self) -> [u8; 4] {
        let bytes: [u8; 4] = [self.source, self.weight, self.bias, self.sink];
        bytes
    }

    /// Trait to perform a random mutation on a Gene. We understand as mutation the flip of a single 
    /// bit only in one of the 4 bytes of the Gene. 
    fn mutate_random(&mut self) {
        let draw_random = rand::thread_rng().gen_range(0..GENOME_MUTATION_TRIES);
        if draw_random < GENOME_MUTATION_RATE {
            let mutation_mask: u32 = 1;
            self.value ^= mutation_mask << rand::thread_rng().gen_range(0..32);
            self.rebuild();
        }
    }

    /// Trait to perform a mutation on a Gene based on the given odds. Odds is the probability of a 
    /// mutation as a part per mil (e.g. if odds=10, there is 1% of chances to mutate: 10 / 1000). 
    /// We understand as mutation the flip of a single bit only in one of the 4 bytes of the Gene.
    fn mutate_on_odds(&mut self, odds: u16) {
        let draw_random = rand::thread_rng().gen_range(0..GENOME_MUTATION_TRIES);
        if draw_random < odds {
            let mutation_mask: u32 = 1;
            self.value ^= mutation_mask << rand::thread_rng().gen_range(0..32);
            self.rebuild();
        }
    }

    /// Trait to always mutate the Gene. We understand as mutation the flip of a single bit only in 
    /// one of the 4 bytes of the Gene.
    fn mutate_deterministic(&mut self) {
        let mutation_mask: u32 = 1;
        self.value ^= mutation_mask << rand::thread_rng().gen_range(0..32);
        self.rebuild();
    }

    /// Trait to print the the whole binary number of the Gene (unique value).
    fn print_binary(&self) {
        println!("{:0<32b}", self.value);
    }

}

impl Genome {
    /// Constructor to create Genome object with a random adn sequence.
    pub fn new_random() -> Self {
        let genome_id: u32 = draw_counter() as u32;
        let mut adn: Vec<Gene> = Vec::new();
        for _GENOME_IDx in 0..GENOME_SIZE {
            let gene = Gene::new_random();
            adn.push(gene);
        }
        Genome {id: genome_id, adn: adn}
    }

    /// Trait to perform a random mutation on each Gene. 
    pub fn mutate_random(&mut self) {
        for gene in self.adn.iter_mut() {
            gene.mutate_random();
        }
    }

    /// Trait to perform a certain mutation on each Gene. 
    pub fn mutate_deterministic(&mut self) {
        for gene in self.adn.iter_mut() {
            gene.mutate_deterministic();
        }
    }

    /// Trait to perform a mutation on each Gene based on the given odds. 
    pub fn mutate_on_odds(&mut self, odds: u16) {
        for gene in self.adn.iter_mut() {
            gene.mutate_on_odds(odds);
        }
    }

    /// Trait to print the whole Genome sequence.
    pub fn print(&self) {
        println!("Genome ID: {}", self.id);
        let mut gene_counter: u8 = 1;
        for gene in self.adn.iter() {
            println!("Gen {gene_counter}:\t{}", gene.value);
            gene_counter += 1;
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::genome;

    use super::*;

    #[test]
    fn test_gene_new_from_bytes() {
        let gene_a: Gene = Gene::new_from_bytes([0b11111111, 0b01111110, 0b11100111, 0b00000001]);
        let gene_a_copy: Gene = Gene::new_from_bytes([0b11111111, 0b01111110, 0b11100111, 0b00000001]);
        let gene_b: Gene = Gene::new_from_bytes([0b11111111, 0b01111110, 0b11100111, 0b10000001]);
        assert_eq!(gene_a, gene_a_copy);
        assert_ne!(gene_a, gene_b);
    }

    #[test]
    fn test_gene_new_random() {
        // FIXME: This is a probabilistic test, how to make it robust?
        let gene_a: Gene = Gene::new_random();
        let gene_b: Gene = Gene::new_random();
        assert_ne!(gene_a, gene_b);
    }

    #[test]
    fn test_gene_rebuild() {
        let mut gene_a: Gene = Gene::new_random();
        let gene_b: Gene = gene_a.clone();
        gene_a.value += 1;
        gene_a.rebuild();
        assert_ne!(gene_a, gene_b);
        assert_eq!(gene_a, gene_a);
    }

    #[test]
    fn test_gene_transmute_unpacking() {
        let mut gene_a: Gene = Gene::new_random();
        let gene_b: Gene = gene_a.clone();
        gene_a.rebuild();
        assert_eq!(gene_a.source, gene_b.source);
        assert_eq!(gene_a.weight, gene_b.weight);
        assert_eq!(gene_a.bias, gene_b.bias);
        assert_eq!(gene_a.sink, gene_b.sink);
    }

    #[test]
    fn test_gene_mutate_on_odds_always() {
        let mut gene_a: Gene = Gene::new_random();
        let gene_b: Gene = gene_a.clone();
        gene_a.mutate_on_odds(1000);
        assert_ne!(gene_a, gene_b);
        assert_eq!(gene_a, gene_a);
    }

    #[test]
    fn test_gene_mutate_on_odds_never() {
        let mut gene_a: Gene = Gene::new_random();
        let gene_b: Gene = gene_a.clone();
        gene_a.mutate_on_odds(0);
        assert_eq!(gene_a, gene_b);
        assert_eq!(gene_a, gene_a);
    }

    #[test]
    fn test_gene_to_bytes() {
        let gene: Gene = Gene::new_from_bytes([0b11111111, 0b01111110, 0b11100111, 0b00000001]);
        let bytes: [u8; 4] = gene.to_bytes();
        assert_eq!(0b11111111, bytes[0]);
        assert_eq!(0b01111110, bytes[1]);
        assert_eq!(0b11100111, bytes[2]);
        assert_eq!(0b00000001, bytes[3]);
    }

    #[test]
    fn test_gene_mutate_deterministic() {
        let mut gene_a: Gene = Gene::new_random();
        let gene_b: Gene = gene_a.clone();
        gene_a.mutate_deterministic();
        assert_ne!(gene_a, gene_b);
        assert_eq!(gene_a, gene_a);
    }

    #[test]
    fn test_genome_mutate_deterministic() {
        let mut genome_a: Genome = Genome::new_random();
        let genome_b: Genome = genome_a.clone();
        genome_a.mutate_deterministic();
        assert_ne!(genome_a.adn, genome_b.adn);
    }

    #[test]
    fn test_genome_mutate_on_odds_always() {
        let mut genome_a: Genome = Genome::new_random();
        let genome_b: Genome = genome_a.clone();
        genome_a.mutate_on_odds(1000);
        assert_ne!(genome_a.adn, genome_b.adn);
    }

    #[test]
    fn test_genome_mutate_on_odds_never() {
        let mut genome_a: Genome = Genome::new_random();
        let genome_b: Genome = genome_a.clone();
        genome_a.mutate_on_odds(0);
        assert_eq!(genome_a.adn, genome_b.adn);
    }

}