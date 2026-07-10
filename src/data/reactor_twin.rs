// Copyright (C) 2024-2026 Tristan Stoltz / Luminous Dynamics
// SPDX-License-Identifier: AGPL-3.0-or-later
// Commercial licensing: see COMMERCIAL_LICENSE.md at repository root

//! Static replay data for the "Reactor Digital Twin" demo page.
//!
//! This JSON is **not fetched live and not generated at runtime**. It's a
//! one-time export of a real run of Symthaea's `FissionTwin` simulator
//! (`symthaea/crates/domains/symthaea-physics`, via
//! `cargo run --release --example reactor_twin_export -p symthaea-physics`)
//! checked in as a static asset and replayed client-side on a timer. See
//! `crate::components::reactor_twin_demo` for the page that plays it back
//! and why this is deliberately kept separate from the globe's real
//! "Nuclear" data layer (`static_data::load_all().nuclear_sites`, real
//! named operating plants).

use serde::Deserialize;

const REACTOR_TWIN_DEMO_JSON: &str = include_str!("../../assets/data/reactor-twin-demo.json");

#[derive(Debug, Clone, Deserialize)]
pub struct ReactorTwinDispatchOrder {
    pub priority_label: String,
    pub description: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ReactorTwinTick {
    pub t_seconds: u32,
    pub power_output: f64,
    pub coolant_temp: f64,
    pub neutron_flux: f64,
    pub pressure: f64,
    pub control_rod_pos: f64,
    pub free_energy: f64,
    pub confidence: f64,
    pub safety_level: String,
    /// True only for the hand-built capstone tick this real run's
    /// steady-state scenario never reached on its own — see module docs.
    pub synthetic: bool,
    pub dispatch_order: Option<ReactorTwinDispatchOrder>,
}

pub fn load_ticks() -> Vec<ReactorTwinTick> {
    serde_json::from_str(REACTOR_TWIN_DEMO_JSON).unwrap_or_else(|e| {
        log::error!("reactor-twin-demo.json failed to parse (page will show 'No data'): {e}");
        Vec::new()
    })
}
