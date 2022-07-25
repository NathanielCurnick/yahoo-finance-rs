use protobuf_codegen::{Codegen, Customize};

fn main() {
    // Build our realtime feed structure
    Codegen::new()
        .out_dir("src/yahoo")
        .inputs(&["src/yahoo/realtime.proto"])
        .includes(&["src"])
        .customize(Customize::default().gen_mod_rs(false))
        .run()
        .expect("Codegen failed.");
}
