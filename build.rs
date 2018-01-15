#[cfg(unix)]
fn main() {
    println!("cargo:rustc-link-search=native=/usr/local/opt/openal-soft/lib");
    println!("cargo:rustc-link-search=native=/usr/local/lib");
}

#[cfg(all(windows, target_arch = "x86", target_env="gnu"))]
fn main () {
    println!("cargo:rustc-link-search=native=C:\\msys32\\mingw64\\lib");
}

#[cfg(all(windows, target_arch = "x86_64", target_env="gnu"))]
fn main () {
    println!("cargo:rustc-link-search=native=C:\\msys64\\mingw64\\lib");
}

#[cfg(all(windows, target_env="msvc"))]
fn main () {
    //println!(r"cargo:rustc-link-search=native=C:\Users\hhati\ears\lib");
    println!(r"cargo:rustc-link-search=native=lib");
}


#[cfg(all(not(windows), not(unix)))]
fn main() {}
