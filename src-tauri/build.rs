fn main() {
    // Re-run Cargo when the ddraw shim DLL is rebuilt so include_bytes! picks up new bytes.
    println!("cargo:rerun-if-changed=../native/ddraw.dll");
    tauri_build::build()
}
