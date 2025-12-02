use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let current_dir = env::current_dir().unwrap();
    println!("cargo:rustc-link-search=native={}", format!("{}/cpp_lib", current_dir.display()));
    println!("cargo:rustc-link-lib=static=cpp");

    let opencv_home = std::env::var("OpenCV_HOME").unwrap();
    println!("cargo:rustc-link-search=native={}", format!("{opencv_home}/lib"));
    println!("cargo:rustc-link-lib={}", "opencv_calib3d");
    println!("cargo:rustc-link-lib={}", "opencv_core");
    println!("cargo:rustc-link-lib={}", "opencv_dnn");
    println!("cargo:rustc-link-lib={}", "opencv_features2d");
    println!("cargo:rustc-link-lib={}", "opencv_flann");
    println!("cargo:rustc-link-lib={}", "opencv_freetype");
    println!("cargo:rustc-link-lib={}", "opencv_gapi");
    println!("cargo:rustc-link-lib={}", "opencv_highgui");
    println!("cargo:rustc-link-lib={}", "opencv_imgcodecs");
    println!("cargo:rustc-link-lib={}", "opencv_imgproc");
    println!("cargo:rustc-link-lib={}", "opencv_ml");
    println!("cargo:rustc-link-lib={}", "opencv_stitching");
    println!("cargo:rustc-link-lib={}", "opencv_photo");
    println!("cargo:rustc-link-lib={}", "opencv_video");
    println!("cargo:rustc-link-lib={}", "opencv_videoio");

    // 链接c++标准库, Rust 默认不链接 C++ 库    
    let os = env::consts::OS;
    match os {
        "linux" => {
            // 在Linux上执行的构建逻辑
            println!("cargo:rustc-link-lib=stdc++");
        }
        "macos" => {
            // 在macOS上执行的构建逻辑
            println!("cargo:rustc-link-lib=c++");
        }
        "windows" => {
            // 在Windows上执行的构建逻辑
            println!("当前操作系统是Windows");
        }
        _ => {
            println!("当前操作系统是未知的");
            // 未知操作系统
        }
    }

    println!("cargo:rustc-link-lib=stdc++");
    Ok(())
}