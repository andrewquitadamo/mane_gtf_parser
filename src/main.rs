use std::fs::File;
use std::env;
use flate2::read::GzDecoder;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashMap;

/*
fn download_file(download_url: &String) -> String {
    let (_, filename) = download_url.rsplit_once('/').unwrap();
    dbg!(filename);
    dbg!(download_url);
    let resp = reqwest::blocking::get(download_url).expect("request failed").bytes();
    let body = resp.expect("body invalid");
    let mut out = File::create(filename).expect("failed to create file");
    out.write(&body).expect("failed to write file");
    return filename.to_string()
}
*/

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
//    let download_url: &String = &args[1];
//    let filename = download_file(download_url);
    let filename: &String = &args[1];

    let file = File::open(filename)?;
    let reader = BufReader::new(GzDecoder::new(file));
    for line in reader.lines() {
        let line_val = line?;
        if !line_val.starts_with('#') {
            let mut attribute_fields_map = HashMap::new();
            let line_copy = line_val.clone();
            let fields: Vec<&str> = line_copy.split('\t').collect();
            if let [chrom, _, feature, start, end, _, strand, _, attributes] = fields[..] {
                if feature == "CDS" {
                      for attribute in attributes.split(';') {
                          if attribute != " " {
                              let key_val: Vec<&str> = attribute.trim().split('"').collect();
                              if key_val.len() > 1 {
                                  attribute_fields_map.insert(key_val[0].trim(), str::replace(key_val[1], '"', ""));
                              }
                          }
                      }
                      println!("{}\t{}\t{}\t{}\t{}\t{}\t{}_exon_{}", chrom, start, end, strand, attribute_fields_map.get("gene_id").unwrap(), attribute_fields_map.get("tag").unwrap(),attribute_fields_map.get("gene_id").unwrap(), attribute_fields_map.get("exon_number").unwrap());
                }
            }
        }
    }

    Ok(())
}
