// Glue between Horizon's `Game` and `mod_crystal_optimizer`
use crate::game::Game;
use crate::game::mod_crystal_optimizer as m;
use crate::game::native_hooks::NativeMod;
use std::time::Duration;

fn is_crystal_or_obsidian(item: &str) -> bool {
    item.ends_with("end_crystal") || item.ends_with("obsidian")
}

pub(super) fn try_break_targeted_crystal(game: &mut Game<'_>) -> bool {
    if let Some(hit) = game.remote_object_hit(true) {
        if game.mob_hit().is_none_or(|(_, d)| hit.distance < d)
            && game.before_block(hit.distance)
            && game.crystals.iter().any(|c| c.id as i32 == hit.id)
        {
            // End Crystal targeted!
            // 1. Send normal attack packet to server
            game.send(horizon_protocol::packets::play::Serverbound::Attack(
                horizon_protocol::packets::play::Attack { id: hit.id },
            ));
            // 2. Reset attack strength
            game.reset_attack_strength();
            // 3. Send punch packet and swing arm animation
            game.swing_arm();
            game.send(horizon_protocol::packets::play::Serverbound::Punch(
                horizon_protocol::packets::Empty,
            ));
            // 4. Client-side instant predict break:
            if m::predict_break_enabled() {
                game.crystals.retain(|c| c.id as i32 != hit.id);
                game.remove_remote_dragon(hit.id);
            }
            // 5. Zero miss delay:
            game.clicks.miss_time = 0;
            return true;
        }
    }
    false
}

pub(super) fn on_frame(game: &mut Game<'_>, _elapsed: Duration) {
    if !game.native_on(m::ID) || !m::is_enabled() {
        return;
    }

    // Zero miss delay penalty
    if game.clicks.miss_time > 0 {
        game.clicks.miss_time = 0;
    }

    // Fast place obsidian and crystals
    if m::fast_place_enabled() {
        let main_match = game.held().is_some_and(|s| is_crystal_or_obsidian(&s.item));
        let off_match = game.offhand.as_ref().is_some_and(|s| is_crystal_or_obsidian(&s.item));
        if main_match || off_match {
            game.use_delay = 0;
            game.clicks.right_click_delay = 0;
        }
    }

    // Auto break if holding attack key down (or if crosshair moves over crystal)
    if game.left && m::auto_break_enabled() {
        try_break_targeted_crystal(game);
    }
}

pub(super) fn on_tick(game: &mut Game<'_>) {
    if !game.native_on(m::ID) || !m::is_enabled() {
        return;
    }

    if game.clicks.miss_time > 0 {
        game.clicks.miss_time = 0;
    }

    if m::fast_place_enabled() {
        let main_match = game.held().is_some_and(|s| is_crystal_or_obsidian(&s.item));
        let off_match = game.offhand.as_ref().is_some_and(|s| is_crystal_or_obsidian(&s.item));
        if main_match || off_match {
            game.use_delay = 0;
            game.clicks.right_click_delay = 0;
        }
    }
}

pub(super) fn on_key(game: &mut Game<'_>, key: &str, pressed: bool) {
    if !pressed || !game.native_on(m::ID) {
        return;
    }

    let toggle_key = game.native_mod_key(m::ID, m::KEY_TOGGLE, m::KEY_TOGGLE_DEFAULT);
    if key == toggle_key && toggle_key != "key.keyboard.unknown" {
        m::toggle_enabled();
        return;
    }

    // If attack key pressed, check crystal attack
    if m::is_enabled() && (key == "key.mouse.left" || game.settings.keys.get("key.attack") == key) {
        try_break_targeted_crystal(game);
    }
}

pub(super) fn on_list(mods: &mut Vec<NativeMod>) {
    mods.push(NativeMod {
        id: m::ID.into(),
        title: format!("{} {}", m::NAME, m::VERSION),
        description: m::DESCRIPTION.into(),
        keys: vec![[
            m::KEY_TOGGLE.into(),
            m::KEY_TOGGLE_LABEL.into(),
            m::KEY_TOGGLE_DEFAULT.into(),
        ]],
    });
}

pub(super) fn on_settings(_game: &mut Game<'_>, mod_id: &str, buttons: &mut Vec<String>) {
    if mod_id != m::ID {
        return;
    }
    let enabled_str = if m::is_enabled() { "ON" } else { "OFF" };
    let predict_str = if m::predict_break_enabled() { "ON" } else { "OFF" };
    let auto_break_str = if m::auto_break_enabled() { "ON" } else { "OFF" };
    let fast_place_str = if m::fast_place_enabled() { "ON" } else { "OFF" };

    buttons.push(format!("Mod Status: {enabled_str}"));
    buttons.push(format!("Predict Break (0ms): {predict_str}"));
    buttons.push(format!("Auto-Break (Hold): {auto_break_str}"));
    buttons.push(format!("Fast Place (Obsidian/Crystal): {fast_place_str}"));
}

pub(super) fn on_click(_game: &mut Game<'_>, mod_id: &str, button: usize) {
    if mod_id != m::ID {
        return;
    }
    match button {
        0 => {
            m::toggle_enabled();
        }
        1 => {
            m::toggle_predict_break();
        }
        2 => {
            m::toggle_auto_break();
        }
        3 => {
            m::toggle_fast_place();
        }
        _ => {}
    }
}
