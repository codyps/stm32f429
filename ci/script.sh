set -ex

dos2unix() {
    tr -s '\r' '\n'
}

main() {
    local svd=STMicro/STM32F429.svd

    cargo check

    # check than the patch can be applied to the original SVD
    local url=https://github.com/tones111/cmsis-svd/raw/stm32f4/data/$svd
    local svd=$(basename $svd)
    local patch="${svd%.*}.patch"
    curl -LO $url
    dos2unix < $svd > $svd.nl
    mv $svd.nl $svd
    dos2unix < $patch | patch -p1 $svd
}

main
