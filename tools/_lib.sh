
if [ -z "$ENVIRON" ]; then
  ENVIRON=development
fi
if [ -z "$GITHASH" ]; then
  GITHASH=$(git log --pretty=format:'%h' -n 1)
fi
if [ -z "$VERSION" ]; then
  VERSION=$GITHASH
fi
