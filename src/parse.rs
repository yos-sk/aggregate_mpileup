use regex::Regex;
use std::error::Error;

pub fn run(input_file: &str, output_file: &str) -> Result<(), Box<dyn Error>> {
    let regex_indel = Regex::new(r"[\+\-]{1}(\d+)").unwrap();
    let regex_mismatch = Regex::new(r"[ACGTNacgtn]{1}").unwrap();
    let regex_header = Regex::new(r"\^[ACGTNacgtn]").unwrap();

    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .has_headers(false)
        .from_path(input_file)?;
        
    let mut writer = csv::WriterBuilder::new()
        .delimiter(b'\t')
        .from_path(output_file)?;
        
    for result in reader.records() {
        let records = result?;
        let chromosome = &records[0];
        let position = &records[1];
        let reference_base = &records[2];

        writer.write_field(&chromosome)?;
        writer.write_field(&position)?;
        writer.write_field(&reference_base)?;

        let mut record_index = 3;
        while record_index < records.len() {
            // get pileup reads
            let pileup_reads = &records[record_index + 1];

            // count insertion occurrence
            let count_insertion_occurrence: i32 = pileup_reads.matches("+").count() as i32;
            
            // count deletion occurrence
            let count_deletion_occurrence: i32 = pileup_reads.matches("-").count() as i32;
            
            // count insertion + deletion base
            let mut count_indel_base: i32 = 0;
            for capture in regex_indel.captures_iter(pileup_reads) {
                let _str_count_indel_base = capture.get(1).unwrap().as_str();
                let _count_indel_base: i32 = _str_count_indel_base.parse().unwrap();
                count_indel_base += _count_indel_base;
            }
            
            // count substitution base
            let count_substitution_base: i32 = (regex_mismatch.find_iter(pileup_reads).count() as i32)
                                              - count_indel_base
                                              - (regex_header.find_iter(pileup_reads).count() as i32);
            
            // count match base
            let count_match_base: i32 = (pileup_reads.matches(".").count() as i32)
                                        + (pileup_reads.matches(",").count() as i32)
                                        - count_insertion_occurrence
                                        - count_deletion_occurrence;

            // count mismatch occurrence
            let count_mismatch_occurrence: i32 = count_substitution_base
                                                 + count_insertion_occurrence
                                                 + count_deletion_occurrence;
            
            // calcurate depth
            let depth = count_match_base + count_mismatch_occurrence;
            
            // output 
            let _output: Vec<i32> = vec![depth,
                                         count_match_base,
                                         count_mismatch_occurrence,
                                         count_substitution_base,
                                         count_insertion_occurrence,
                                         count_deletion_occurrence];
            let output: Vec<String> = _output.iter().map(|x| x.to_string()).collect();
            writer.write_field(output.join(","))?;
            
            record_index += 3;
        }
        writer.write_record(None::<&[u8]>)?;
    }
    Ok(())
}