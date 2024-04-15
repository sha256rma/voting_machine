use csv;
use csv::Writer;
use log::info;
use std::error::Error as StdError;
use std::fs::OpenOptions;
use std::path::Path;

// Trait to be implemented by structs that can be converted to CSV records
pub trait ToCsvRecord {
    fn to_csv_record(&self) -> Vec<String>;
}

pub fn save_to_csv<T: ToCsvRecord>(
    records: &[T],
    headers: &[&str],
    file_path: &str,
) -> Result<(), Box<dyn StdError>> {
    let path = Path::new(file_path);
    let file_exists = path.exists();
    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .read(true)
        .open(file_path)?;

    // Log file creation or opening
    if file_exists {
        info!("Appending to existing CSV file: {}", file_path);
    } else {
        info!("Creating new CSV file: {}", file_path);
    }

    // If the file is newly created or empty, write the headers
    if !file_exists || file.metadata()?.len() == 0 {
        let mut wtr = Writer::from_writer(&file);

        // Log writing headers
        info!("Writing headers to CSV file: {:?}", headers);

        wtr.write_record(headers)?;
        for record in records {
            // Log writing record
            info!("Writing record to CSV file: {:?}", record.to_csv_record());
            wtr.write_record(record.to_csv_record())?;
        }
        wtr.flush()?;
    } else {
        // If the file already exists and is not empty, just append the records
        let mut wtr = Writer::from_writer(&file);
        for record in records {
            // Log writing record
            info!("Writing record to CSV file: {:?}", record.to_csv_record());
            wtr.write_record(record.to_csv_record())?;
        }
        wtr.flush()?;
    }
    Ok(())
}
