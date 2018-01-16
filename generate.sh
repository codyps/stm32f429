#! /bin/sh
d="$(dirname "$0")"
svd2rust -i STM32F429.svd >"$d/src/lib.rs"
rustfmt "$d/src/lib.rs"
