from .. import run
import os

run(
    "wasm-pack build crates/algo_models_ffi --target web --out-dir ../../build/typescript/algo_models -- --color always --no-default-features --features ffi_wasm"
)

# Remove the generated .gitignore file from the pkg directory
if os.path.exists("build/typescript/algo_models/.gitignore"):
    os.remove("build/typescript/algo_models/.gitignore")