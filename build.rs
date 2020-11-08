use std::{env, path::Path};

fn main() {
    let openal_soft_home = env::var("OPENAL_SOFT_PATH").expect("`OPENAL_SOFT_PATH` not found");
    let openal_soft_home = Path::new(openal_soft_home.as_str());
    println!(
        "cargo:rustc-link-search=native={}",
        openal_link_search_path(openal_soft_home).to_str().expect("UTF-8")
    );
    println!("cargo:rustc-link-lib=OpenAL32");
}

#[inline]
fn openal_link_search_path(openal_soft_home: &Path) -> std::path::PathBuf {
    let profile = env::var("PROFILE").unwrap();
    openal_soft_home
        .join("build")
        .join(if let Some(_) = profile.find("debug") {
            "Debug"
        } else if let Some(_) = profile.find("release") {
            "Release"
        } else {
            unreachable!("`PROFILE` environment variable is {:?}", profile);
        })
}
