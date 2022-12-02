use std::io;
use std::env;
use std::path::Path;
use std::path::PathBuf;
extern crate fs_extra;
use fs_extra::dir::CopyOptions;
use fs_extra::dir::copy;

#[cfg(windows)] use winres::WindowsResource;

fn main() -> io::Result<()> {
    let output_path = env::var("OUT_DIR").unwrap();
    let output_path = get_output_path();


    println!("output path");
    println!("cargo:rerun-if-changed=config.json");
    println!("cargo:warning=Hello from build.rs");
    println!("cargo:warning=CWD is {:?}", env::current_dir().unwrap());
    println!("cargo:warning=OUT_DIR is {:?}", env::var("OUT_DIR").unwrap());
    println!("cargo:warning=CARGO_MANIFEST_DIR is {:?}", env::var("CARGO_MANIFEST_DIR").unwrap());
    println!("cargo:warning=PROFILE is {:?}", env::var("PROFILE").unwrap());

    println!("cargo:warning=Calculated build path: {:?}", output_path);

    let options = CopyOptions::new();

    let input_path = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("res");
    let output_path = Path::new(&output_path);

    println!("cargo:warning={:?} {:?}", input_path, output_path);

    let res = copy("./res", output_path.as_os_str().to_str().unwrap(), &options);

    if res.is_ok() {
        res.unwrap();
    }


    #[cfg(windows)] {
        WindowsResource::new()
            // This path can be absolute, or relative to your crate root.
            .set_icon("res/icon.ico")
            .compile()?;
    }
    Ok(())
}

fn get_output_path() -> PathBuf {
    //<root or manifest path>/target/<profile>/
    let manifest_dir_string = env::var("CARGO_MANIFEST_DIR").unwrap();
    let build_type = env::var("PROFILE").unwrap();
    let path = Path::new(&manifest_dir_string).join("target").join(build_type);
    return PathBuf::from(path);
}