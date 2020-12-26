extern crate version_check;

fn main() {
    let version = version_check::Version::read().expect("Rustc to give us its version");

    for i in 0..65536 {
        if version.at_least(&format!("1.{}.0", i)) {
            println!("cargo:rustc-cfg=rustc_1_{}_0", i)
        } else {
            break;
        }
    }
}
