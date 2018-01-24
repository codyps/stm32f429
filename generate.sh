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

cp "$1" "$t"
patch "$t" STM32F429.patch
svd2rust -i "$t" | rustfmt >"$d/src/lib.rs"
