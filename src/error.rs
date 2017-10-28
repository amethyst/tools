error_chain! {
    foreign_links {
        Io(::std::io::Error) #[doc = "IO error"];
    }

    errors {
        New(name: String) {
            description("project creation failed")
            display("project creation for project {:?} failed", name)
        }
    }
}
