# iupac_replace_symbols

This is a small program that replaces all occurences of non-A/C/G/T nucleotide symbols in a (DNA) FASTA file by a random matching base. See https://www.bioinformatics.org/sms/iupac.html for the IUPAC codes.

## Download

You can download a binary on the [release page](https://github.com/njbirth/iupac-replace-symbols/releases). The Linux binary worked on my own computer. However, I cannot guarantee that it will run everywhere. I didn't test the Windows binary.

If you want (or the above mentioned binaries don't work for some reason), you can of course compile it yourself using `cargo build --release`. This requires Rust and Cargo to be installed. See https://www.rust-lang.org/learn/get-started for installation instructions.

## Usage

```
./iupac_replace_symbols -i <FASTA file> [-o <output file>]
```

## Some notes

- The program is able to process upper and lower case nucleotide symbols. The output, however, will always be upper case.
- The sequences in the output file won't have any line breaks within a sequence, regardless of the input format.
