fn crystal_optimizer_list(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    crate::game::mod_crystal_optimizer_glue::on_list(mods);
}
