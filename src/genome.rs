use crate::definitions::*;
use random_number::random;

pub fn genome_split_genes(gene: u32) -> [u8; 4] {
    let source_id: u8 = ((gene & SOURCE_ID_BITMASK) >> 24) as u8;
    let source_w: u8 = ((gene & SOURCE_W_BITMASK) >> 16) as u8;
    let source_b: u8 = ((gene & SOURCE_B_BITMASK) >> 8) as u8;
    let sink_id: u8 = (gene & SINK_ID_BITMASK) as u8;
    // println!("{gene:b} -> {source_id:0>8b} {source_w:0>8b} {source_b:0>8b} {sink_id:0>8b}");
    let genes: [u8; 4] = [source_id, source_w, source_b, sink_id];
    genes
}

fn genome_generate_random_byte() -> u8 {
    random!()
}

pub fn genome_generate_gene() -> u32 {
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


#[cfg(test)]
mod tests {
    use crate::definitions::*;
    use super::*;

    #[test]
    fn test_genome_split_genes() {
        let gene: u32 = 1234567890;
        let gene_array: [u8; 4] = genome_split_genes(gene);
        assert_eq!(((gene & SOURCE_ID_BITMASK) >> 24) as u8, gene_array[0]);
        assert_eq!(((gene & SOURCE_W_BITMASK) >> 16) as u8, gene_array[1]);
        assert_eq!(((gene & SOURCE_B_BITMASK) >> 8) as u8, gene_array[2]);
        assert_eq!((gene & SINK_ID_BITMASK) as u8, gene_array[3]);
    }
}