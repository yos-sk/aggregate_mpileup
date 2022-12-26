use regex::Regex;
use std::error::Error;
use std::io;
use std::process;

fn parse() -> Result<(), Box<dyn Error>> {
    let regex_indel = Regex::new(r"[\+\-]{1}(\d+)").unwrap();
    let regex_mismatch_and_indel = Regex::new(r"[ACGTNacgtn]{1}").unwrap();
    let mut reader = csv::ReaderBuilder::new().delimiter(b'\t').has_headers(false).from_reader(io::stdin());

    for results in reader.records() {
        let record = results?;
        let chromosome = &record[0];
        let position = &record[1];
        let reference_base = &record[2];

        print!("{}\t{}\t{}", chromosome, position, reference_base);

        let mut record_index = 3;
        while record_index < record.len() {
            // pileup reads
            let pileup_reads = &record[record_index + 1];

            // count matched bases
            let count_match = pileup_reads.matches(".").count() + pileup_reads.matches(",").count();

            // count indel bases
            let mut count_indel = 0;
            for capture in regex_indel.captures_iter(pileup_reads) {
                let _str_count_indel = capture.get(1).unwrap().as_str();
                let _count_indel: usize = _str_count_indel.parse().unwrap();
                count_indel += _count_indel;
            }

            // count mismatch bases
            let count_mismatch_and_indel = regex_mismatch_and_indel.find_iter(pileup_reads).count();
            let count_mismatch = count_mismatch_and_indel - count_indel;

            // depth
            let depth = count_match + count_mismatch;

            print!("\t{}:{}:{}", count_match, count_mismatch, depth);

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
