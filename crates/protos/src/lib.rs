pub mod sslvision {
    include!(concat!(env!("OUT_DIR"), "/proto_sslvision.rs"));
}

pub mod gamecontroller {
    include!(concat!(env!("OUT_DIR"), "/proto_gamecontroller.rs"));
}

pub mod roboteam {
    include!(concat!(env!("OUT_DIR"), "/proto_roboteam.rs"));
}
