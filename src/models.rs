//Point of this module was to reconstruct the data from the csv file into a float decimal type in order to calculate the top countries and their attributes in regards to water pollution.
use serde::Deserialize;
use serde_with::{serde_as, DisplayFromStr};

#[serde_as]
#[derive(Debug, Deserialize, Clone)]
//In order to reconstruct data this struct was used.
pub struct WaterData {
    #[serde(rename = "Country")]
    pub country: String,

    #[serde(rename = "Water Source Type")]
    pub body: String,

    #[serde(rename = "pH Level")]
    #[serde_as(as = "DisplayFromStr")]
    pub ph: f32,

    #[serde(rename = "Turbidity (NTU)")]
    #[serde_as(as = "DisplayFromStr")]
    pub turbidity: f32,

    #[serde(rename = "Dissolved Oxygen (mg/L)")]
    #[serde_as(as = "DisplayFromStr")]
    pub doxygen: f32,

    #[serde(rename = "Nitrate Level (mg/L)")]
    #[serde_as(as = "DisplayFromStr")]
    pub nitrate: f32,

    #[serde(rename = "Contaminant Level (ppm)")]
    #[serde_as(as = "DisplayFromStr")]
    pub lead: f32,
}

impl WaterData {
    //Used to find the highest Turbidity, Nitrate, Lead Concentration, and Dissolved Oxygen.
    pub fn contamination_score(&self) -> f32 {
        self.turbidity + self.nitrate + self.lead - self.doxygen
    }
}
