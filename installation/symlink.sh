TARGET_PATH="$(pwd)/node_modules/@repokit/core"

mkdir -p $TARGET_PATH

ln -s $(pwd)/externals $TARGET_PATH || true