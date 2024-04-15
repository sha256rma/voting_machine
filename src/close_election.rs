use csv;
use std::error::Error;

pub fn close_election(election_id: &str) -> Result<(), Box<dyn Error>> {
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
            updated_elections.push((id.clone(), name, false)); // Modify Is_Open to false
        } else {
            updated_elections.push((id, name, is_open));
        }
    }

    if !found {
        println!("Election with ID {} not found.", election_id);
        return Ok(());
    }

    let mut writer = csv::Writer::from_path("elections.csv")?;
    writer.write_record(&["Election_ID", "Election_Name", "Is_Open"])?;

    for election in updated_elections {
        writer.serialize(election)?;
    }

    writer.flush()?;
    Ok(())
}
