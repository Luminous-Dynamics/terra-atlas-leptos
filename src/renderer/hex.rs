// Copyright (C) 2024-2026 Tristan Stoltz / Luminous Dynamics
// SPDX-License-Identifier: AGPL-3.0-or-later
// Commercial licensing: see COMMERCIAL_LICENSE.md at repository root

//! H3 hexagonal grid picking — hover a cell, click to drill in. Ported
//! from `sol-atlas-bevy/src/h3_grid.rs` + `cell_entry.rs` onto this
//! crate's existing WebGL2 ray-sphere picking (`super::picking`) instead
//! of Bevy's ray/gizmo APIs. Turns the globe from a display into an
//! instrument: resolution tracks camera distance continuously, so
//! repeated clicks reveal progressively finer hexes with no resolution
//! "pop" independent of the zoom motion itself.

use super::math::Vec3;
use h3o::{CellIndex, LatLng, Resolution};

/// Camera distance (globe radii) at/beyond which hex resolution bottoms
/// out at its coarsest — matches this crate's `ZOOM_MAX`.
const RESOLUTION_FAR_DISTANCE: f32 = 8.0;
/// Camera distance at/within which hex resolution caps at its finest —
/// the drill-in floor `drill_target_distance` converges to.
const RESOLUTION_NEAR_DISTANCE: f32 = 1.05;
const RESOLUTION_MIN: u8 = 0;
/// Capped well below H3's max (15) — past this a hex is smaller than a
/// single screen pixel would resolve at any distance this globe is ever
/// rendered from.
const RESOLUTION_MAX: u8 = 9;

/// Each click zooms the camera to this fraction of its current distance.
const DRILL_ZOOM_FACTOR: f32 = 0.4;

/// Maps camera distance to H3 resolution — coarse (whole-region hexes)
/// from orbit, progressively finer as the camera approaches the surface.
pub fn resolution_for_distance(distance: f32) -> Resolution {
    let t = ((RESOLUTION_FAR_DISTANCE - distance)
        / (RESOLUTION_FAR_DISTANCE - RESOLUTION_NEAR_DISTANCE))
        .clamp(0.0, 1.0);
    let level = RESOLUTION_MIN as f32 + t * (RESOLUTION_MAX - RESOLUTION_MIN) as f32;
    Resolution::try_from(level.round() as u8).unwrap_or(Resolution::Zero)
}

/// Convert lat/lon (degrees) to the containing H3 cell at the given
/// resolution. `None` only for non-finite input (h3o's own validity check).
pub fn latlon_to_cell(lat: f64, lon: f64, resolution: Resolution) -> Option<CellIndex> {
    LatLng::new(lat, lon).ok().map(|ll| ll.to_cell(resolution))
}

/// The H3 cell under a screen-space ray hit on the globe, at the
/// resolution appropriate for the camera's current distance. Returns
/// `None` if the ray misses the sphere or the hit doesn't resolve to a
/// valid cell.
pub fn cell_at_hit(hit: Vec3, camera_distance: f32) -> Option<CellIndex> {
    let (lat, lon) = sol_atlas_core::geo::xyz_to_lat_lon([hit.x, hit.y, hit.z], 1.0);
    latlon_to_cell(lat, lon, resolution_for_distance(camera_distance))
}

/// The cell's boundary vertices projected onto a sphere of the given
/// radius, flattened to `[x, y, z, x, y, z, ...]` for direct VBO upload.
pub fn boundary_positions(cell: CellIndex, radius: f64) -> Vec<f32> {
    cell.boundary()
        .iter()
        .flat_map(|ll| sol_atlas_core::geo::lat_lon_to_xyz(ll.lat(), ll.lng(), radius))
        .collect()
}

/// Target camera distance after one drill-in click: a fraction of the
/// current distance, floored at `RESOLUTION_NEAR_DISTANCE` so drilling
/// never pushes the camera through the surface.
pub fn drill_target_distance(current_distance: f32) -> f32 {
    (current_distance * DRILL_ZOOM_FACTOR).max(RESOLUTION_NEAR_DISTANCE)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_point_resolves_to_a_cell() {
        // San Francisco, roughly.
        let cell = latlon_to_cell(37.7749, -122.4194, Resolution::Two);
        assert!(cell.is_some());
    }

    #[test]
    fn resolution_coarsens_with_distance() {
        let far = resolution_for_distance(RESOLUTION_FAR_DISTANCE);
        let mid =
            resolution_for_distance((RESOLUTION_FAR_DISTANCE + RESOLUTION_NEAR_DISTANCE) / 2.0);
        let near = resolution_for_distance(RESOLUTION_NEAR_DISTANCE);
        assert!(u8::from(far) < u8::from(mid));
        assert!(u8::from(mid) < u8::from(near));
    }

    #[test]
    fn resolution_clamps_beyond_the_defined_range() {
        assert_eq!(
            resolution_for_distance(RESOLUTION_FAR_DISTANCE * 10.0),
            Resolution::try_from(RESOLUTION_MIN).unwrap()
        );
        assert_eq!(
            resolution_for_distance(0.0),
            Resolution::try_from(RESOLUTION_MAX).unwrap()
        );
    }

    #[test]
    fn invalid_coordinates_return_none() {
        assert!(latlon_to_cell(f64::NAN, 0.0, Resolution::Two).is_none());
    }

    #[test]
    fn boundary_has_five_or_six_vertices() {
        let cell = latlon_to_cell(37.7749, -122.4194, Resolution::Two).unwrap();
        let boundary = boundary_positions(cell, 1.0);
        // H3 cells are hexagons (6 vertices) except 12 pentagons per
        // resolution — 3 floats per vertex.
        assert!(boundary.len() == 5 * 3 || boundary.len() == 6 * 3);
    }

    #[test]
    fn boundary_vertices_lie_on_the_sphere() {
        let cell = latlon_to_cell(37.7749, -122.4194, Resolution::Two).unwrap();
        let boundary = boundary_positions(cell, 1.0);
        for chunk in boundary.chunks(3) {
            let len = (chunk[0] * chunk[0] + chunk[1] * chunk[1] + chunk[2] * chunk[2]).sqrt();
            assert!((len - 1.0).abs() < 1e-4, "vertex not on unit sphere: {len}");
        }
    }

    #[test]
    fn drill_never_crosses_the_near_floor() {
        let d = drill_target_distance(RESOLUTION_NEAR_DISTANCE * 1.1);
        assert!(d >= RESOLUTION_NEAR_DISTANCE);
        // Repeated drilling converges to the floor, never below it.
        let mut d = 8.0f32;
        for _ in 0..20 {
            d = drill_target_distance(d);
        }
        assert!((d - RESOLUTION_NEAR_DISTANCE).abs() < 0.01);
    }
}
