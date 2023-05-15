use ::std::env;
use ::std::fs;
use ::std::path::PathBuf;

fn generate(dst: &str, src: &[&str]) {
    let mut out = PathBuf::from(env::var("OUT_DIR").unwrap());
    out.push(dst);

    let code = ::iamgroot::gen_code(src);

    fs::write(&out, code).unwrap_or_else(|err| {
        panic!(
            "Failed to write generated code to `{}`: {}",
            out.display(),
            err,
        )
    });
}

fn main() {
    generate(
        "gen.rs",
        &[
            "./api/starknet_api_openrpc.json",
            "./api/starknet_write_api.json",
            "./api/starknet_trace_api_openrpc.json",
        ],
    );
}
