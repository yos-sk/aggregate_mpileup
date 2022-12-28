use regex::Regex;
use std::error::Error;
use std::io;
use std::process;

fn parse() -> Result<(), Box<dyn Error>> {
    let regex_indel = Regex::new(r"[\+\-]{1}(\d+)").unwrap();
    let regex_mismatch = Regex::new(r"[ACGTNacgtn]{1}").unwrap();
    let mut reader = csv::ReaderBuilder::new().delimiter(b'\t').has_headers(false).from_reader(io::stdin());

    for result in reader.records() {
        let records = result?;
        let chromosome = &records[0];
        let position = &records[1];
        let reference_base = &records[2];

        print!("{}\t{}\t{}", chromosome, position, reference_base);

        let mut record_index = 3;
        while record_index < records.len() {
            // pileup reads
            let pileup_reads = &records[record_index + 1];

            // count insertion occurrence
            let count_insertion_occurrence = pileup_reads.matches("+").count();
            
            // count deletion occurrence
            let count_deletion_occurrence = pileup_reads.matches("-").count();
            
            // count insertion + deletion base
            let mut count_indel_base = 0;
            for capture in regex_indel.captures_iter(pileup_reads) {
                let _str_count_indel_base = capture.get(1).unwrap().as_str();
                let _count_indel_base: usize = _str_count_indel_base.parse().unwrap();
                count_indel_base += _count_indel_base;
            }
            
            // count substitution base
            let count_substitution_base = regex_mismatch.find_iter(pileup_reads).count() - count_indel_base;
            
            // count match base
            let count_match_base = pileup_reads.matches(".").count() + pileup_reads.matches(",").count() - count_insertion_occurrence - count_deletion_occurrence;

            // count mismatch occurrence
            let count_mismatch_occurrence = count_substitution_base + count_insertion_occurrence + count_deletion_occurrence;
            
            // depth
            let depth = count_match_base + count_mismatch_occurrence;

            print!("\t{},{},{},{},{},{}", depth, count_match_base, count_mismatch_occurrence, count_substitution_base, count_insertion_occurrence, count_deletion_occurrence);

            record_index += 3;
        }
        println!("");
    }
    Ok(())
}

fn main() {
    if let Err(error) = parse() {
        eprintln!("{}", error);
        process::exit(1);
    }
}
