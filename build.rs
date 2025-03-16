fn main() {
    let config =
    slint_build::CompilerConfiguration::new()
    .with_style("material-dark".into());
    slint_build::compile_with_config("ui/app-window.slint", config).unwrap();
    // slint_build::compile("ui/app-window.slint").expect("Slint build failed");
}
