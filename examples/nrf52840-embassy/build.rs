// Copyright © 2023 Analog Devices, Inc. All Rights Reserved. This software is
// proprietary to Analog Devices, Inc. and its licensors.
//
// SPDX-License-Identifier: Apache-2.0
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::env;
use std::fs;
use std::path::PathBuf;
fn main() {
    let memory_layout = "nRF52840.x";

    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    println!("cargo:rustc-link-search={}", out.display());
    println!("selected file: {}", memory_layout);
    fs::copy(memory_layout, out.join("memory.x")).unwrap();
    println!("cargo:rerun-if-changed={}", memory_layout);
    println!("cargo:rustc-link-arg-bins=--nmagic");
    println!("cargo:rustc-link-arg-bins=-Tlink.x");
    println!("cargo:rustc-link-arg-bins=-Tdefmt.x");
    println!("cargo:rerun-if-changed=build.rs");
}
