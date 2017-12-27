error_chain! {
    foreign_links {
        Io(::std::io::Error) #[doc = "IO error"];
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
    }
}
