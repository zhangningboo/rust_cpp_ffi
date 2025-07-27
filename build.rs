use std::env;
use cmake;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let cpp = cmake::build("cpp");
    println!("cargo:rustc-link-search=native={}", cpp.display());
    println!("cargo:rustc-link-lib=static=cpp");

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
    Ok(())
}