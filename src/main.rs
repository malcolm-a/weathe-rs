use std::sync::{Arc, Mutex};

use cursive::{Cursive, CursiveExt, theme::{BaseColor, Color, PaletteColor}};
use cursive::traits::*;
use cursive::views::{Dialog, EditView, LinearLayout, TextView};
use reqwest::Client;
use serde::Deserialize;
use tokio::runtime::Runtime;

use api_key::API_KEY;

mod api_key;

const BASE_URL: &str = "https://api.weatherapi.com/v1/current.json";

#[derive(Deserialize)]
struct WeatherResponse {
    location: Location,
    current: Current,
}

#[derive(Deserialize)]
struct Location {
    name: String,
    region: String,
    country: String,
}

#[derive(Deserialize)]
struct Current {
    temp_c: f64,
    feelslike_c: f64,
    wind_kph: f64,
    wind_dir: String,
    pressure_mb: f64,
    precip_mm: f64,
    humidity: i32,
    cloud: i32,
    condition: Condition,
}

#[derive(Deserialize)]
struct Condition {
    text: String,
    code: i32,
}


fn get_weather_emoji(condition_code: i32) -> &'static str {
    match condition_code {
        1000 => "☀️", // Sunny / Clear
        1003 => "🌤️", // Partly cloudy
        1006 | 1009 => "☁️", // Cloudy
        1030 | 1135 | 1147 => "🌫️", // Mist / Fog
        1063 | 1150 | 1180 | 1240 | 1195 | 1243 | 1273 => "🌧️", // Rain
        1183 | 1186 | 1192 => "🌦️", // Rain
        1087 | 1276 => "🌩️", // Thunderstorm
        1066 | 1114 | 1210 | 1213 | 1255 | 1117 | 1225 | 1258 => "❄️", // Snow
        1069 | 1204 | 1249 => "🌨️", // Sleet
        1198 | 1237 | 1261 => "🧊", // Freezing rain / Ice
        _ => "🌍", // Default icon for unknown weather
    }
}

async fn fetch_weather(query: &str) -> Result<WeatherResponse, reqwest::Error> {
    let url = format!("{}?key={}&q={}", BASE_URL, API_KEY, query);
    let client = Client::new();
    let response = client.get(&url).send().await?.json::<WeatherResponse>().await?;
    Ok(response)
}

fn update_weather(s: &mut Cursive, query: &str, rt: Arc<Runtime>) {
    let result = rt.block_on(fetch_weather(query));
    match result {
        Ok(weather) => {
            let emoji = get_weather_emoji(weather.current.condition.code);
            s.call_on_name("weather_text", |view: &mut TextView| {
                view.set_content(format!(
                    "{} {}, {}\n{}\n{}°C (Feels like: {}°C)\n\
                    Wind: {} kph, {}\nPressure: {} mb\nPrecipitation: {} mm\nHumidity: {}%\nCloud Cover: {}%",
                    emoji,
                    weather.location.name,
                    weather.location.country,
                    weather.current.condition.text,
                    weather.current.temp_c,
                    weather.current.feelslike_c,
                    weather.current.wind_kph,
                    weather.current.wind_dir,
                    weather.current.pressure_mb,
                    weather.current.precip_mm,
                    weather.current.humidity,
                    weather.current.cloud
                ));
            });
        }
        Err(_) => {
            s.call_on_name("weather_text", |view: &mut TextView| {
                view.set_content("Failed to fetch weather data.");
            });
        }
    }
}

fn main() {
    let rt = Arc::new(Runtime::new().unwrap());
    let siv = Arc::new(Mutex::new(Cursive::default()));

    {
        let siv = Arc::clone(&siv);
        let rt_ref = Arc::clone(&rt);
        let mut siv_lock = siv.lock().unwrap();

        // Create a custom theme with a black background
        let mut theme = siv_lock.current_theme().clone();
        theme.palette[PaletteColor::Background] = Color::Dark(BaseColor::Black); // Set background to black
        theme.palette[PaletteColor::View] = Color::Dark(BaseColor::Black); // Set view background to black
        theme.palette[PaletteColor::Primary] = Color::Light(BaseColor::White); // Text in white
        siv_lock.set_theme(theme);


        // Create the layout with the search bar at the top
        let layout = LinearLayout::vertical()
            .child(
                EditView::new()
                    .on_submit(move |s, query| {
                        let rt_ref = Arc::clone(&rt_ref);
                        update_weather(s, query, rt_ref);
                    })
                    .with_name("search")
                    .fixed_width(30)
            )
            .child(TextView::new("Fetching weather...").with_name("weather_text"));

        siv_lock.add_layer(
            Dialog::new()
                .title("Weather App")
                .content(layout)
                .button("Quit", |s| s.quit()),
        );

        siv_lock.focus_name("search").unwrap();

        update_weather(&mut siv_lock, "auto:ip", Arc::clone(&rt));
    }

    let mut siv_lock = siv.lock().unwrap();
    siv_lock.run();
}