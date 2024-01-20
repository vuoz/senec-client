use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiData<'a> {
    pub ts: i64,
    pub stat_state: &'a str,
    pub gui_bat_data_power: &'a str,
    pub gui_inverter_power: &'a str,
    pub gui_house_pow: &'a str,
    pub gui_grid_pow: &'a str,
    pub gui_bat_data_fuel_charge: &'a str,
    pub gui_charging_info: &'a str,
    pub gui_boosting_info: &'a str,
}
