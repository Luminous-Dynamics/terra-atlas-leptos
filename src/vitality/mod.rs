// Copyright (C) 2024-2026 Tristan Stoltz / Luminous Dynamics
// SPDX-License-Identifier: AGPL-3.0-or-later
// Commercial licensing: see COMMERCIAL_LICENSE.md at repository root

//! Vitality: the biological substrate coupling for the Sol Atlas UI.
//!
//! The UI is a living organism, not a dashboard — it is physically
//! constrained by its substrate (battery), aware of the sun (circadian),
//! and rewards stillness (homeostasis). Low energy means torpor (dimmed
//! glow, slower frames), never error popups.
//!
//! Vendored/adapted from `mycelix-workspace/crates/mycelix-leptos-core`
//! (`thermodynamic.rs`, `util.rs`) and `mycelix-hearth`'s `circadian.rs`
//! rather than depended upon: the standalone GitHub Pages repo
//! (terra-atlas-leptos) cannot reach monorepo path dependencies, and these
//! systems are deliberately zero-transport (no Holochain, no network).
//! Homeostasis is reinterpreted for an atlas: stillness = no interaction,
//! not an empty work queue.

mod circadian;
mod homeostasis;
mod thermodynamic;

pub use circadian::{CircadianPhase, CircadianState, provide_circadian_context};
pub use homeostasis::{HomeostasisState, provide_homeostasis_context};
pub use thermodynamic::{ThermodynamicState, provide_thermodynamic_context};

use wasm_bindgen::JsCast;

/// Set a CSS custom property on the document root element.
pub fn set_css_var(name: &str, value: &str) {
    if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
            if let Some(root) = document.document_element() {
                if let Some(el) = root.dyn_ref::<web_sys::HtmlElement>() {
                    let _ = el.style().set_property(name, value);
                }
            }
        }
    }
}

/// Wire up every vitality subsystem and provide their contexts.
pub fn provide_vitality() -> ThermodynamicState {
    provide_circadian_context();
    provide_homeostasis_context();
    provide_thermodynamic_context()
}
