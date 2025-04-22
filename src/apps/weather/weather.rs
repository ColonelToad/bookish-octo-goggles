use reqwest;
use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize, Debug)]
struct WeatherData {
    latitude: f64,
    longitude: f64,
    elevation: f64,
    generationtime_ms: f64,
    utc_offset_seconds: i64,
    timezone: String,
    timezone_abbreviation: String,
    hourly : HourlyData,
    hourly_units: HourlyUnits,
}

#[derive(Deserialize, Debug)]
struct HourlyData {
    temperature_2m: Vec<f64>,
    precipitation_probability: Vec<f64>,
}

#[derive(Deserialize, Debug)]
struct HourlyUnits {
    temperature_2m: String,
}

#[derive(Debug)]
struct WeatherSimplified {
    temperatures: Vec<f64>,
    precipitation_probabilities: Vec<f64>,
}


impl From<WeatherData> for WeatherSimplified {
    fn from(data: WeatherData) -> Self {
        Self {
            temperatures: data.hourly.temperature_2m,
            precipitation_probabilities: data.hourly.precipitation_probability,
        }
    }
}

#[derive(Debug)]
struct HourlyDP {
    hour: usize, //hour index
    temperature_2m: f64,
    precipitation_probability: f64,
}

impl WeatherSimplified {
    fn hourly_entry(self) -> Vec<HourlyDP> {
        self.temperatures
            .into_iter()
            .zip(self.precipitation_probabilities.into_iter())
            .enumerate() // Get the index of each element
            .map(|(hour, (temp, precip))| HourlyDP {
                hour,
                temperature_2m: temp,
                precipitation_probability: precip,
            })
            .collect()
    }
}


#[tokio::main]
async fn main() -> Result <(), Box<dyn Error>> {

    let api_ep = "https://api.open-meteo.com/v1/forecast?latitude=28.6024812001&longitude=81.2001&hourly=temperature_2m,precipitation_probability&timezone=America%2FNew_York&forecast_days=1&wind_speed_unit=mph&temperature_unit=fahrenheit&precipitation_unit=inch";

    let response: WeatherData = reqwest::get(api_ep).await?.json().await?;

    let response_simplified: WeatherSimplified = response.into();
    let hourly_entry = response_simplified.hourly_entry();

    for entry in hourly_entry {
        println!("Hour: {}, Temperature: {}°F, Precipitation Chance: {}%",entry.hour, entry.temperature_2m, entry.precipitation_probability);
    }
    Ok(())
}
