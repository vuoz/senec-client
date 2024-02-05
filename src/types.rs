use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiData<'a> {
    pub ts: &'a str,
    pub stat_state: &'a str,
    pub gui_bat_data_power: &'a str,
    pub gui_inverter_power: &'a str,
    pub gui_house_pow: &'a str,
    pub gui_grid_pow: &'a str,
    pub gui_bat_data_fuel_charge: &'a str,
    pub gui_charging_info: &'a str,
    pub gui_boosting_info: &'a str,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiDataWithWeather<'a> {
    pub ts: &'a str,
    pub stat_state: &'a str,

    // this is the power used to charge the battery
    pub gui_bat_data_power: &'a str,
    pub gui_inverter_power: &'a str,
    pub gui_house_pow: &'a str,
    pub gui_grid_pow: &'a str,
    pub gui_bat_data_fuel_charge: &'a str,
    pub gui_charging_info: &'a str,
    pub gui_boosting_info: &'a str,
    pub weather: Weather<'a>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TotalData<'a> {
    pub consumption: &'a str,
    pub generated: &'a str,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Weather<'a> {
    pub latitude: i64,
    pub longitude: i64,
    pub generationtime_ms: i64,
    pub utc_offset_seconds: i64,
    pub timezone: &'a str,
    pub timezone_abbreviation: &'a str,
    pub elevation: i64,
    pub hourly_units: Hourly<'a>,
    pub hourly: Hourly<'a>,
    pub daily_units: Daily<'a>,
    pub daily: Daily<'a>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Daily<'a> {
    pub time: Option<&'a str>,
    pub sunrise: Option<&'a str>,
    pub sunset: Option<&'a str>,
    pub sunshine_duration: Option<&'a str>,
    pub uv_index_max: Option<&'a str>,
    pub uv_index_clear_sky_max: Option<&'a str>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hourly<'a> {
    pub time: Option<&'a str>,
    pub cloud_cover: Option<&'a str>,
    pub visibility: Option<&'a str>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiDataWithWeatherNew<'a> {
    pub ts: &'a str,
    pub stat_state: &'a str,

    // this is the power used to charge the battery
    pub gui_bat_data_power: &'a str,
    pub gui_inverter_power: &'a str,
    pub gui_house_pow: &'a str,
    pub gui_grid_pow: &'a str,
    pub gui_bat_data_fuel_charge: &'a str,
    pub gui_charging_info: &'a str,
    pub gui_boosting_info: &'a str,
    pub weather: ApiRespHourly<'a>,

    pub total_data: TotalData<'a>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiRespHourly<'a> {
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub generationtime_ms: Option<f64>,
    pub utc_offset_seconds: Option<f64>,
    pub timezone: Option<&'a str>,
    pub timezone_abbreviation: Option<&'a str>,
    pub elevation: Option<f64>,
    pub hourly_units: Option<HourlyUnitsForRespHourly<'a>>,
    pub hourly: Option<HourlyForRespHourly>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HourlyForRespHourly {
    pub time: Option<Vec<Vec<u8>>>,
    pub temperature_2m: Option<Vec<f64>>,
    pub rain: Option<Vec<f64>>,
    pub showers: Option<Vec<f64>>,
    pub cloud_cover: Option<Vec<f64>>,
    pub uv_index: Option<Vec<f64>>,
    pub uv_index_clear_sky: Option<Vec<f64>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HourlyUnitsForRespHourly<'a> {
    pub time: Option<&'a str>,
    pub temperature_2m: Option<&'a str>,
    pub rain: Option<&'a str>,
    pub showers: Option<&'a str>,
    pub cloud_cover: Option<&'a str>,
    pub uv_index: Option<&'a str>,
    pub uv_index_clear_sky: Option<&'a str>,
}
