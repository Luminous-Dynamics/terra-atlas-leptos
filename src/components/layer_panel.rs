// Copyright (C) 2024-2026 Tristan Stoltz / Luminous Dynamics
// SPDX-License-Identifier: AGPL-3.0-or-later
// Commercial licensing: see COMMERCIAL_LICENSE.md at repository root
use leptos::prelude::*;

use crate::data::types::{DataKind, Layer};
use crate::state::globe_state::GlobeState;

#[component]
pub fn LayerPanel() -> impl IntoView {
    let globe_state = expect_context::<GlobeState>();
    let layers = [
        Layer::Geothermal,
        Layer::Maglev,
        Layer::ResontiaVaults,
        Layer::TerraLumina,
        Layer::Energy,
        Layer::Regions,
        Layer::SupplyChain,
        Layer::Climate,
        Layer::Emergency,
        Layer::Health,
        Layer::Robotics,
        Layer::FossilDeposits,
        Layer::Nuclear,
    ];

    view! {
        <div class="layer-panel">
            <h3>"Layers"</h3>
            {layers.into_iter().map(|layer| {
                let gs = globe_state.clone();
                let gs_click = globe_state.clone();
                let gs_change = globe_state.clone();
                let active = move || gs.active_layers.read().contains(&layer);
                let prov = layer.provenance();

                view! {
                    // Row click toggles; the checkbox handles its own change
                    // (keyboard included) and stops click propagation so a
                    // direct checkbox click doesn't double-toggle via the row.
                    <div class="layer-toggle" on:click=move |_| gs_click.toggle_layer(layer) title=prov.summary()>
                        <span class="layer-dot" style=move || format!("background: {}", if active() { layer.css_color() } else { "#333" }) />
                        <input
                            type="checkbox"
                            prop:checked=active
                            aria-label=layer.label()
                            on:click=|e| e.stop_propagation()
                            on:change=move |_| gs_change.toggle_layer(layer)
                        />
                        <label>{layer.label()}</label>
                        {(prov.kind == DataKind::Scenario).then(|| view! {
                            <span
                                title="Speculative planning fiction — these do not exist"
                                style="margin-left: 6px; font-size: 9px; letter-spacing: 1px; text-transform: uppercase; color: rgba(251,191,36,0.8); border: 1px solid rgba(251,191,36,0.4); border-radius: 3px; padding: 0 4px"
                            >"scenario"</span>
                        })}
                    </div>
                }
            }).collect::<Vec<_>>()}

            <div style="margin-top: 10px; font-size: 10px; line-height: 1.5; opacity: 0.6">
                "Data are static snapshots, not live feeds — hover a layer for source and date. Layers tagged "
                <span style="color: rgba(251,191,36,0.9)">"scenario"</span>
                " are speculative planning fiction."
            </div>

            <div style="margin-top: 12px; padding-top: 10px; border-top: 1px solid rgba(0,255,136,0.1); font-size: 11px; opacity: 0.7">
                <div style="margin-bottom: 6px; font-weight: bold">"EROI Legend"</div>
                <div style="display: flex; align-items: center; gap: 4px; margin: 2px 0">
                    <span style="width: 8px; height: 8px; border-radius: 50%; background: rgb(15,186,130); display: inline-block" />
                    <span>"> 12:1 Civilization"</span>
                </div>
                <div style="display: flex; align-items: center; gap: 4px; margin: 2px 0">
                    <span style="width: 8px; height: 8px; border-radius: 50%; background: rgb(250,191,36); display: inline-block" />
                    <span>"5-12:1 Sustainable"</span>
                </div>
                <div style="display: flex; align-items: center; gap: 4px; margin: 2px 0">
                    <span style="width: 8px; height: 8px; border-radius: 50%; background: rgb(240,69,69); display: inline-block" />
                    <span>"3-5:1 Marginal"</span>
                </div>
                <div style="display: flex; align-items: center; gap: 4px; margin: 2px 0">
                    <span style="width: 8px; height: 8px; border-radius: 50%; background: rgb(128,26,26); display: inline-block" />
                    <span>"< 3:1 Unviable"</span>
                </div>
            </div>

            <div style="margin-top: 12px; padding-top: 10px; border-top: 1px solid rgba(0,255,136,0.1)">
                <div class="layer-toggle" on:click=move |_| globe_state.show_core.update(|v| *v = !*v)>
                    <span class="layer-dot" style=move || format!("background: {}", if globe_state.show_core.get() { "#FFD700" } else { "#333" }) />
                    <input type="checkbox" prop:checked=move || globe_state.show_core.get() />
                    <label>"Core Cutaway"</label>
                </div>
            </div>
        </div>
    }
}
