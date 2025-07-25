use cmake;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rustc-link-lib=dylib=resolv");
    
    let cpp = cmake::build("cpp");
    println!("cargo:rustc-link-search=native={}", cpp.display());
    println!("cargo:rustc-link-lib=static=cpp");
    println!("cargo:rustc-link-lib=c++"); // 链接c++标准库, Rust 默认不链接 C++ 库

    Ok(())
}