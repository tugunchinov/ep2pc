extern crate prost_build;

fn main() {
    prost_build::compile_protos(
        &[
            "src/discovery/requests/requests.proto",
            "src/discovery/responses/responses.proto",
        ],
        &["src/"],
    )
    .expect("failed building protos");
}
