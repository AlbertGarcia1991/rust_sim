mod genome;
mod definitions;

fn main() {
    let genoma: u32 = genome::genome_generate_gene();
    println!("{genoma:0>32b}");
}
