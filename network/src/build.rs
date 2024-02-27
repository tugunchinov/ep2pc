extern crate prost_build;

fn main() {
    prost_build::compile_protos(
        &[
            "src/messages/discovery/requests/requests.proto",
            "src/messages/discovery/responses/responses.proto",
        ],
        &["src/"],
    )
    .expect("failed building protos");
}
