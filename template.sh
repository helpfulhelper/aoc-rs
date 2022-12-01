#!/bin/bash
AOC_BASE_DIR="/home/ast/Projects/aoc/aoc-rs"
AOC_INPUT_DIR="/home/ast/Projects/aoc/input"
TEMPLATE="${AOC_BASE_DIR}/src/bin/template/template.rs"

YEAR=$1
DAY=$2

if [ -z ${YEAR} ]; then
    YEAR = `date +%Y`
fi

if [ -z ${DAY} ]; then
    DAY = `date +%d`
fi

BASE="aoc${YEAR}"
NEW_FILE="aoc${YEAR}_${DAY}.rs"
MOD_FILE="${AOC_BASE_DIR}/src/bin/${BASE}/mod.rs"
NEW_FULL_PATH="${AOC_BASE_DIR}/src/bin/${BASE}/${NEW_FILE}"


aoc_dl() {
    aoc -y ${YEAR} -d ${DAY} -o -i ${AOC_INPUT_DIR}/${YEAR}/${DAY}.txt download
}

aoc_read() {
    aoc -y ${YEAR} -d ${DAY} -o -p ${AOC_INPUT_DIR}/${YEAR}/${DAY}.md read > /dev/null
}

echo "Downloading input..."
aoc_dl
echo "Downloading puzzle description..."
aoc_read
echo "Copying template"
cp "${TEMPLATE}" "${NEW_FULL_PATH}"
echo "Fixing years in template"
sed -i "s/YYYY/${YEAR}/g" ${NEW_FULL_PATH}
echo "Fixing days in template"
sed -i "s/DD/${DAY}/g" ${NEW_FULL_PATH}
echo "Fixing first half of mod.rs"
sed -i "/aoc${YEAR}_${DAY}/ s/\/\/.//" ${MOD_FILE}
echo "Fixing second half of mod.rs"
sed -i "/day${DAY}/ s/\/\/.//" ${MOD_FILE}
