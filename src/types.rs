use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiData {
    pub ts: i64,
    pub stat_state: i64,
    pub gui_bat_data_power: i64,
    pub gui_inverter_power: f64,
    pub gui_house_pow: f64,
    pub gui_grid_pow: f64,
    pub gui_bat_data_fuel_charge: f64,
    pub gui_charging_info: i64,
    pub gui_boosting_info: i64,
}
