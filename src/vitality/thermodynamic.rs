// Copyright (C) 2024-2026 Tristan Stoltz / Luminous Dynamics
// SPDX-License-Identifier: AGPL-3.0-or-later
// Commercial licensing: see COMMERCIAL_LICENSE.md at repository root

//! Thermodynamic UI coupling: device energy -> CSS variables + renderer.
//!
//! The Battery Status API is a fingerprinting vector: available on
//! Chrome/Android, disabled in Safari and Firefox. Its absence is handled
//! gracefully — full energy, no torpor.

use super::set_css_var;
use gloo_timers::callback::Interval;
use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

/// Below this battery level the UI enters torpor.
const TORPOR_BATTERY_THRESHOLD: f64 = 0.15;

/// Battery polling interval (ms).
const BATTERY_POLL_MS: u32 = 10_000;

/// Reactive state for thermodynamic coupling.
#[derive(Clone, Copy)]
pub struct ThermodynamicState {
    /// Device battery level 0.0-1.0 (1.0 if the API is unavailable).
    pub device_energy: ReadSignal<f64>,
    /// Torpor level 0.0-1.0 (0.0 = fully active, 1.0 = deep torpor).
    pub torpor_level: ReadSignal<f64>,
    /// Whether the Battery API is available.
    pub battery_available: ReadSignal<bool>,
}

/// Initialize thermodynamic coupling, provide it as context, and mirror it
/// into `--device-energy` / `--torpor-level` CSS variables.
pub fn provide_thermodynamic_context() -> ThermodynamicState {
    let (device_energy, set_device_energy) = signal(1.0_f64);
    let (torpor_level, set_torpor_level) = signal(0.0_f64);
    let (battery_available, set_battery_available) = signal(false);

    spawn_local(async move {
        match try_get_battery().await {
            Some(battery) => {
                set_battery_available.set(true);
                let level = battery.level();
                set_device_energy.set(level);
                update_torpor(level, &set_torpor_level);

                let battery_clone = battery.clone();
                let interval = Interval::new(BATTERY_POLL_MS, move || {
                    let level = battery_clone.level();
                    set_device_energy.set(level);
                    update_torpor(level, &set_torpor_level);
                });
                interval.forget(); // lives for app lifetime
            }
            None => {
                log::info!("vitality: Battery API unavailable — defaulting to full energy");
                set_battery_available.set(false);
            }
        }
    });

    let state = ThermodynamicState {
        device_energy,
        torpor_level,
        battery_available,
    };

    provide_context(state);

    Effect::new(move |_| {
        set_css_var("--device-energy", &format!("{:.3}", device_energy.get()));
        set_css_var("--torpor-level", &format!("{:.3}", torpor_level.get()));
    });

    state
}

fn update_torpor(battery_level: f64, set_torpor: &WriteSignal<f64>) {
    if battery_level < TORPOR_BATTERY_THRESHOLD {
        let torpor = 1.0 - (battery_level / TORPOR_BATTERY_THRESHOLD);
        set_torpor.set(torpor.clamp(0.0, 1.0));
    } else {
        set_torpor.set(0.0);
    }
}

async fn try_get_battery() -> Option<web_sys::BatteryManager> {
    let window = web_sys::window()?;
    let navigator = window.navigator();
    let get_battery = js_sys::Reflect::get(&navigator, &JsValue::from_str("getBattery")).ok()?;
    if get_battery.is_undefined() || get_battery.is_null() {
        return None;
    }
    let func: js_sys::Function = get_battery.dyn_into().ok()?;
    let promise: js_sys::Promise = func.call0(&navigator).ok()?.dyn_into().ok()?;
    let result = wasm_bindgen_futures::JsFuture::from(promise).await.ok()?;
    result.dyn_into().ok()
}
