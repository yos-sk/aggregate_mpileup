use std::error::Error;

pub fn run(input_file: &str,
           output_whitelist_file: &str,
           output_blacklist_file: &str,
           _min_depth: &str,
           _min_mismatch_ratio: &str,
           _min_samples_for_blacklist: &str,
           _min_samples_for_whitelist: &str)
       -> Result<(), Box<dyn Error>> {

    let min_depth: i32 = _min_depth.parse().unwrap();
    let min_mismatch_ratio: f64 = _min_mismatch_ratio.parse().unwrap();
    let min_samples_for_blacklist: i32 = _min_samples_for_blacklist.parse().unwrap();
    let min_samples_for_whitelist: i32 = _min_samples_for_whitelist.parse().unwrap();

    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .has_headers(false)
        .from_path(input_file)?;
        
    let mut whitelist_writer = csv::WriterBuilder::new()
        .delimiter(b'\t')
        .from_path(output_whitelist_file)?;
    
    let mut blacklist_writer = csv::WriterBuilder::new()
        .delimiter(b'\t')
        .from_path(output_blacklist_file)?;

    for result in reader.records() {
        let records = &result?;
        let chromosome = &records[0];
        let position = &records[1];
        let reference_base = &records[2];
       
        let mut record_index = 3;
        let mut count_whitelist = 0; 
        let mut count_blacklist = 0;
        let mut whitelist: Vec<String> = Vec::new();
        let mut blacklist: Vec<String> = Vec::new();
        while record_index < records.len() {
            let parsed_pileup: Vec<&str> = records[record_index].split(",").collect();
            
            let depth: i32 = parsed_pileup[0].parse().unwrap();
            //let count_match: i32 = parsed_pileup[1].parse().unwrap();
            let count_mismatch: i32 = parsed_pileup[2].parse().unwrap();
           
            // whitelist 
            if depth >= min_depth {
                count_whitelist += 1;
            }
            whitelist.push(depth.to_string());

            // blacklist            
            let mut percent_mismatch: f64 = 0.0;
            if depth > 0 {
                percent_mismatch = f64::from(count_mismatch) / f64::from(depth);
            }
                
            if percent_mismatch >= min_mismatch_ratio {
                count_blacklist += 1;
                blacklist.push(format!("{}/{}({:.5})",
                                       count_mismatch.to_string(),
                                       depth,
                                       percent_mismatch.to_string()))
                                       
            }
            
            record_index += 1;
        }
        
        // output whitelist
        if count_whitelist >= min_samples_for_whitelist {
            whitelist_writer.write_record(&[chromosome,
                                            position,
                                            reference_base,
                                            &count_whitelist.to_string(),
                                            &whitelist.join(",")])?;
        }
        
        // output blacklist
        if count_blacklist >= min_samples_for_blacklist {
            blacklist_writer.write_record(&[chromosome,
                                            position,
                                            reference_base,
                                            &count_blacklist.to_string(),
                                            &blacklist.join(",")])?;
        }
    }
    Ok(())
}