//build.rs

//fn main() {
//    cc::Build::new()
//        .file("src/led.c")
//        .compile("led");   // 输出 `libled.a`
//}



use std::process::Command;
use std::env;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    // note that there are a number of downsides to this approach, the comments
    // below detail how to improve the portability of these commands.
    Command::new("gcc").args(&["src/led.c", "-c", "-fPIC", "-o"])
                       .arg(&format!("{}/led.o", out_dir))
                       .status().unwrap();

    Command::new("ar").args(&["crus", "libled.a", "led.o"])
                      .current_dir(&Path::new(&out_dir))
                      .status().unwrap();

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=led");
    println!("cargo:rustc-link-lib=dylib=wiringPi");
}
