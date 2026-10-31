fn crystal_optimizer_click(
    game: &mut crate::game::Game<'_>,
    mod_id: &str,
    button: usize,
) {
    crate::game::mod_crystal_optimizer_glue::on_click(game, mod_id, button);
}
