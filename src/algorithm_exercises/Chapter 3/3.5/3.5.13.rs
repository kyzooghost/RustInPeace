#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// 3.5.13
// Modify LookupCSV to make a program RangeLookupCSV that takes two key values from the standard input and prints all key-value pairs in the .csv file such that the key falls within the range specified.

use std::error::Error;
use std::fs::File;
use std::process;
use std::io;
use chrono::{NaiveDate};

mod utils {pub mod RedBlackTree;}
use utils::RedBlackTree::RedBlackBST as ST;

fn run() -> Result<(), Box<dyn Error>> {
    // Parse CSV file contents into Symbol Table
    let FILE_PATH: String = "src/algorithm_exercises/Chapter 3/3.5/DJIA.csv".to_string();
    let CSV_FILE = File::open(FILE_PATH)?;
    let mut reader = csv::Reader::from_reader(CSV_FILE);
    let mut st = ST::new();

    for result in reader.records().into_iter() {
        let record = result?;
        let key = record.get(4).unwrap().parse::<f32>().unwrap(); // Close Price

        // Parse date
        let value = record.get(0).unwrap().to_string(); // Date
        let mut year: u32 = value[value.len()-2..].parse::<u32>().unwrap();
        
        if year <= 6 {
            year += 2000
        } else {
            year += 1900
        }

        let month_string = &value[value.len()-6..value.len()-3];
        let mut month: String = "".to_string();
        match month_string {
            "Jan" => month = "01".to_string(),
            "Feb" => month = "02".to_string(),
            "Mar" => month = "03".to_string(),
            "Apr" => month = "04".to_string(),
            "May" => month = "05".to_string(),
            "Jun" => month = "06".to_string(),
            "Jul" => month = "07".to_string(),
            "Aug" => month = "08".to_string(),
            "Sep" => month = "09".to_string(),
            "Oct" => month = "10".to_string(),
            "Nov" => month = "11".to_string(),
            "Dec" => month = "12".to_string(),
            _ => {}
        }

        let day_string = value[..value.len()-7].to_string();
        let mut date_string = year.to_string();
        date_string.push_str("-");
        date_string.push_str(&month);
        date_string.push_str("-");
        date_string.push_str(&day_string);

        let date = NaiveDate::parse_from_str(&date_string, "%Y-%m-%d")?;

        // Insert into symbol tree
        st.put(key, date);
    }

    // stdin query for key

    let global_min: f32 = st.min().unwrap();
    let global_max: f32 = st.max().unwrap();
    println!("Min close price: {:?}, max close price: {:?}", global_min, global_max);

    let mut min: f32;
    let mut max: f32;

    loop {
        println!("Enter minimum close price to query for: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        min = input.trim().parse::<f32>()?;
    
        println!("Enter maximum close price to query for: ");
        input = String::new();
        io::stdin().read_line(&mut input)?;
        max = input.trim().parse::<f32>()?;

        if min < max {break}
        else {
            println!("Cannot have min >= max, please re-enter minimum and maximum close prices.");
        }
    }

    let keys = st.keys(min, max);
    let values = keys.iter().map(|&x| st.get(x).unwrap()).collect::<Vec<_>>();

    println!("Dates where close price was between {:?} and {:?}: {:?}", min, max, values);

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}