/// Basic fixed-PUE calculator
pub fn calculate_basic_pue(it_load_kw: f64, cooling_overhead_kw: f64, facility_overhead_kw: f64) -> f64 {
    let total_facility_power = it_load_kw + cooling_overhead_kw + facility_overhead_kw;
    total_facility_power / it_load_kw
}
