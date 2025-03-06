pub fn get_gfx_path(filename: &str) -> String {
    let exe_path = std::env::current_exe().unwrap();
    let exe_dir = exe_path.parent().unwrap();
    let asset_path = exe_dir.join("gfx").join(filename);
    asset_path.to_string_lossy().to_string()
}