if ! command -v docker && command -v docker.exe &>/dev/null; then
  DOCKER=docker.exe
else
  DOCKER=docker
fi

if ! command -v cargo &>/dev/null && command -v cargo.exe &>/dev/null; then
  CARGO=cargo.exe
else
  CARGO=cargo
fi

set -e
REGISTRY=registry.jhalsey.com
TAG=$REGISTRY/kiwi-admin:latest
$CARGO clean
$DOCKER build -t $TAG .
$DOCKER login $REGISTRY
$DOCKER push $TAG
