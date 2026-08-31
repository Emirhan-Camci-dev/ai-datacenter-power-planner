/// Static steady-state power flow and basic single-transformer overload check.
pub fn check_transformer_overload(load_kw: f64, transformer_capacity_kva: f64, power_factor: f64) -> bool {
    let apparent_power_kva = load_kw / power_factor;
    apparent_power_kva > transformer_capacity_kva
}
