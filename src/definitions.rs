pub const SOURCE_ID_BITMASK: u32 = 0xFF << 24;
pub const SOURCE_W_BITMASK: u32 = 0xFF << 16;
pub const SOURCE_B_BITMASK: u32 = 0xFF << 8;
pub const SINK_ID_BITMASK: u32 = 0xFF;

pub const GENOMA_SIZE: u16 = 16;
pub const GENOME_MUTATION_TRIES: u16 = 1000;
pub const GENOME_MUTATION_RATE: u16 = 1;  // Defined as odds per GENOME_MUTATION_TRIES