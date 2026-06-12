pub mod glypheris {
    pub mod animation {
        include!(concat!(env!("CARGO_MANIFEST_DIR"), "/gen/glypheris.animation.rs"));
    }
    pub mod agents {
        include!(concat!(env!("CARGO_MANIFEST_DIR"), "/gen/glypheris.agents.rs"));
    }
    pub mod inference {
        include!(concat!(env!("CARGO_MANIFEST_DIR"), "/gen/glypheris.inference.rs"));
    }
}
