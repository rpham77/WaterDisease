mod models;
mod processor;
use processor::{read_data, top_sources, top_countries};
fn main() {
    let data = read_data("water_pollution_disease.csv").expect("Failed to read data");
    let top_sources = top_sources(&data, 5);
    println!("Top 5 Most Contaminated Water Sources (by type):");
    for (i, (source, avg_score)) in top_sources.iter().enumerate() {
      println!("{}. {} - Average Pollution Score: {:.2}", i + 1, source, avg_score);
}
    let top_countries = top_countries(&data, 5);
    println!("\nTop 5 Most Contaminated Countries (avg. metrics):");
    for (i, c) in top_countries.iter().enumerate() {
        println!(
            "{}. {} | pH: {:.2}, Turbidity: {:.2}, DO: {:.2}, Nitrate: {:.2}, Lead: {:.2}",
            i + 1, c.country, c.ph, c.turbidity, c.doxygen, c.nitrate, c.lead
        );
    }
} 