// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Copyright (c) 2026 c1ph3rc4t

//! Compile morton.ll

#![allow(clippy::unwrap_used)]

fn main() {
    println!("cargo:rerun-if-changed=src/llvm_ir/morton.ll");
    let target = std::env::var("TARGET").unwrap();
    let src = std::fs::read_to_string("src/llvm_ir/morton.ll").unwrap();
    let out = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("morton.ll");
    std::fs::write(&out, format!("target triple = \"{target}\"\n{src}")).unwrap();

    cc::Build::new()
        .compiler("clang")
        .file(&out)
        .compile("morton");
}
