use std::ffi::c_ulong;

use windows::{w, core::{PCWSTR, HSTRING}};

use crate::{interfaces::INTERFACES, sdk::{PlayerEntity, Vec3, vector_transform}};

pub unsafe fn player_esp(font: c_ulong) {
    let index = INTERFACES.engine.get_localplayer();
    let p_local = INTERFACES.entlist.get_client_entity(index);

    if p_local.is_null() {
        return;
    }

    let localplayer = PlayerEntity { start: p_local };

    let nplayers = INTERFACES.engine.get_max_clients();
    for i in 1..nplayers {
        let p_ent = INTERFACES.entlist.get_client_entity(i);

        if p_ent.is_null() {
            continue;
        }

        let ent = PlayerEntity { start: p_ent };

        if p_ent == p_local || ent.is_dormant() || !ent.is_alive() || ent.team() == localplayer.team() {
            continue;
        }

        let mins = *(ent.get_collideable_mins());
        let maxs = *(ent.get_collideable_maxs());

        let points = [ mins.clone(),
            Vec3::new(mins.x, maxs.y, mins.z),
            Vec3::new(maxs.x, maxs.y, mins.z),
            Vec3::new(maxs.x, mins.y, mins.z),
            maxs.clone(),
            Vec3::new(mins.x, maxs.y, maxs.z),
            Vec3::new(mins.x, mins.y, maxs.z),
            Vec3::new(maxs.x, mins.y, maxs.z) ];
        let mut points_transformed = [Vec3::default(); 8];
        
        for i in 0..8 {
            vector_transform(&points[i], ent.entity_to_world_transform().as_ref().unwrap(), &mut points_transformed[i]);
        }

        let origin = ent.origin();
        // let mut origin_trans = Vec3::default();
        // vector_transform(&origin, ent.entity_to_world_transform().as_ref().unwrap(), &mut origin_trans);
        let mut org = Vec3::default();
        let mut flb = Vec3::default();
        let mut brt = Vec3::default();
        let mut blb = Vec3::default();
        let mut frt = Vec3::default();
        let mut frb = Vec3::default();
        let mut brb = Vec3::default();
        let mut blt = Vec3::default();
        let mut flt = Vec3::default();
      
        if INTERFACES.debug_overlay.screen_position(&origin, &mut org) != 0
        || INTERFACES.debug_overlay.screen_position(&points_transformed[3], &mut flb) != 0
        || INTERFACES.debug_overlay.screen_position(&points_transformed[5], &mut brt) != 0
        || INTERFACES.debug_overlay.screen_position(&points_transformed[0], &mut blb) != 0
        || INTERFACES.debug_overlay.screen_position(&points_transformed[4], &mut frt) != 0
        || INTERFACES.debug_overlay.screen_position(&points_transformed[2], &mut frb) != 0
        || INTERFACES.debug_overlay.screen_position(&points_transformed[1], &mut brb) != 0
        || INTERFACES.debug_overlay.screen_position(&points_transformed[6], &mut blt) != 0
        || INTERFACES.debug_overlay.screen_position(&points_transformed[7], &mut flt) != 0 {
            continue;
        }

        let arr = [ flb, brt, blb, frt, frb, brb, blt, flt ];

        let mut left = flb.x;
        let mut top = flb.y;
        let mut right = flb.x;
        let mut bottom = flb.y;

        for i in 1..8 {
            if left > arr[i].x {
                left = arr[i].x;
            }
            if top < arr[i].y {
                top = arr[i].y;
            }
            if right < arr[i].x {
                right = arr[i].x;
            }
            if bottom > arr[i].y {
                bottom = arr[i].y;
            }
        }
        
        let h = if ent.max_health() < ent.health() { top - bottom } else { (top - bottom) * (ent.health() as f32 / ent.max_health() as f32)};
        INTERFACES.surface.draw_set_color(0, 0, 0, 255);
        INTERFACES.surface.draw_filled_rect(left as i32 - 2, top as i32 + 2, left as i32 + 3, bottom as i32 - 2);
        
        if ent.is_overhealed() {
            INTERFACES.surface.draw_set_color(0, 0, 255, 255);
        } else {
            INTERFACES.surface.draw_set_color(0, 255, 0, 255);
        }
        INTERFACES.surface.draw_filled_rect(left as i32, top as i32, left as i32 + 1, (top - h) as i32);

        INTERFACES.surface.draw_set_text_color(0, 0, 255, 255);
        INTERFACES.surface.draw_set_text_pos(right as i32, bottom as i32);
        
        if ent.is_overhealed() {
            INTERFACES.surface.draw_print_text(w!("Overhealed"), 10);
        }
    }
}