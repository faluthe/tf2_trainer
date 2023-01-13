use std::{ffi::{c_void, c_int, c_char, c_ulong, CString, c_float}, ops::Add, mem};
use windows::{core::PCWSTR, w, Win32::System::LibraryLoader};

use crate::{macros::{vfunc, netvar}, interfaces::INTERFACES, netvars, scanner};

// Interfaces
pub struct Engine { pub start: *mut c_void }
unsafe impl Send for Engine {}
unsafe impl Sync for Engine {}
pub struct EntityList { pub start: *mut c_void }
unsafe impl Send for EntityList {}
unsafe impl Sync for EntityList {}
pub struct BaseClient { pub start: *mut c_void }
unsafe impl Send for BaseClient {}
unsafe impl Sync for BaseClient {}
pub struct Surface { pub start: *mut c_void }
unsafe impl Send for Surface {}
unsafe impl Sync for Surface {}
pub struct DebugOverlay { pub start: *mut c_void }
unsafe impl Send for DebugOverlay {}
unsafe impl Sync for DebugOverlay {}

// Classes
pub struct PlayerEntity { pub start: *mut c_void }
pub struct WeaponEntity { pub start: *mut c_void }

// Interface impls
impl Engine {
    pub fn get_localplayer(&self) -> i32 {
        let func = vfunc!(self.start, i32, 12);
        func(self.start)
    }
    pub fn get_max_clients(&self) -> i32 {
        let func = vfunc!(self.start, i32, 21);
        func(self.start)
    }
}

impl EntityList {
    pub fn get_client_entity(&self, index: i32) -> *mut c_void {
        let func = vfunc!(self.start, *mut c_void, 3, i32);
        func(self.start, index)
    }
}

impl BaseClient {
    pub fn get_all_classes(&self) -> *mut ClientClass {
        let func = vfunc!(self.start, *mut ClientClass, 8);
        func(self.start)
    }
}

impl Surface {
    pub fn text_create_font(&self) -> c_ulong {
        let func = vfunc!(self.start, c_ulong, 66);
        func(self.start)
    }
    pub fn set_font_glyph_set(&self, font: c_ulong, font_name: &str, size: c_int, weight: c_int, flags: c_int) -> bool {
        let func = vfunc!(self.start, bool, 67, c_ulong, *const c_char, c_int, c_int, c_int, c_int, c_int, c_int, c_int);
        let font_name = CString::new(font_name).unwrap();
        func(self.start, font, font_name.as_ptr(), size, weight, 0, 0, flags, 0, 0)
    }
    pub fn draw_set_color(&self, r: c_int, g: c_int, b: c_int, a: c_int) {
        let func = vfunc!(self.start, (), 11, c_int, c_int, c_int, c_int);
        func(self.start, r, g, b, a)
    }
    pub fn draw_filled_rect(&self, x0: c_int, y0: c_int, x1: c_int, y1: c_int) {
        let func = vfunc!(self.start, (), 12, c_int, c_int, c_int, c_int);
        func(self.start, x0, y0, x1, y1)
    }
    pub fn _draw_outlined_rect(&self, x0: c_int, y0: c_int, x1: c_int, y1: c_int) {
        let func = vfunc!(self.start, (), 14, c_int, c_int, c_int, c_int);
        func(self.start, x0, y0, x1, y1)
    }
    pub fn draw_set_text_font(&self, font: c_ulong) {
        let func = vfunc!(self.start, (), 17, c_ulong);
        func(self.start, font)
    }
    pub fn draw_set_text_color(&self, r: c_int, g: c_int, b: c_int, a: c_int) {
        let func = vfunc!(self.start, (), 19, c_int, c_int, c_int, c_int);
        func(self.start, r, g, b, a)
    }
    pub fn draw_set_text_pos(&self, x: c_int, y: c_int) {
        let func = vfunc!(self.start, (), 20, c_int, c_int);
        func(self.start, x, y)
    }
    pub fn draw_print_text(&self, text: PCWSTR, len: c_int) {
        let func = vfunc!(self.start, (), 22, PCWSTR, c_int, c_int);
        func(self.start, text, len, 0)
    }
}

impl DebugOverlay {
    pub fn screen_position(&self, point: &Vec3, screen: &mut Vec3) -> c_int {
        let func = vfunc!(self.start, c_int, 11, *const Vec3, *mut Vec3);
        func(self.start, point as *const Vec3, screen as *mut Vec3)
    }
}

// Class impls
impl PlayerEntity {
    pub fn get_renderable(&self) -> *mut c_void {
        (self.start as usize + 0x4) as *mut c_void
    }
    pub fn get_networkable(&self) -> *mut c_void {
        (self.start as usize + 0x8) as *mut c_void
    }
    pub unsafe fn health(&self) -> i32 {
        netvar!(self.start, "DT_BasePlayer", "m_iHealth", i32)
    }
    pub unsafe fn max_health(&self) -> i32 {
        vfunc!(self.start, i32, 107)(self.start)
    }
    pub unsafe fn lifestate(&self) -> u8 {
        netvar!(self.start, "DT_BasePlayer", "m_lifeState", u8)
    }
    pub unsafe fn is_alive(&self) -> bool {
        self.lifestate() == 0
    }
    pub unsafe fn flags(&self) -> i32 {
        netvar!(self.start, "DT_BasePlayer", "m_fFlags", i32)
    }
    pub unsafe fn team(&self) -> i32 {
        netvar!(self.start, "DT_BaseEntity", "m_iTeamNum", i32)
    }
    pub unsafe fn origin(&self) -> Vec3 {
        netvar!(self.start, "DT_BasePlayer", "m_vecOrigin", Vec3)
    }
    pub unsafe fn active_weapon(&self) -> Option<WeaponEntity> {
        let weapon = netvar!(self.start, "DT_BaseCombatCharacter", "m_hActiveWeapon", usize);
        let start = INTERFACES.entlist.get_client_entity(weapon as i32 & 0xFFF);
        if !start.is_null() {
            Some(WeaponEntity { start })
        } else {
            None
        }
    }
    pub unsafe fn get_shared(&self) -> *mut c_void {
        static mut OFFSET: usize = 0;
        if OFFSET == 0 {
            OFFSET = netvars::get("DT_TFPlayer", "m_Shared").unwrap();
            println!("{} @ {:#X}", "m_Shared", OFFSET);
        }
        ((self.start as usize) + OFFSET) as *mut c_void
    }
    pub unsafe fn in_cond(&self, cond: Conditions) -> bool {
        static mut ADDR: usize = 0;
        if ADDR == 0 {
            let client_mod = LibraryLoader::GetModuleHandleW(w!("client.dll")).unwrap();
            ADDR = scanner::find_sig(client_mod, "55 8B EC 83 EC 08 56 57 8B 7D 08 8B F1 83 FF 20").unwrap() as usize;
            println!("Address of the function: {:x}", ADDR);
        }

        let func: extern "thiscall" fn(*mut c_void, i32) -> bool = unsafe { mem::transmute(ADDR as *mut c_void) };
        func(self.get_shared(), cond as i32)
    }
    pub unsafe fn is_overhealed(&self) -> bool {
        self.in_cond(Conditions::TF_COND_HEALTH_OVERHEALED)
    }
    pub unsafe fn get_collideable(&self) -> *mut c_void {
        static mut OFFSET: usize = 0;
        if OFFSET == 0 {
            OFFSET = netvars::get("DT_BaseEntity", "m_Collision").unwrap();
            println!("{} @ {:#X}", "m_Collision", OFFSET);
        }
        ((self.start as usize) + OFFSET) as *mut c_void
    }
    pub unsafe fn get_collideable_mins(&self) -> *const Vec3 {
        vfunc!(self.get_collideable(), *const Vec3, 1)(self.get_collideable())
    }
    pub unsafe fn get_collideable_maxs(&self) -> *const Vec3 {
        vfunc!(self.get_collideable(), *const Vec3, 2)(self.get_collideable())
    }
    pub unsafe fn entity_to_world_transform(&self) -> *mut [[f32; 4]; 3] {
        let func = vfunc!(self.get_renderable(), *mut [[f32; 4]; 3], 34);
        func(self.get_renderable())
    }
    pub unsafe fn is_dormant(&self) -> bool {
        let func = vfunc!(self.get_networkable(), bool, 8);
        func(self.get_networkable())
    }
	    // pub unsafe fn get_render_bounds(&self, mins: &mut Vec3, maxs: &mut Vec3) {
    //     let func = vfunc!(self.get_renderable(), (), 20, *mut Vec3, *mut Vec3);
    //     func(self.get_renderable(), mins as *mut Vec3, maxs as *mut Vec3)
    // }
}

impl WeaponEntity {
    pub unsafe fn can_backstab(&self) -> bool {
        netvar!(self.start, "DT_TFWeaponKnife", "m_bReadyToBackstab", bool)
    }
    pub unsafe fn _item_def_index(&self) -> i32 {
        netvar!(self.start, "DT_BaseCombatWeapon", "m_iItemDefinitionIndex", i32)
    }
    pub unsafe fn weapon_id(&self) -> i32 {
        let func = vfunc!(self.start, i32, 380);
        func(self.start)
    }
    pub unsafe fn is_knife(&self) -> bool {
        self.weapon_id() == Weapons::TF_WEAPON_KNIFE as i32
    }
}

// Structs
struct QAngle{
    _pitch: f32,
    _yaw: f32,
    _roll: f32,
}

pub struct CUserCmd {
    _vtable: *mut c_void,
    _command_number: i32,
    _tick_count: i32,
    _viewangles: QAngle,
    _forwardmove: f32,
    _sidemove: f32,
    _upmove: f32,
    pub buttons: i32,
}

pub struct ClientClass {
    _pad: [u8; 8],
    _network_name: *const c_char,
    pub recv_table: *mut RecvTable,
    pub next: *mut ClientClass,
    _id: c_int,
}

pub struct RecvTable {
    pub props: *mut RecvProp,
    pub nprops: c_int,
    _decoder: *mut c_void,
    pub net_table_name: *mut c_char,
}

// Needs to be of size 3C!! netvars::check_table uses .offset()
pub struct RecvProp {
    pub var_name: *mut c_char,
    _pad0: [u8; 4],
    _flags: c_int,
    _str_buf_size: c_int,
    _b_inside_array: c_int,
    _extra: *const c_void,
    _array_prop: *mut RecvProp,
    _pad1: [u8; 12],
    pub data_table: *mut RecvTable,
    pub offset: c_int,
    _elementstride: c_int,
    _n_elements: c_int,
    _parent_array_prop_name: *const c_char,
}

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: c_float,
    pub y: c_float,
    pub z: c_float,
}

impl Vec3 {
    pub fn default() -> Vec3 {
        Vec3 { x: 0.0, y: 0.0, z: 0.0 }
    }
    pub fn new(x: c_float, y: c_float, z: c_float) -> Vec3 {
        Vec3{ x, y, z }
    }
    pub fn dot(&self, other_vec: &[f32; 4]) -> f32 {
        self.x * other_vec[0] + self.y * other_vec[1] + self.z * other_vec[2]
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

pub fn vector_transform(the_vec: &Vec3, the_matrix: &[[f32; 4]; 3], out: &mut Vec3) {
    out.x = the_vec.dot(&the_matrix[0]) + the_matrix[0][3];
    out.y = the_vec.dot(&the_matrix[1]) + the_matrix[1][3];
    out.z = the_vec.dot(&the_matrix[2]) + the_matrix[2][3];
}

#[allow(non_camel_case_types, dead_code)]
enum Weapons {
    TF_WEAPON_NONE = 0,
	TF_WEAPON_BAT,
	TF_WEAPON_BAT_WOOD,
	TF_WEAPON_BOTTLE, 
	TF_WEAPON_FIREAXE,
	TF_WEAPON_CLUB,
	TF_WEAPON_CROWBAR,
	TF_WEAPON_KNIFE,
	TF_WEAPON_FISTS,
	TF_WEAPON_SHOVEL,
	TF_WEAPON_WRENCH,
	TF_WEAPON_BONESAW,
	TF_WEAPON_SHOTGUN_PRIMARY,
	TF_WEAPON_SHOTGUN_SOLDIER,
	TF_WEAPON_SHOTGUN_HWG,
	TF_WEAPON_SHOTGUN_PYRO,
	TF_WEAPON_SCATTERGUN,
	TF_WEAPON_SNIPERRIFLE,
	TF_WEAPON_MINIGUN,
	TF_WEAPON_SMG,
	TF_WEAPON_SYRINGEGUN_MEDIC,
	TF_WEAPON_TRANQ,
	TF_WEAPON_ROCKETLAUNCHER,
	TF_WEAPON_GRENADELAUNCHER,
	TF_WEAPON_PIPEBOMBLAUNCHER,
	TF_WEAPON_FLAMETHROWER,
	TF_WEAPON_GRENADE_NORMAL,
	TF_WEAPON_GRENADE_CONCUSSION,
	TF_WEAPON_GRENADE_NAIL,
	TF_WEAPON_GRENADE_MIRV,
	TF_WEAPON_GRENADE_MIRV_DEMOMAN,
	TF_WEAPON_GRENADE_NAPALM,
	TF_WEAPON_GRENADE_GAS,
	TF_WEAPON_GRENADE_EMP,
	TF_WEAPON_GRENADE_CALTROP,
	TF_WEAPON_GRENADE_PIPEBOMB,
	TF_WEAPON_GRENADE_SMOKE_BOMB,
	TF_WEAPON_GRENADE_HEAL,
	TF_WEAPON_GRENADE_STUNBALL,
	TF_WEAPON_GRENADE_JAR,
	TF_WEAPON_GRENADE_JAR_MILK,
	TF_WEAPON_PISTOL,
	TF_WEAPON_PISTOL_SCOUT,
	TF_WEAPON_REVOLVER,
	TF_WEAPON_NAILGUN,
	TF_WEAPON_PDA,
	TF_WEAPON_PDA_ENGINEER_BUILD,
	TF_WEAPON_PDA_ENGINEER_DESTROY,
	TF_WEAPON_PDA_SPY,
	TF_WEAPON_BUILDER,
	TF_WEAPON_MEDIGUN,
	TF_WEAPON_GRENADE_MIRVBOMB,
	TF_WEAPON_FLAMETHROWER_ROCKET,
	TF_WEAPON_GRENADE_DEMOMAN,
	TF_WEAPON_SENTRY_BULLET,
	TF_WEAPON_SENTRY_ROCKET,
	TF_WEAPON_DISPENSER,
	TF_WEAPON_INVIS,
	TF_WEAPON_FLAREGUN,
	TF_WEAPON_LUNCHBOX,
	TF_WEAPON_JAR,
	TF_WEAPON_COMPOUND_BOW,
	TF_WEAPON_BUFF_ITEM,
	TF_WEAPON_PUMPKIN_BOMB,
	TF_WEAPON_SWORD, 
	TF_WEAPON_ROCKETLAUNCHER_DIRECTHIT,
	TF_WEAPON_LIFELINE,
	TF_WEAPON_LASER_POINTER,
	TF_WEAPON_DISPENSER_GUN,
	TF_WEAPON_SENTRY_REVENGE,
	TF_WEAPON_JAR_MILK,
	TF_WEAPON_HANDGUN_SCOUT_PRIMARY,
	TF_WEAPON_BAT_FISH,
	TF_WEAPON_CROSSBOW,
	TF_WEAPON_STICKBOMB,
	TF_WEAPON_HANDGUN_SCOUT_SECONDARY,
	TF_WEAPON_SODA_POPPER,
	TF_WEAPON_SNIPERRIFLE_DECAP,
	TF_WEAPON_RAYGUN,
	TF_WEAPON_PARTICLE_CANNON,
	TF_WEAPON_MECHANICAL_ARM,
	TF_WEAPON_DRG_POMSON,
	TF_WEAPON_BAT_GIFTWRAP,
	TF_WEAPON_GRENADE_ORNAMENT_BALL,
	TF_WEAPON_FLAREGUN_REVENGE,
	TF_WEAPON_PEP_BRAWLER_BLASTER,
	TF_WEAPON_CLEAVER,
	TF_WEAPON_GRENADE_CLEAVER,
	TF_WEAPON_STICKY_BALL_LAUNCHER,
	TF_WEAPON_GRENADE_STICKY_BALL,
	TF_WEAPON_SHOTGUN_BUILDING_RESCUE,
	TF_WEAPON_CANNON,
	TF_WEAPON_THROWABLE,
	TF_WEAPON_GRENADE_THROWABLE,
	TF_WEAPON_PDA_SPY_BUILD,
	TF_WEAPON_GRENADE_WATERBALLOON,
	TF_WEAPON_HARVESTER_SAW,
	TF_WEAPON_SPELLBOOK,
	TF_WEAPON_SPELLBOOK_PROJECTILE,
	TF_WEAPON_SNIPERRIFLE_CLASSIC,
	TF_WEAPON_PARACHUTE,
	TF_WEAPON_GRAPPLINGHOOK,
	TF_WEAPON_PASSTIME_GUN,
}

#[allow(non_camel_case_types, dead_code)]
pub enum Conditions {
    TF_COND_INVALID = -1,
	TF_COND_AIMING = 0,		// Sniper aiming, Heavy minigun.
	TF_COND_ZOOMED,
	TF_COND_DISGUISING,
	TF_COND_DISGUISED,
	TF_COND_STEALTHED,		// Spy specific
	TF_COND_INVULNERABLE,
	TF_COND_TELEPORTED,
	TF_COND_TAUNTING,
	TF_COND_INVULNERABLE_WEARINGOFF,
	TF_COND_STEALTHED_BLINK,
	TF_COND_SELECTED_TO_TELEPORT,
	TF_COND_CRITBOOSTED,	// DO NOT RE-USE THIS -- THIS IS FOR KRITZKRIEG AND REVENGE CRITS ONLY
	TF_COND_TMPDAMAGEBONUS,
	TF_COND_FEIGN_DEATH,
	TF_COND_PHASE,
	TF_COND_STUNNED,		// Any type of stun. Check iStunFlags for more info.
	TF_COND_OFFENSEBUFF,
	TF_COND_SHIELD_CHARGE,
	TF_COND_DEMO_BUFF,
	TF_COND_ENERGY_BUFF,
	TF_COND_RADIUSHEAL,
	TF_COND_HEALTH_BUFF,
	TF_COND_BURNING,
	TF_COND_HEALTH_OVERHEALED,
	TF_COND_URINE,
	TF_COND_BLEEDING,
	TF_COND_DEFENSEBUFF,	// 35% defense! No crit damage.
	TF_COND_MAD_MILK,
	TF_COND_MEGAHEAL,
	TF_COND_REGENONDAMAGEBUFF,
	TF_COND_MARKEDFORDEATH,
	TF_COND_NOHEALINGDAMAGEBUFF,
	TF_COND_SPEED_BOOST,				// = 32
	TF_COND_CRITBOOSTED_PUMPKIN,		// Brandon hates bits
	TF_COND_CRITBOOSTED_USER_BUFF,
	TF_COND_CRITBOOSTED_DEMO_CHARGE,
	TF_COND_SODAPOPPER_HYPE,
	TF_COND_CRITBOOSTED_FIRST_BLOOD,	// arena mode first blood
	TF_COND_CRITBOOSTED_BONUS_TIME,
	TF_COND_CRITBOOSTED_CTF_CAPTURE,
	TF_COND_CRITBOOSTED_ON_KILL,		// =40. KGB, etc.
	TF_COND_CANNOT_SWITCH_FROM_MELEE,
	TF_COND_DEFENSEBUFF_NO_CRIT_BLOCK,	// 35% defense! Still damaged by crits.
	TF_COND_REPROGRAMMED,				// Bots only
	TF_COND_CRITBOOSTED_RAGE_BUFF,
	TF_COND_DEFENSEBUFF_HIGH,			// 75% defense! Still damaged by crits.
	TF_COND_SNIPERCHARGE_RAGE_BUFF,		// Sniper Rage - Charge time speed up
	TF_COND_DISGUISE_WEARINGOFF,		// Applied for half-second post-disguise
	TF_COND_MARKEDFORDEATH_SILENT,		// Sans sound
	TF_COND_DISGUISED_AS_DISPENSER,
	TF_COND_SAPPED,						// =50. Bots only
	TF_COND_INVULNERABLE_HIDE_UNLESS_DAMAGED,
	TF_COND_INVULNERABLE_USER_BUFF,
	TF_COND_HALLOWEEN_BOMB_HEAD,
	TF_COND_HALLOWEEN_THRILLER,
	TF_COND_RADIUSHEAL_ON_DAMAGE,
	TF_COND_CRITBOOSTED_CARD_EFFECT,
	TF_COND_INVULNERABLE_CARD_EFFECT,
	TF_COND_MEDIGUN_UBER_BULLET_RESIST,
	TF_COND_MEDIGUN_UBER_BLAST_RESIST,
	TF_COND_MEDIGUN_UBER_FIRE_RESIST,		// =60
	TF_COND_MEDIGUN_SMALL_BULLET_RESIST,
	TF_COND_MEDIGUN_SMALL_BLAST_RESIST,
	TF_COND_MEDIGUN_SMALL_FIRE_RESIST,
	TF_COND_STEALTHED_USER_BUFF,			// Any class can have this
	TF_COND_MEDIGUN_DEBUFF,
	TF_COND_STEALTHED_USER_BUFF_FADING,
	TF_COND_BULLET_IMMUNE,
	TF_COND_BLAST_IMMUNE,
	TF_COND_FIRE_IMMUNE,
	TF_COND_PREVENT_DEATH,					// =70
	TF_COND_MVM_BOT_STUN_RADIOWAVE, 		// Bots only
	TF_COND_HALLOWEEN_SPEED_BOOST,
	TF_COND_HALLOWEEN_QUICK_HEAL,
	TF_COND_HALLOWEEN_GIANT,
	TF_COND_HALLOWEEN_TINY,
	TF_COND_HALLOWEEN_IN_HELL,
	TF_COND_HALLOWEEN_GHOST_MODE,			// =77
	TF_COND_MINICRITBOOSTED_ON_KILL,
	TF_COND_OBSCURED_SMOKE,
	TF_COND_PARACHUTE_DEPLOYED,				// =80
	TF_COND_BLASTJUMPING,
	TF_COND_HALLOWEEN_KART,
	TF_COND_HALLOWEEN_KART_DASH,
	TF_COND_BALLOON_HEAD,					// =84 larger head, lower-gravity-feeling jumps
	TF_COND_MELEE_ONLY,						// =85 melee only
	TF_COND_SWIMMING_CURSE,					// player movement become swimming movement
	TF_COND_FREEZE_INPUT,					// freezes player input
	TF_COND_HALLOWEEN_KART_CAGE,			// attach cage model to player while in kart
	TF_COND_DONOTUSE_0,
	TF_COND_RUNE_STRENGTH,
	TF_COND_RUNE_HASTE,
	TF_COND_RUNE_REGEN,
	TF_COND_RUNE_RESIST,
	TF_COND_RUNE_VAMPIRE,
	TF_COND_RUNE_REFLECT,
	TF_COND_RUNE_PRECISION,
	TF_COND_RUNE_AGILITY,
	TF_COND_GRAPPLINGHOOK,
	TF_COND_GRAPPLINGHOOK_SAFEFALL,
	TF_COND_GRAPPLINGHOOK_LATCHED,
	TF_COND_GRAPPLINGHOOK_BLEEDING,
	TF_COND_AFTERBURN_IMMUNE,
	TF_COND_RUNE_KNOCKOUT,
	TF_COND_RUNE_IMBALANCE,
	TF_COND_CRITBOOSTED_RUNE_TEMP,
	TF_COND_PASSTIME_INTERCEPTION,
	TF_COND_SWIMMING_NO_EFFECTS,			// =107_DNOC_FT
	TF_COND_PURGATORY,
	TF_COND_RUNE_KING,
	TF_COND_RUNE_PLAGUE,
	TF_COND_RUNE_SUPERNOVA,
	TF_COND_PLAGUE,
	TF_COND_KING_BUFFED,
	TF_COND_TEAM_GLOWS,						// used to show team glows to living players
	TF_COND_KNOCKED_INTO_AIR,
	TF_COND_COMPETITIVE_WINNER,
	TF_COND_COMPETITIVE_LOSER,
	TF_COND_HEALING_DEBUFF,
	TF_COND_PASSTIME_PENALTY_DEBUFF,		// when carrying the ball without any teammates nearby	
	TF_COND_GRAPPLED_TO_PLAYER,
	TF_COND_GRAPPLED_BY_PLAYER,
}