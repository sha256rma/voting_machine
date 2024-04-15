use chrono::Utc;
use csv;
use log::{error, info};
use std::error::Error;

pub fn close_election(election_id: &str) -> Result<(), Box<dyn Error>> {
    // Log the start of the function with timestamp and election ID
    let timestamp = Utc::now();
    info!("{}: Closing election with ID: {}", timestamp, election_id);

    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path("elections.csv")?;

    let mut elections = reader.deserialize::<(String, String, bool)>();
    let mut updated_elections = Vec::new();
    let mut found = false;

    while let Some(result) = elections.next() {
        let (id, name, is_open) = result?;
        if id == election_id {
            found = true;
            updated_elections.push((id.clone(), name.clone(), false)); // Modify Is_Open to false
                                                                       // Log the action of closing the election including election details
            let timestamp = Utc::now();
            info!(
                "{}: Election with ID {} closed. Details: Name: {}, Is_Open: false",
                timestamp, election_id, name
            );
        } else {
            updated_elections.push((id, name, is_open));
        }
    }

    if !found {
        // Log election not found with timestamp and election ID
        let timestamp = Utc::now();
        error!("{}: Election with ID {} not found.", timestamp, election_id);
        return Ok(());
    }

    let mut writer = csv::Writer::from_path("elections.csv")?;
    writer.write_record(&["Election_ID", "Election_Name", "Is_Open"])?;

    for election in &updated_elections {
        // Log each updated election with timestamp
        let timestamp = Utc::now();
        info!("{}: Updated election: {:?}", timestamp, election);
        writer.serialize(election)?;
    }

    // Log the success of the operation with timestamp and election ID
    let timestamp = Utc::now();
    info!(
        "{}: Election with ID {} closed successfully.",
        timestamp, election_id
    );

    writer.flush()?;
    Ok(())
}
