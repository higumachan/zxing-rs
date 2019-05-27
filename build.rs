use std::path::Path;
use glob::glob;
use std::env;
use std::vec;
use cc;

fn main() {
    let c_api_src_dir = Path::new("c_api/src");
    let c_api_sources: Vec<_> = glob(c_api_src_dir.join("*.cpp").to_str().unwrap())
        .unwrap()
        .map(|x| { x.unwrap() })
        .collect()
    ;
    let core_src_dir = Path::new("submodules/zxing-cpp/core/src");
    let core_sources: Vec<_> = glob(core_src_dir.join("**/*.cpp").to_str().unwrap())
        .unwrap()
        .map(|x| { x.unwrap() })
        .collect()
    ;

    cc::Build::new()
        .cpp(true)
        .flag("-std=c++11")
        .flag("-Wno-missing-braces")
        .include(core_src_dir)
        .files(core_sources)
        .skip_when_compiled(true)
        .compile("zxing_core")
    ;
    cc::Build::new()
        .cpp(true)
        .flag("-std=c++11")
        .flag("-v")
        .flag("-g")
        .include(c_api_src_dir)
        .include(core_src_dir)
        .files(c_api_sources)
        .compile("zxing_c_api")
    ;
}
