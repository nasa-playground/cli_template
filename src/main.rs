use clap::{crate_description, crate_name, crate_version, App, AppSettings, Arg};

type Result<T> = std::result::Result<T, Error>;
type Error = Box<dyn std::error::Error + Send + Sync>;

fn main() -> Result<()> {
    let app = build_app();

    let matches = app.get_matches();

    Ok(())
}

fn build_app() -> App<'static, 'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .setting(AppSettings::DeriveDisplayOrder)
        .setting(AppSettings::ColoredHelp)
}
