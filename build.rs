// Build script entry point
fn main() {
    // This code block only compiles when building for Windows
    #[cfg(target_os = "windows")]
    {
        // Path to the Windows resource file that defines the application icon
        let resource_path = "assets/windows/app.rc";
        
        // Compile and embed the resource file into the Windows executable
        // This allows Windows to use the .ico file as the application icon
        // The type parameters are:
        // - _: Automatically infer the resource path type
        // - String: Type for macro strings
        // - Vec<String>: Type for the collection of macros
        embed_resource::compile::<_, String, Vec<String>>(resource_path, vec![]);
    }
}