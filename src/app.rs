use clap::{crate_authors, crate_description, crate_version, App, ArgMatches};

pub fn cli() -> ArgMatches<'static> {
    App::new("rdl")
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .get_matches()
}
