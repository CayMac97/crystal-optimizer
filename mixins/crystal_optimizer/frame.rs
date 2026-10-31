fn crystal_optimizer_frame(game: &mut crate::game::Game<'_>, elapsed: std::time::Duration) {
    crate::game::mod_crystal_optimizer_glue::on_frame(game, elapsed);
}
