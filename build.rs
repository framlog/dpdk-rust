extern crate bindgen;

use std::env;
use std::path::{Path, PathBuf};
use std::fs::{read_dir, ReadDir, File, DirEntry};
use std::io::prelude::*;

fn foreach_in_dir<T>(dir_path: &String, mut f: T) where T: FnMut(DirEntry) {
    let mut dir_stack = Vec::<ReadDir>::new();
    dir_stack.push(read_dir(Path::new(dir_path)).unwrap());
    while dir_stack.len() > 0 {
        let cur_dir = dir_stack.pop().unwrap();
        for entry in cur_dir {
            if let Ok(entry) = entry {
                if entry.file_type().unwrap().is_dir() {
                    dir_stack.push(read_dir(entry.path()).unwrap());
                } else {
                    f(entry);
                }
            }
        }
    }
}

#[allow(unused_must_use)]
fn main() {
    // touch dpdk environment variables
    let rte_sdk = env::var("RTE_SDK").expect("No RTE_SDK found.");
    let rte_target = env::var("RTE_TARGET").expect("No RTE_TARGET found");
    let include_path = format!("{}/{}/include", rte_sdk, rte_target);
    let lib_path = format!("{}/{}/lib", rte_sdk, rte_target);

    // rerun only if files in `include_path` change. 
    println!("cargo:rerun-if-changed={}", include_path);

    // generate wrapper.h
    let mut lines = Vec::<String>::new();
    lines.extend_from_slice(&[
        "// automatic generated, don't modify it.".to_string(),
    ]);
    foreach_in_dir(&include_path, |ent| {
        if let Some(ext) = ent.path().extension() {
            if "h" != ext {
                return;
            }
            let file_name = ent.file_name();
            let filename = file_name.to_str().unwrap();
            if !filename.ends_with("_32.h") && !filename.ends_with("_64.h") {
                lines.push(format!("#include <{}>",
                                   ent.path().strip_prefix(&include_path).unwrap().to_str().unwrap()));
            }
        }
    });

    match File::create(env::var("CARGO_MANIFEST_DIR").unwrap() + "/src/wrapper.h") {
        Ok(mut file) => lines.into_iter().for_each(|l| {
            file.write((l + "\n").as_bytes());
        }),
        Err(reason) => {
            eprintln!("Create wrapper file failed: {}", reason);
            ::std::process::exit(1);
        }
    }

    // set linked library properly
    println!("cargo:rustc-link-search={}", lib_path);
    foreach_in_dir(&lib_path, |ent| {
        if let Some(ext) = ent.path().extension() {
            if "a" != ext {
                return;
            } 
            let file_name = ent.file_name();
            let filename = file_name.to_str().unwrap();
            if filename.starts_with("librte_") {
                println!("cargo:rustc-link-lib=static={}", &filename[3..filename.len()-2]);
            }
        }
    });

    let bindings = bindgen::Builder::default()
        .header("./src/wrapper.h")
        .blacklist_type("max_align_t")
        .clang_args(vec![format!("-I{}", include_path), "-msse4.2".to_string()])
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("dpdk.rs"))
        .expect("Couldn't write bindings!");
}
