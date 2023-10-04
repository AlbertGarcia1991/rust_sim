use crate::definitions::*;
use rand::Rng;

pub fn genome_split_gene(gene: u32) -> [u8; 4] {
    // TODO: Docstring
    let source_id: u8 = ((gene & SOURCE_ID_BITMASK) >> 24) as u8;
    let source_w: u8 = ((gene & SOURCE_W_BITMASK) >> 16) as u8;
    let source_b: u8 = ((gene & SOURCE_B_BITMASK) >> 8) as u8;
    let sink_id: u8 = (gene & SINK_ID_BITMASK) as u8;
    let genes: [u8; 4] = [source_id, source_w, source_b, sink_id];
    genes
}

pub fn genome_generate_random_genome() -> Vec<u32> {
    // TODO: UnitTest
    // TODO: Docstring
    let mut genome: Vec<u32> = Vec::new();
    for x in 1..GENOMA_SIZE {
        genome.push(genome_generate_random_gene());
    };
    genome
}

pub fn genome_mutate_genome(genome: &mut Vec<u32>) -> &mut Vec<u32> {
    // TODO: UnitTest
    // TODO: Docstring
    let mut iterator: std::slice::IterMut<'_, u32> = genome.iter_mut(); 
    while let Some(gene) = iterator.next() { 
        genome_mutate_gene(gene); 
    }
    genome
}

fn genome_generate_random_byte() -> u8 {
    /// Returns a randomly generated u8 (a gene is made up of 4 independent bytes)
    let random_byte: u8 = rand::thread_rng().gen_range(0..u8::MAX+1);
    random_byte
}

fn genome_generate_random_gene() -> u32 {
    // TODO: Docstring
    let mut gene: u32 = 0;
    for x in 1..5 {
        let byte: u8 = genome_generate_random_byte();
        match x {
            1 => gene = gene | ((byte as u32) << 24),
            2 => gene = gene | ((byte as u32) << 16),
            3 => gene = gene | ((byte as u32) << 8),
            _ => gene = gene | (byte as u32)
        };
        println!("GENE {x}: {byte:0>8b} -> {gene:0>32b}");
    };
    gene
}

fn genome_mutate_gene(gene: &mut u32) -> &mut u32 {
    // TODO: UnitTest
    // TODO: Docstring
    let draw_random = rand::thread_rng().gen_range(0..GENOME_MUTATION_TRIES);
    if draw_random < GENOME_MUTATION_RATE {
        let mut mutation_mask: u32 = 1;
        *gene ^= mutation_mask << rand::thread_rng().gen_range(0..32);
    }
    gene
}

#[cfg(test)]
mod tests {
    use crate::definitions::*;
    use super::*;

    #[test]
    fn test_genome_split_gene() {
        let gene: u32 = 1234567890;
        let gene_array: [u8; 4] = genome_split_gene(gene);
        assert_eq!(((gene & SOURCE_ID_BITMASK) >> 24) as u8, gene_array[0]);
        assert_eq!(((gene & SOURCE_W_BITMASK) >> 16) as u8, gene_array[1]);
        assert_eq!(((gene & SOURCE_B_BITMASK) >> 8) as u8, gene_array[2]);
        assert_eq!((gene & SINK_ID_BITMASK) as u8, gene_array[3]);
    }

    #[test]
    fn test_genome_generate_gen() {
        let gene_static: u32 = 0xAFA9BEEC;
        let mut gene: u32 = 0;
        for x in 1..5 {
            match x {
                1 => gene = gene | ((0xAF as u32) << 24),
                2 => gene = gene | ((0xA9 as u32) << 16),
                3 => gene = gene | ((0xBE as u32) << 8),
                _ => gene = gene | 0xEC as u32
            };
        };
        assert_eq!(gene_static, gene);
    }
}