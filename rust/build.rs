fn main() {


    // 如果在macos上运行时，一下全关闭

    // 设置对应的库
    //  编译 ios模拟器时 打开这个注释 cargo lipo --targets aarch64-apple-ios-sim --release
    // println!("cargo:rustc-link-search=native=./ios_curl/artifacts-iphonesimulator/lib");


    //  编译 ios真机时 打开这个注释 cargo lipo --targets aarch64-apple-ios --release
    // println!("cargo:rustc-link-search=native=./ios_curl/artifacts-iphoneos/lib");
}