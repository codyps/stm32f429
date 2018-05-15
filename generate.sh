#! /bin/sh

d="$(dirname "$0")"
t=
on_exit() {
	if [ "$t" ]; then
		rm -f "$t"
	fi
}
trap on_exit EXIT
t=$(mktemp)

cp "$1" "$t" || exit 1
patch -lN "$t" STM32F429.patch

svd2rust --target cortex-m -i "$t" || exit 1
rustfmt lib.rs
rustfmt build.rs
mkdir -p src
mv lib.rs src/
