use cmake;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let cpp = cmake::build("cpp");
    println!("cargo:rustc-link-search=native={}", cpp.display());
    println!("cargo:rustc-link-lib=static=cpp");

    // 链接c++标准库, Rust 默认不链接 C++ 库
    println!("cargo:rustc-link-lib=stdc++");  // Linux
    // println!("cargo:rustc-link-lib=c++"); // macOS/Clang 常用

    Ok(())
}