#!/usr/bin/env bash
set -eu

PROJECT_ROOT=$(dirname "$(realpath "$(dirname "$0")")")

CARGO_MAKE_VERSION=0.37.23
DASEL_VERSION=2.8.1

i=1
echo "${i}. Install cargo-make for task runner (https://github.com/sagiegurari/cargo-make)"
CARGO_MAKE_PKG=cargo-make-v${CARGO_MAKE_VERSION}-x86_64-unknown-linux-musl
curl -SLf https://github.com/sagiegurari/cargo-make/releases/download/${CARGO_MAKE_VERSION}/${CARGO_MAKE_PKG}.zip \
    -o ${CARGO_MAKE_PKG}.zip
unzip -o ${CARGO_MAKE_PKG}.zip "${CARGO_MAKE_PKG}/makers"
chmod +x "${CARGO_MAKE_PKG}/makers"
mv "${CARGO_MAKE_PKG}/makers" "${PROJECT_ROOT}/makers"
rm -rf ${CARGO_MAKE_PKG} ${CARGO_MAKE_PKG}.zip
"${PROJECT_ROOT}"/makers --version
#cargo install cargo-make
echo -e "Done $((i++)).\n"

echo "${i}. Copy .env template"
cp "${PROJECT_ROOT}/.env-template" "${PROJECT_ROOT}/.env"
echo -e "Done $((i++)).\n"

TOOLS_BIN_DIR=${PROJECT_ROOT}/bin
rm -rf "${TOOLS_BIN_DIR}" "${PROJECT_ROOT}/node_modules"
mkdir -p "${TOOLS_BIN_DIR}"

echo "${i}. Install formatters for this project"
pnpm install --verbose
echo -e "Done $((i++)).\n"

echo "${i}. Install dasel to extract values from a TOML file."
DASEL_IMAGE_NAME=ghcr.io/tomwright/dasel
if ! docker images | grep "${DASEL_IMAGE_NAME}" | grep "${DASEL_VERSION}"; then
    echo "Image not found. Pulling ${DASEL_IMAGE_NAME}:${DASEL_VERSION}..."
    docker pull "${DASEL_IMAGE_NAME}:${DASEL_VERSION}"
else
    echo "Image ${DASEL_IMAGE_NAME}:${DASEL_VERSION} already exists."
fi

cat << EOF > "${TOOLS_BIN_DIR}/dasel"
#!/usr/bin/env bash
docker run -i --rm ${DASEL_IMAGE_NAME}:${DASEL_VERSION} "\$@"
EOF
chmod +x "${TOOLS_BIN_DIR}/dasel"
"${TOOLS_BIN_DIR}/dasel" --version
echo -e "Done $((i++))\n"
