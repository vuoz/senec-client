use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiData {
    pub ts: i64,
    pub stat_state: f64,
    pub gui_bat_data_power: f64,
    pub gui_inverter_power: f64,
    pub gui_house_pow: f64,
    pub gui_grid_pow: f64,
    pub gui_bat_data_fuel_charge: i64,
    pub gui_charging_info: f64,
    pub gui_boosting_info: f64,
}
