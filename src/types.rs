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
