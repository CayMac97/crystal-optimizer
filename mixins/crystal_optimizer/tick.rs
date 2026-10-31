fn crystal_optimizer_tick(game: &mut crate::game::Game<'_>) {
    crate::game::mod_crystal_optimizer_glue::on_tick(game);
}
