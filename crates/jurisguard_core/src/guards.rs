use crate::metrics::MetricFields;

/// Result of checking Lyapunov and per-axis hard limits for one step.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResidualCheck {
    Ok,
    ViolatedAxis,     // some r_x,i > 1 (should never happen if schema is respected)
    IncreasedResidual // V_{t+1} > V_t
}

/// Hard ecosafety predicate:
/// - rejects any transition with r_x,i(t+1) > 1.0
/// - rejects any transition with V_{t+1} > V_t
pub fn residual_ok(prev: &MetricFields, next: &MetricFields) -> ResidualCheck {
    // Per-axis hard corridor check (defensive: allow tiny epsilon for FP error).
    let axis_violation = next
        .rx
        .iter()
        .any(|&v| !v.is_finite() || v > 1.0 + 1e-9);

    if axis_violation {
        return ResidualCheck::ViolatedAxis;
    }

    // Lyapunov invariant: V_{t+1} <= V_t
    if !next.vt.is_finite() || next.vt > prev.vt + 1e-9 {
        return ResidualCheck::IncreasedResidual;
    }

    ResidualCheck::Ok
}
