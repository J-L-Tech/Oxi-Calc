use {std::env, winresource::WindowsResource,};

fn main() {
    if env::var_os("CARGO_CFG_WINDOWS").is_some() {
        WindowsResource::new()
            // This path can be absolute, or relative to your crate root.
            .set_icon("math-calc-icon.ico")
            .compile().unwrap();
    }
    slint_build::compile("ui/appwindow.slint").unwrap();
}
