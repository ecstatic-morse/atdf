#!/usr/bin/env bash

set -o errexit
set -o pipefail
#set -o xtrace

url="http://packs.download.atmel.com"
atpacks="Atmel.ATmega_DFP.1.2.203 Atmel.ATtiny_DFP.1.3.169"
outdir=tests/data
testfile="tests/data/suite.rs"

function get_pack {
    atpack=$1
    outdir="${outdir}/${atpack}"

    mkdir -p "$outdir"
    pushd "$outdir" >/dev/null

    echo "Fetching ${atpack}..."
    curl --silent "${url}/${atpack}.atpack" > "${atpack}.zip"

    yes | unzip "${atpack}.zip" "atdf/*" 2>/dev/null
    rm "${atpack}.zip"

    popd >/dev/null
}

for pack in $atpacks; do
    ( get_pack "$pack" ) &
done

wait

function gen_test {
    ident=${1##*/}
    cat >>${testfile} <<-EOF
		#[test]
		fn ${ident%.*}() {
		    parse("$1");
		}

	EOF
}

echo "Generating tests..."
mkdir -p `dirname ${testfile}`
echo -n "" > ${testfile}
for f in ${outdir}/*/atdf/*.atdf; do
    gen_test $f
done
