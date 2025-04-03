fn main() {
    windows_app_manifest();
    
    // 添加comctl32链接库，版本6.0及以上
    println!("cargo:rustc-link-lib=dylib=comctl32");
    
    // 设置COMCTL32_ISOLATION=isolation_aware_enabled，确保使用正确的Common Controls版本
    println!("cargo:rustc-env=COMCTL32_ISOLATION=isolation_aware_enabled");
}

fn windows_app_manifest() {
    let mut windows = tauri_build::WindowsAttributes::new();
    let manifest = include_str!("manifest.xml"); 

    windows = windows.app_manifest(manifest);

    let attrs = tauri_build::Attributes::new().windows_attributes(windows);
    tauri_build::try_build(attrs).expect("failed to run build script");
}