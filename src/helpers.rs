use std::sync::Once;

use crate::{interfaces::INTERFACES, sdk::PlayerEntity};

pub fn localplayer() -> Option<PlayerEntity> {
    let index = INTERFACES.engine.get_localplayer();
    let plocal = INTERFACES.entlist.get_client_entity(index);
    if !plocal.is_null() {
        static FOUND: Once = Once::new();
        FOUND.call_once(|| {
            println!("Local player @ {:X}", plocal as usize);
        });
        Some(PlayerEntity{ start: plocal })
    } else {
        None
    }
}