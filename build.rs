use std::{env, path::Path};

fn main() {
    let p = env::var("OPENAL_SOFT_PATH").expect("`OPENAL_SOFT_PATH` not found");
    let openal_soft_home = Path::new(p.as_str());
    println!(
        "cargo:rustc-link-search=native={}",
        link_search_path(openal_soft_home).to_str().expect("UTF-8")
    );
    println!("cargo:rustc-link-lib=OpenAL32");
}

fn link_search_path(openal_soft_home: &Path) -> std::path::PathBuf {
    let profile = env::var("PROFILE").unwrap();
    profile.find("debug").map_or_else(
        || {
            profile
                .find("release")
                .map(|_| openal_soft_home.join("build").join("Release"))
                .unwrap()
        },
        |_| openal_soft_home.join("build").join("Debug"),
    )
}
