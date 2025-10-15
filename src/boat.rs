use serde::{Deserialize, Serialize};

type ErrorCode = Option<String>; // "0x0174"
type ErrorCodeMessage = Option<Vec<String>>; // ["Battery", "The battery is fully charged", "info"]
type ErrorCodeTime = Option<String>; // "Mon, 22 Sep 2025 07:21:16 GMT"

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoatStatus {
    pub afs_retraction_state: u16,
    pub aps3_error_code: ErrorCode,
    pub aps3_error_code_msg: ErrorCodeMessage,
    pub aps3_error_code_time: ErrorCodeTime,
    pub aux_supply_voltage: f32,
    pub battery_soc: f32,
    pub battery_voltage: f32,
    pub bms_charger_total_output_power: f32,
    pub bms_error_last_seen_code: ErrorCode,
    pub bms_error_last_seen_code_msg: ErrorCodeMessage,
    pub bms_error_last_seen_code_time: ErrorCodeTime,
    pub bms_flag_fully_charged_latched: u32,
    pub bms_flag_fully_charged_latched_soc: f32,
    pub bms_flag_fully_charged_msg: ErrorCodeMessage,
    pub bms_flag_fully_charged_time: ErrorCodeTime,
    pub charger_present: bool,
    pub charger_status_flags: ErrorCode,
    pub charger_status_flags_msg: ErrorCodeMessage,
    pub charger_status_flags_time: ErrorCodeTime,
    pub charging: bool,
    pub dcu_board_error_code: ErrorCode,
    pub dcu_board_error_code_msg: ErrorCodeMessage,
    pub dcu_board_error_code_time: ErrorCodeTime,
    pub ffs_retraction_state: u16,
    pub flight_controller_board_error_code: ErrorCode,
    pub flight_controller_board_error_code_msg: ErrorCodeMessage,
    pub flight_controller_board_error_code_time: ErrorCodeTime,
    pub geofence_msg: ErrorCodeMessage,
    pub inverter_aft_error_code: ErrorCode,
    pub inverter_aft_error_code_msg: ErrorCodeMessage,
    pub inverter_aft_error_code_time: ErrorCodeTime,
    pub inverter_error_code: ErrorCode,
    pub inverter_error_code_msg: ErrorCodeMessage,
    pub inverter_error_code_time: ErrorCodeTime,
    pub inverter_front_error_code: ErrorCode,
    pub inverter_front_error_code_msg: ErrorCodeMessage,
    pub inverter_front_error_code_time: ErrorCodeTime,
    pub kwh_remaining: f32,
    pub power_consumption_kwh_nm: f32,
    pub radar_0_board_error_code: ErrorCode,
    pub radar_0_board_error_code_msg: ErrorCodeMessage,
    pub radar_0_board_error_code_time: ErrorCodeTime,
    pub radar_1_board_error_code: ErrorCode,
    pub radar_1_board_error_code_msg: ErrorCodeMessage,
    pub radar_1_board_error_code_time: ErrorCodeTime,
    pub range_nm: f32,
    pub retraction_board_error_code: ErrorCode,
    pub retraction_board_error_code_msg: ErrorCodeMessage,
    pub retraction_board_error_code_time: ErrorCodeTime,
    pub time: ErrorCodeTime,
    pub time_to_full_min: Option<String>,
    pub valve_controller_error_code: ErrorCode,
    pub valve_controller_error_code_msg: ErrorCodeMessage,
    pub valve_controller_error_code_time: ErrorCodeTime,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_boat_status() {
        let s = include_str!("../tests/data/boat_status.json");
        let status: BoatStatus = serde_json::from_str(&s).unwrap();

        assert_eq!(15, status.afs_retraction_state);
        assert_eq!(None, status.time_to_full_min);
    }
}
