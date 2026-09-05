fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }

    let mut resource = winresource::WindowsResource::new();
    resource
        .set("FileDescription", "clipls")
        .set("ProductName", "clipls")
        .set("InternalName", "clipls.exe")
        .set("OriginalFilename", "clipls.exe")
        .set("CompanyName", "H2Depot")
        .set("LegalCopyright", "Copyright (c) 2026 H2Depot");

    resource
        .compile()
        .expect("failed to compile Windows resources");
}
