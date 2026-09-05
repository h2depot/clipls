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
        .set("CompanyName", "h2depot")
        .set("LegalCopyright", "Copyright (c) 2026 h2depot");

    resource
        .compile()
        .expect("failed to compile Windows resources");
}
