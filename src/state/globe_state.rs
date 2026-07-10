// Copyright (C) 2024-2026 Tristan Stoltz / Luminous Dynamics
// SPDX-License-Identifier: AGPL-3.0-or-later
// Commercial licensing: see COMMERCIAL_LICENSE.md at repository root
use leptos::prelude::*;
use std::collections::HashSet;

use crate::components::planet_nav::PlanetTarget;
use crate::data::types::{HoverInfo, Layer, SelectedItem};

#[derive(Clone)]
pub struct GlobeState {
    pub active_layers: RwSignal<HashSet<Layer>>,
    pub hovered: RwSignal<Option<HoverInfo>>,
    pub selected: RwSignal<Option<SelectedItem>>,
    pub mouse_pos: RwSignal<(f32, f32)>,
    pub timeline_year: RwSignal<u32>,
    pub show_core: RwSignal<bool>,
    pub focused_planet: RwSignal<Option<PlanetTarget>>,
    /// One-line ephemeral message (provenance on layer toggle, epoch
    /// narrative on timeline change) shown as whisper-text bottom-center.
    /// The whisper component clears it after a few breaths.
    pub whisper: RwSignal<Option<String>>,
}

impl GlobeState {
    pub fn new() -> Self {
        // Default: real data only (USACE dams + NRC SMR pipeline).
        // Truth before theater — scenario layers are opt-in.
        let mut default_layers = HashSet::new();
        default_layers.insert(Layer::Energy);

        Self {
            active_layers: RwSignal::new(default_layers),
            hovered: RwSignal::new(None),
            selected: RwSignal::new(None),
            mouse_pos: RwSignal::new((0.0, 0.0)),
            timeline_year: RwSignal::new(150), // default: Maturation epoch
            show_core: RwSignal::new(false),
            focused_planet: RwSignal::new(None),
            whisper: RwSignal::new(None),
        }
    }

    pub fn toggle_layer(&self, layer: Layer) {
        self.active_layers.update(|layers| {
            if layers.contains(&layer) {
                layers.remove(&layer);
            } else {
                layers.insert(layer);
            }
        });
    }

    pub fn is_layer_active(&self, layer: Layer) -> bool {
        self.active_layers.read().contains(&layer)
    }
}
