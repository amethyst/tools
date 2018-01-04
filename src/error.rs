error_chain! {
    foreign_links {
        Io(::std::io::Error) #[doc = "IO error"];
        Version(::semver::ReqParseError) #[doc = "Could not parse version"];
    }

    errors {
        New(name: String) {
            description("project creation failed")
            display("project creation for project {:?} failed", name)
        }

        /// Failed to fetch amethyst crate version from crates.io
        FetchVersionFailure {
            description("Failed to fetch latest version of amethyst")
        }

        /// The fetched crates.io JSON is not valid
        InvalidCratesIoJson {
            description("The JSON fetched from crates.io is invalid")
        }

        /// Don't have a template matching this version
        UnsupportedVersion(version: String) {
            description("Unsupported version of Amethyst requested")
            display("This version of amethyst_tools does not support the requested version {:?}", version)
        }

    }
}
