// Core normalized metrics shared by all WBTC shards.
// Source-of-truth ranges must match MetricFields.v1 ALN schema.

#[derive(Clone, Copy, Debug)]
pub struct MetricFields {
    /// Knowledge-factor K ∈ [0,1]
    pub k: f64,
    /// Eco-impact E ∈ [0,1]
    pub e: f64,
    /// Risk-of-harm R ∈ [0,1]
    pub r: f64,
    /// Normalized risk coordinates r_x[i] ∈ [0,1]
    pub rx: Vec<f64>,
    /// Lyapunov residual V_t ≥ 0
    pub vt: f64,
}

impl MetricFields {
    /// Cheap structural sanity check (CI should call this before using the fields).
    pub fn is_well_formed(&self) -> bool {
        fn in_01(x: f64) -> bool {
            x.is_finite() && x >= 0.0 && x <= 1.0
        }

        in_01(self.k)
            && in_01(self.e)
            && in_01(self.r)
            && self.vt.is_finite()
            && self.vt >= 0.0
            && !self.rx.is_empty()
            && self.rx.iter().all(|v| in_01(*v))
    }
}
