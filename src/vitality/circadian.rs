// Copyright (C) 2024-2026 Tristan Stoltz / Luminous Dynamics
// SPDX-License-Identifier: AGPL-3.0-or-later
// Commercial licensing: see COMMERCIAL_LICENSE.md at repository root

//! Circadian awareness: a sun-facing atlas should know what time it is.
//!
//! Dawn = golden warmth, Day = full clarity, Dusk = deepening amber,
//! Night = dim and restful. Feeds `--circadian-warmth` / `--circadian-
//! brightness`, which the stylesheet folds into text opacity and glow.

use super::set_css_var;
use gloo_timers::callback::Interval;
use leptos::prelude::*;

/// Circadian phase based on local browser time.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CircadianPhase {
    /// 5am-9am
    Dawn,
    /// 9am-5pm
    Day,
    /// 5pm-9pm
    Dusk,
    /// 9pm-5am
    Night,
}

impl CircadianPhase {
    pub fn from_hour(hour: u32) -> Self {
        match hour {
            5..=8 => Self::Dawn,
            9..=16 => Self::Day,
            17..=20 => Self::Dusk,
            _ => Self::Night,
        }
    }

    pub fn warmth(&self) -> f64 {
        match self {
            Self::Dawn => 1.15,
            Self::Day => 1.0,
            Self::Dusk => 1.1,
            Self::Night => 0.85,
        }
    }

    pub fn brightness(&self) -> f64 {
        match self {
            Self::Dawn => 0.95,
            Self::Day => 1.0,
            Self::Dusk => 0.9,
            Self::Night => 0.8,
        }
    }
}

#[derive(Clone, Copy)]
pub struct CircadianState {
    pub phase: ReadSignal<CircadianPhase>,
    pub hour: ReadSignal<u32>,
}

/// Initialize circadian awareness; refreshes every 60 seconds.
pub fn provide_circadian_context() -> CircadianState {
    let current_hour = get_local_hour();
    let (hour, set_hour) = signal(current_hour);
    let (phase, set_phase) = signal(CircadianPhase::from_hour(current_hour));

    let interval = Interval::new(60_000, move || {
        let h = get_local_hour();
        set_hour.set(h);
        set_phase.set(CircadianPhase::from_hour(h));
    });
    interval.forget(); // lives for app lifetime

    Effect::new(move |_| {
        let p = phase.get();
        set_css_var("--circadian-warmth", &format!("{:.3}", p.warmth()));
        set_css_var("--circadian-brightness", &format!("{:.3}", p.brightness()));
    });

    let state = CircadianState { phase, hour };
    provide_context(state);
    state
}

fn get_local_hour() -> u32 {
    js_sys::Date::new_0().get_hours()
}
