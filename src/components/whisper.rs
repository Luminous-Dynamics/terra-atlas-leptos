// Copyright (C) 2024-2026 Tristan Stoltz / Luminous Dynamics
// SPDX-License-Identifier: AGPL-3.0-or-later
// Commercial licensing: see COMMERCIAL_LICENSE.md at repository root

//! Whisper-text: one quiet line, bottom-center, that breathes in when
//! something has context to offer (layer provenance, epoch narrative) and
//! breathes out again after a few seconds. Replaces permanently-docked
//! disclaimer furniture — honesty without noise.

use gloo_timers::callback::Timeout;
use leptos::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

use crate::state::globe_state::GlobeState;

/// How long a whisper lingers before breathing out (ms).
const WHISPER_MS: u32 = 4_500;

#[component]
pub fn Whisper() -> impl IntoView {
    let globe_state = expect_context::<GlobeState>();
    let whisper = globe_state.whisper;

    // Last displayed text is kept so the fade-out doesn't blank mid-breath.
    let (text, set_text) = signal(String::new());
    let (visible, set_visible) = signal(false);

    let pending: Rc<RefCell<Option<Timeout>>> = Rc::new(RefCell::new(None));

    Effect::new(move |_| {
        if let Some(msg) = whisper.get() {
            set_text.set(msg);
            set_visible.set(true);
            // Restart the breath-out timer; dropping a Timeout cancels it.
            let timeout = Timeout::new(WHISPER_MS, move || {
                set_visible.set(false);
                whisper.set(None);
            });
            *pending.borrow_mut() = Some(timeout);
        }
    });

    view! {
        <div class="whisper" class:visible=visible aria-live="polite">
            {text}
        </div>
    }
}
