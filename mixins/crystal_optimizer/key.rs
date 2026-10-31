fn crystal_optimizer_key(game: &mut crate::game::Game<'_>, key: &str, pressed: bool) {
    crate::game::mod_crystal_optimizer_glue::on_key(game, key, pressed);
}
