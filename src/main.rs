use std::fs::File;
use std::io::Write;
use rand::seq::SliceRandom;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "iupac_replace_symbols", about = "Randomly replaces non-A/C/G/T symbols")]
struct Options {
    /// Input file in FASTA format
    #[structopt(short = "i")]
    pub infile: String,
    /// Output file
    #[structopt(short = "o", default_value = "outfile.fasta")]
    pub outfile: String,
}

fn main() {
    let opt = Options::from_args();

    let mut reader = needletail::parse_fastx_file(&opt.infile)
        .expect("Invalid FASTA file");

    let mut outfile = File::create(&opt.outfile)
        .expect("Unable to create outfile");

    while let Some(record) = reader.next() {
        let seqrec = record.expect("Invalid record");

        let header = String::from_utf8_lossy(seqrec.id());
        let new_seq = replace(&String::from_utf8_lossy(&seqrec.seq()));

        outfile.write_all(format!(">{}\n", header).as_bytes())
            .expect("Error writing to outfile");

        outfile.write_all(format!("{}\n", new_seq).as_bytes())
            .expect("Error writing to outfile");
    }
}

fn replace(sequence: &str) -> String {
    sequence.chars()
        .map(|c| replace_symbol(c))
        .collect()
}

fn replace_symbol(symbol: char) -> char {
    match symbol {
        'A' | 'a' => 'A',
        'C' | 'c' => 'C',
        'G' | 'g' => 'G',
        'T' | 't' => 'T',
        'W' | 'w' => rand_symbol(&['A', 'T']),
        'S' | 's' => rand_symbol(&['C', 'G']),
        'M' | 'm' => rand_symbol(&['A', 'C']),
        'K' | 'k' => rand_symbol(&['G', 'T']),
        'R' | 'r' => rand_symbol(&['A', 'G']),
        'Y' | 'y' => rand_symbol(&['C', 'T']),
        'B' | 'b' => rand_symbol(&['C', 'G', 'T']),
        'D' | 'd' => rand_symbol(&['A', 'G', 'T']),
        'H' | 'h' => rand_symbol(&['A', 'C', 'T']),
        'V' | 'v' => rand_symbol(&['A', 'C', 'G']),
        'N' | 'n' => rand_symbol(&['A', 'C', 'G', 'T']),
        _ => panic!("Unknown symbol: {}", symbol)
    }
}

fn rand_symbol(chars: &[char]) -> char {
    chars.choose(&mut rand::thread_rng()).unwrap().to_owned()
}