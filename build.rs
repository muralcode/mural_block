fn main() {
    cc::Build::new()
        .cpp(true)
        .include("C:/eigen-3.4.0")
        .file("src/adaptive_kmeans/adaptivekmeans.cpp")
        .compile("adaptivekmeans");

    // // Using bindgen to generate Rust bindings for C/C++ headers
    // let _bindings = bindgen::Builder::default()
    //     .header("src/adaptive_kmeans/adaptivekmeans.h")
    //     .generate()
    //     .expect("Unable to generate bindings");
}
