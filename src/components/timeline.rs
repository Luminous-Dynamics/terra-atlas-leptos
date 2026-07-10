// Copyright (C) 2024-2026 Tristan Stoltz / Luminous Dynamics
// SPDX-License-Identifier: AGPL-3.0-or-later
// Commercial licensing: see COMMERCIAL_LICENSE.md at repository root

//! The 777-year epoch band. Scrubbing it is the hero interaction — the
//! renderer grows vaults and corridors with the year. The band shows the
//! five epochs as segments; the year reads in gold tabular numerals; the
//! epoch narrative appears as whisper-text only when the epoch CHANGES,
//! not as permanently docked caption furniture.
//!
//!   Foundation (0-25) · Growth (26-50) · Maturation (51-150) ·
//!   Hardening (151-300) · Deep Time (301-777)

use leptos::prelude::*;
use wasm_bindgen::JsCast;

use crate::state::globe_state::GlobeState;

fn epoch_name(y: u32) -> &'static str {
    match y {
        0..=25 => "Foundation",
        26..=50 => "Growth",
        51..=150 => "Maturation",
        151..=300 => "Hardening",
        _ => "Deep Time",
    }
}

fn epoch_detail(y: u32) -> &'static str {
    match y {
        0..=25 => "First geothermal taps online. Survey teams mapping vault sites.",
        26..=50 => "Maglev corridors under construction. Initial vault excavation.",
        51..=150 => "Vault network expanding. Spaceport construction after 4 vaults.",
        151..=300 => "Full maglev grid operational. All 12 vaults active.",
        _ => "Multi-century resilience. Self-sustaining civilization infrastructure.",
    }
}

#[component]
pub fn Timeline() -> impl IntoView {
    let globe_state = expect_context::<GlobeState>();
    let year = globe_state.timeline_year;
    let whisper = globe_state.whisper;

    let on_input = move |ev: web_sys::Event| {
        let Some(target) = ev.target() else { return };
        let input: web_sys::HtmlInputElement = target.unchecked_into();
        let val: u32 = input.value().parse().unwrap_or(0);
        let prev_epoch = epoch_name(year.get_untracked());
        year.set(val);
        // Narrate only on epoch transitions — scrubbing inside an epoch
        // stays silent.
        let new_epoch = epoch_name(val);
        if new_epoch != prev_epoch {
            whisper.set(Some(format!("{new_epoch} — {}", epoch_detail(val))));
        }
    };

    view! {
        <div class="epoch-band">
            <div class="epoch-band-header">
                <span class="epoch-band-year">{move || format!("{:03}", year.get())}</span>
                <span class="epoch-band-name">{move || epoch_name(year.get())}</span>
            </div>
            <div class="epoch-band-track">
                // Epoch segment shading sits behind the range input
                <div class="epoch-band-segments" aria-hidden="true">
                    <span style="flex-grow: 25"></span>
                    <span style="flex-grow: 25"></span>
                    <span style="flex-grow: 100"></span>
                    <span style="flex-grow: 150"></span>
                    <span style="flex-grow: 477"></span>
                </div>
                <input
                    type="range"
                    min="0"
                    max="777"
                    prop:value=move || year.get().to_string()
                    on:input=on_input
                    class="epoch-band-slider"
                    aria-label="Simulation year (0-777)"
                />
            </div>
        </div>
    }
}
