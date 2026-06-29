fn main() {
    if std::env::consts::OS != "windows" {
        println!("This script is designed for and only works on Windows 10 and onwards.")
    } else {
        let mut winres = winresource::WindowsResource::new();
        winres.set_manifest(
            r#"<assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
            <trustInfo xmlns="urn:schemas-microsoft-com:asm.v3">
                <security>
                    <requestedPrivileges>
                        <requestedExecutionLevel level="requireAdministrator" uiAccess="false" />
                    </requestedPrivileges>
                </security>
            </trustInfo>
            </assembly>"#,
        );
        winres.compile().unwrap();
    }
}
