from .. import run

import itertools
import os
import shutil

# For now just support Tier 1 & 2 targets: https://doc.rust-lang.org/nightly/rustc/platform-support.html
#
# For some targets we need to combine binaries, thus we have targets and fat_targets
# See https://developer.apple.com/forums/thread/666335
targets = ["aarch64-apple-ios"]

fat_targets = {
    "ios-sim": [
        "x86_64-apple-ios",
        "aarch64-apple-ios-sim",
    ],
    "catalyst": [
        "x86_64-apple-ios-macabi",
        "aarch64-apple-ios-macabi",
    ],
    "macos": [
        "x86_64-apple-darwin",
        "aarch64-apple-darwin",
    ],
}

cargo_build_cmd = "cargo --color always build --features ffi_uniffi"

all_targets = list(itertools.chain.from_iterable(fat_targets.values())) + targets
for target in all_targets:
    run(f"rustup target add {target}")
    cargo_build_cmd += f" --target {target}"

run(cargo_build_cmd)
run(
    "cargo --color always run -p uniffi-bindgen generate --no-format --library target/aarch64-apple-darwin/debug/libalgo_models_ffi.dylib --language swift --out-dir target/debug/swift/algo_models"
)

create_xcf_cmd = "xcodebuild -create-xcframework"
for target in targets:
    create_xcf_cmd += f" -library target/{target}/debug/libalgo_models_ffi.dylib"

for fat_target_name in fat_targets:
    lib_paths: list[str] = []
    for target in fat_targets[fat_target_name]:
        lib_paths.append(f"target/{target}/debug/libalgo_models_ffi.dylib")

    run(
        f"lipo -create {' '.join(lib_paths)} -output target/debug/libalgo_models_ffi-{fat_target_name}.dylib",
    )
    create_xcf_cmd += (
        f" -library target/debug/libalgo_models_ffi-{fat_target_name}.dylib"
    )

create_xcf_cmd += " -headers target/debug/swift/algo_models/ -output target/debug/algo_models.xcframework"

if os.path.exists("target/debug/algo_models.xcframework"):
    shutil.rmtree("target/debug/algo_models.xcframework")

# xcframework needs the modulemap to be named module.modulemap
os.rename(
    "target/debug/swift/algo_models/algo_modelsFFI.modulemap",
    "target/debug/swift/algo_models/module.modulemap",
)

# replace var with let to resolve swift concurrency issues
# I believe this is fixed in https://github.com/mozilla/uniffi-rs/pull/2294
# The above PR is available in uniffi-rs 0.29.0, but we won't be updating until
# Nord generators (i.e. Golang) are updated to use 0.29.0
with open("target/debug/swift/algo_models/algo_models.swift", "r") as file:
    content = file.read()

content = content.replace("private var initializationResult", "private let initializationResult")

with open("target/debug/swift/algo_models/algo_models.swift", "w") as file:
    file.write(content)

shutil.move(
    "target/debug/swift/algo_models/algo_models.swift",
    "build/swift/AlgoModels/Sources/AlgoModels/AlgoModels.swift",
)

run(create_xcf_cmd)