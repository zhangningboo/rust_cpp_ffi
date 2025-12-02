use std::env;
use glob::glob;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ----------------------------------------------------------------------
    // 1. 获取环境变量
    // ----------------------------------------------------------------------
    let opencv_home = env::var("OpenCV_HOME").expect("OpenCV_HOME 环境变量未设置！");
    let opencv_lib_dir = format!("{}/lib", opencv_home);
    let ffmpeg_home = env::var("FFMPEG_DIR").expect("FFMPEG_DIR 环境变量未设置！");
    let ffmpeg_lib_dir = format!("{}/lib", ffmpeg_home);

    println!("cargo:warning=OpenCV Lib Path: {}", opencv_lib_dir);

    // ----------------------------------------------------------------------
    // 2. 编译 C++ 静态库 (libcpp.a)
    // ----------------------------------------------------------------------
    let mut build = cc::Build::new();
    build
        .cpp(true)
        .std("c++17") // 必须与 OpenCV 编译版本一致
        .include("cpp/include")
        .include(format!("{}/include", opencv_home))
        .include(format!("{}/include/opencv4", opencv_home))
        .include(format!("{}/include", ffmpeg_home))
        .warnings(false);

    for entry in glob("cpp/src/*.cpp").expect("Failed to read glob pattern") {
        if let Ok(path) = entry {
            build.file(&path);
            println!("cargo:rerun-if-changed={}", path.display());
        }
    }
    // 生成 libcpp.a。cc crate 会自动发出 cargo:rustc-link-lib=static=cpp
    build.compile("cpp"); 

    // ----------------------------------------------------------------------
    // 3. 强制配置链接参数 (The Nuclear Option)
    // ----------------------------------------------------------------------
    // 我们不再使用 rustc-link-lib，而是使用 rustc-link-arg 直接传给 linker。
    // 这样可以保证参数绝对存在，且顺序是我们指定的顺序。

    // === OpenCV ===
    println!("cargo:rustc-link-search=native={}", opencv_lib_dir);
    println!("cargo:rustc-link-arg=-Wl,-rpath,{}", opencv_lib_dir);

    // 检查是否是 monolithic build (opencv_world)
    // 如果你确定你的 OpenCV 只有 libopencv_world.so，请只保留 opencv_world
    // 否则使用下面的完整列表。
    let opencv_libs = vec![
        // 高层库在前
        "opencv_gapi", "opencv_stitching", "opencv_calib3d", "opencv_features2d", 
        "opencv_highgui", "opencv_videoio", "opencv_photo", "opencv_imgcodecs", 
        "opencv_video", "opencv_dnn", "opencv_imgproc", "opencv_flann", "opencv_ml", 
        // 核心库在后
        "opencv_core" 
    ];

    for lib in opencv_libs {
        // 直接注入 -l 参数，强制链接
        println!("cargo:rustc-link-arg=-l{}", lib);  // -l 绕过 Cargo 的依赖检查，强制 gcc 链接这些库
    }

    // === FFmpeg ===
    println!("cargo:rustc-link-search=native={}", ffmpeg_lib_dir);
    println!("cargo:rustc-link-arg=-Wl,-rpath,{}", ffmpeg_lib_dir);

    let ffmpeg_libs = vec![
        "avdevice", "avfilter", "avformat", "avcodec", 
        "swresample", "swscale", "avutil", "postproc"
    ];
    for lib in ffmpeg_libs {
        println!("cargo:rustc-link-arg=-l{}", lib);
    }

    // === C++ Standard Library ===
    // 再次强制链接 stdc++
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    if target_os == "linux" {
        println!("cargo:rustc-link-arg=-lstdc++");
    } else if target_os == "macos" {
        println!("cargo:rustc-link-arg=-lc++");
    }

    Ok(())
}