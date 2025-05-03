use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use crate::models::WaterData;
//This takes the &str path and can return a vector if it is successful or an error.
pub fn read_data(file_path: &str) -> Result<Vec<WaterData>, Box<dyn Error>> {
//Opens the file and returns the fail if it doesn't work out   
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);

    let mut records = Vec::new();
//Goes over each deserialized row in the file and will put it back into the vector or an error.
    for result in rdr.deserialize() {
        let record: WaterData = result?;
        records.push(record);
    }
    Ok(records)
}
//This would return the most polluted water sources.
pub fn top_sources(data: &[WaterData], top_n: usize) -> Vec<(String, f32)> {
    use std::collections::HashMap;
//Uses HashMap from the body of water as well as the vector previously included.
    let mut source_scores: HashMap<String, Vec<f32>> = HashMap::new();
//This for loop would create another vector for the body of water as well as adding the contaminant level.
    for record in data {
        source_scores
//Makes a vector to put the body of water in.
            .entry(record.body.clone())
            .or_default()
//Adds contamination level inside vector
            .push(record.contamination_score());
    }

    let mut averages: Vec<(String, f32)> = source_scores
        .into_iter()
        .map(|(source, scores)| {
            let avg = scores.iter().sum::<f32>() / scores.len() as f32;
            (source, avg)
        })
        .collect();

    averages.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    averages.into_iter().take(top_n).collect()
}
//Records the attributes in a country's water quality.
pub struct CountryScore {
    pub country: String,
    pub ph: f32,
    pub turbidity: f32,
    pub doxygen: f32,
    pub nitrate: f32,
    pub lead: f32,
    pub contamination_score: f32,
}
//Records the top 5 countries with the most contaminated level.
pub fn top_countries(data: &[WaterData], top_n: usize) -> Vec<CountryScore> {
    let mut country_data: HashMap<String, Vec<&WaterData>> = HashMap::new();
//Sorts out by name of the country.
    for record in data {
        country_data.entry(record.country.clone()).or_default().push(record);
    }

    let mut averages: Vec<CountryScore> = country_data
        .into_iter()
        .map(|(country, records)| {
            let count = records.len() as f32;

            let (mut ph, mut turbidity, mut doxygen, mut nitrate, mut lead, mut score) =
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
//Loops through each contaminant level found earlier.
            for r in &records {
                ph += r.ph;
                turbidity += r.turbidity;
                doxygen += r.doxygen;
                nitrate += r.nitrate;
                lead += r.lead;
                score += r.contamination_score();
            }

            CountryScore {
                country,
                ph: ph / count,
                turbidity: turbidity / count,
                doxygen: doxygen / count,
                nitrate: nitrate / count,
                lead: lead / count,
                contamination_score: score / count,
            }
        })
        .collect();

    averages.sort_by(|a, b| b.contamination_score.partial_cmp(&a.contamination_score).unwrap());
    averages.into_iter().take(top_n).collect()
}