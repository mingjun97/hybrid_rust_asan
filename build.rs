fn main(){
    println!("cargo:rustc-link-search=native=./lib");
    println!("cargo:rustc-link-lib=dylib=my");
    println!("cargo:rustc-link-lib=dylib=asan");
}