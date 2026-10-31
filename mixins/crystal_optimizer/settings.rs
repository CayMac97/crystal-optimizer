fn crystal_optimizer_settings(
    game: &mut crate::game::Game<'_>,
    mod_id: &str,
    buttons: &mut Vec<String>,
) {
    crate::game::mod_crystal_optimizer_glue::on_settings(game, mod_id, buttons);
}
