// Copyright (C) 2024-2026 Tristan Stoltz / Luminous Dynamics
// SPDX-License-Identifier: AGPL-3.0-or-later
// Commercial licensing: see COMMERCIAL_LICENSE.md at repository root

//! Homeostasis, reinterpreted for an atlas: stillness itself is the state.
//!
//! mycelix apps derive homeostasis from empty work queues; Sol Atlas has no
//! work queue, so the organism instead settles when the human stops
//! interacting. After [`IDLE_SECS`] without pointer/wheel/key/touch input,
//! `--homeostasis` eases to 1 and the chrome breathes out of the way,
//! leaving the globe alone with its slow rotation. Any interaction wakes it.

use super::set_css_var;
use gloo_timers::callback::Interval;
use leptos::prelude::*;
use std::cell::Cell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

/// Seconds of stillness before the UI settles into homeostasis.
const IDLE_SECS: f64 = 45.0;

#[derive(Clone, Copy)]
pub struct HomeostasisState {
    /// True when the UI has settled (no interaction for IDLE_SECS).
    pub in_homeostasis: ReadSignal<bool>,
}

/// Initialize idle-based homeostasis and mirror it into `--homeostasis`.
pub fn provide_homeostasis_context() -> HomeostasisState {
    let (in_homeostasis, set_in_homeostasis) = signal(false);

    let last_touch: Rc<Cell<f64>> = Rc::new(Cell::new(now_secs()));

    if let Some(document) = web_sys::window().and_then(|w| w.document()) {
        for event in ["pointerdown", "wheel", "keydown", "touchstart"] {
            let lt = last_touch.clone();
            let wake = Closure::wrap(Box::new(move |_: web_sys::Event| {
                lt.set(now_secs());
            }) as Box<dyn FnMut(web_sys::Event)>);
            let _ = document.add_event_listener_with_callback(event, wake.as_ref().unchecked_ref());
            wake.forget(); // lives for app lifetime
        }
    }

    let lt = last_touch.clone();
    let interval = Interval::new(1_000, move || {
        let settled = now_secs() - lt.get() > IDLE_SECS;
        set_in_homeostasis.set(settled);
    });
    interval.forget();

    Effect::new(move |_| {
        set_css_var(
            "--homeostasis",
            if in_homeostasis.get() { "1" } else { "0" },
        );
    });

    let state = HomeostasisState { in_homeostasis };
    provide_context(state);
    state
}

fn now_secs() -> f64 {
    js_sys::Date::now() / 1000.0
}
