// Copyright (C) 2024-2026 Tristan Stoltz / Luminous Dynamics
// SPDX-License-Identifier: AGPL-3.0-or-later
// Commercial licensing: see COMMERCIAL_LICENSE.md at repository root

//! "Reactor Digital Twin" demo — replays a static, pre-recorded run of
//! Symthaea's `FissionTwin` HDC anomaly detector
//! (`symthaea-physics::fission_dispatch`) so the pipeline shape (sensor
//! registration -> telemetry -> Orange/Red dispatch order) is visible
//! somewhere in Sol Atlas.
//!
//! **Deliberately not part of the globe.** The globe's real "Nuclear"
//! layer shows actual named operating plants (Palo Verde, Vogtle, ...).
//! This page is a standalone route so a simulated reading is never
//! rendered next to, or confused with, a real one. See
//! `symthaea/NUCLEAR_ENERGY_PLAN_2026-07-06.md` Phase 4 for the full
//! context and the honest finding that this scenario's free energy never
//! reaches Orange on its own (the last tick is a hand-built capstone,
//! flagged `synthetic` in the data and in this UI).

use leptos::prelude::*;
use leptos_router::components::A;

use crate::data::reactor_twin::{ReactorTwinTick, load_ticks};

const TICK_INTERVAL_MS: u64 = 400;

#[component]
pub fn ReactorTwinDemo() -> impl IntoView {
    let ticks = load_ticks();
    let len = ticks.len();
    let ticks = StoredValue::new(ticks);
    let index = RwSignal::new(0usize);

    Effect::new(move |_| {
        if len == 0 {
            return;
        }
        let handle = set_interval_with_handle(
            move || {
                index.update(|i| *i = (*i + 1) % len);
            },
            std::time::Duration::from_millis(TICK_INTERVAL_MS),
        );
        on_cleanup(move || {
            if let Ok(h) = handle {
                h.clear();
            }
        });
    });

    let current = move || ticks.with_value(|t| t.get(index.get()).cloned());

    view! {
        <div class="reactor-twin-page">
            <A href="/">"\u{2190} Back to globe"</A>
            <h1>"Reactor Digital Twin — Simulator Demo"</h1>
            <p class="reactor-twin-disclaimer">
                "This is a "<strong>"replayed simulation"</strong>", not live plant data. "
                "Every value below comes from one recorded run of Symthaea's FissionTwin "
                "HDC anomaly detector (a research simulator, "
                "symthaea-physics::fission_dispatch), looping for demonstration. It is "
                "not connected to any real reactor, and not associated with any plant "
                "shown on the globe's Nuclear layer or any SMR investment listing."
            </p>
            {move || match current() {
                None => view! { <p>"No data."</p> }.into_any(),
                Some(tick) => view! { <ReactorTwinTickView tick=tick/> }.into_any(),
            }}
        </div>
    }
}

#[component]
fn ReactorTwinTickView(tick: ReactorTwinTick) -> impl IntoView {
    let safety_class = format!("safety-{}", tick.safety_level.to_lowercase());
    view! {
        <div class="reactor-twin-panel">
            <div class=format!("reactor-twin-safety {safety_class}")>
                {tick.safety_level.clone()}
                {tick.synthetic.then(|| view! {
                    <span class="reactor-twin-synthetic-tag">
                        " (synthetic capstone — this run never reached Orange on its own)"
                    </span>
                })}
            </div>
            <div class="reactor-twin-clock">"t = " {tick.t_seconds} "s"</div>
            <div class="reactor-twin-grid">
                <ReactorTwinField label="Power output" value=format!("{:.2}", tick.power_output)/>
                <ReactorTwinField label="Coolant temp" value=format!("{:.1}", tick.coolant_temp)/>
                <ReactorTwinField label="Neutron flux" value=format!("{:.2}", tick.neutron_flux)/>
                <ReactorTwinField label="Pressure" value=format!("{:.1}", tick.pressure)/>
                <ReactorTwinField label="Control rod pos" value=format!("{:.2}", tick.control_rod_pos)/>
                <ReactorTwinField label="Free energy" value=format!("{:.3}", tick.free_energy)/>
                <ReactorTwinField label="Confidence" value=format!("{:.0}%", tick.confidence * 100.0)/>
            </div>
            {tick.dispatch_order.as_ref().map(|order| {
                let priority = order.priority_label.clone();
                let description = order.description.clone();
                view! {
                    <div class="reactor-twin-dispatch">
                        <h3>"Simulated dispatch order (" {priority} ")"</h3>
                        <p>{description}</p>
                    </div>
                }
            })}
        </div>
    }
}

#[component]
fn ReactorTwinField(label: &'static str, value: String) -> impl IntoView {
    view! {
        <div class="reactor-twin-field">
            <span class="reactor-twin-field-label">{label}</span>
            <span class="reactor-twin-field-value">{value}</span>
        </div>
    }
}
