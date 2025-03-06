use anyhow;

fn main() {
    /*
       --- What is the OUT_DIR? ---
       The OUT_DIR is an environment variable that is set by Cargo when it runs the build script. It is the path to the
       directory where the build script should write its output. This means that all the .rs files that are generated
       by prost_build::compile_protos will be written to this folder. This folder is probably named somewhere in the
        direction of ./target/(debug|release)/build/protos-{SOME_HASH_HERE}/out/

        --- What is _.rs? ---
        _.rs is the file that contains the generated code of all protobuf files that do not have a package declaration.
        Since none of the protobuf files have a package declaration, they are all put in the file OUT_DIR/_.rs . It is
        therefore needed to rename this file after every call to prost_build::compile_protos to prevent it from being
        overwritten by the next call.
    */


    build_proto_sslvision()
    .expect("Failed to compile proto files within proto_sslvision");

    build_proto_gamecontroller()
    .expect("Failed to compile proto files within proto_gamecontroller");

    build_proto_roboteam()
    .expect("Failed to compile proto files within proto_roboteam");

}

fn build_proto_sslvision() -> anyhow::Result<()> {
    prost_build::compile_protos(
        &[
            "proto_sslvision/messages_robocup_ssl_detection_tracked.proto",
            "proto_sslvision/messages_robocup_ssl_detection.proto",
            "proto_sslvision/messages_robocup_ssl_geometry_legacy.proto",
            "proto_sslvision/messages_robocup_ssl_geometry.proto",
            "proto_sslvision/messages_robocup_ssl_wrapper_legacy.proto",
            "proto_sslvision/messages_robocup_ssl_wrapper_tracked.proto",
            "proto_sslvision/messages_robocup_ssl_wrapper.proto",
        ],
        &["proto_sslvision"],
    )?;

    // Since these protobuf files do not have a package declaration, they are all put in the file OUT_DIR/_.rs
    // This file needs to be renamed to prevent it being overwritten by the next call to prost_build::compile_protos
    std::fs::rename(
        std::path::Path::new(std::env::var("OUT_DIR").unwrap().as_str()).join("_.rs"),
        std::path::Path::new(std::env::var("OUT_DIR").unwrap().as_str()).join("proto_sslvision.rs"),
    )?;
    Ok(())
}

fn build_proto_gamecontroller() -> anyhow::Result<()> {
    prost_build::compile_protos(
        &[
            "proto_gamecontroller/state/ssl_gc_common.proto",
            "proto_gamecontroller/state/ssl_gc_game_event.proto",
            "proto_gamecontroller/state/ssl_gc_referee_message.proto",
            "proto_gamecontroller/state/ssl_gc_state.proto",
            "proto_gamecontroller/geom/ssl_gc_geometry.proto",
        ],
        &["proto_gamecontroller"],
    )?;

    // Since these protobuf files do not have a package declaration, they are all put in the file OUT_DIR/_.rs
    // This file needs to be renamed to prevent it being overwritten by the next call to prost_build::compile_protos
    std::fs::rename(
        std::path::Path::new(std::env::var("OUT_DIR").unwrap().as_str()).join("_.rs"),
        std::path::Path::new(std::env::var("OUT_DIR").unwrap().as_str()).join("proto_gamecontroller.rs"),
    )?;
    Ok(())
}

fn build_proto_roboteam() -> anyhow::Result<()> {
    prost_build::compile_protos(
        &[
            "proto_roboteam/RobotCommands.proto",
            "proto_roboteam/RobotFeedback.proto",
            "proto_roboteam/RobotParameters.proto",
            "proto_roboteam/RobotProcessedFeedback.proto",
            "proto_roboteam/Vector2f.proto",
            "proto_roboteam/World.proto",
            "proto_roboteam/WorldBall.proto",
            "proto_roboteam/WorldRobot.proto",
        ],
        &["proto_roboteam"],
    )?;

    // Since these protobuf files do not have a package declaration, they are all put in the file OUT_DIR/_.rs
    // This file needs to be renamed to prevent it being overwritten by the next call to prost_build::compile_protos
    std::fs::rename(
        std::path::Path::new(std::env::var("OUT_DIR").unwrap().as_str()).join("_.rs"),
        std::path::Path::new(std::env::var("OUT_DIR").unwrap().as_str()).join("proto_roboteam.rs"),
    )?;
    Ok(())
}