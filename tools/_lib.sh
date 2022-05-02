
if [ -z "$ENVIRON" ]; then
  echo "\$ENVIRON is not defined"
  exit 1
fi

if [ -z "$GITHASH" ]; then
  GITHASH=$(git log --pretty=format:'%h' -n 1)
fi
if [ -z "$VERSION" ]; then
  VERSION=$GITHASH
fi
if [ -z "$INSTAUNIT" ]; then
  INSTAUNIT=instaunit
fi

append_flags () {
  if [ -z "$1" ]; then
    echo "$2"
  else
    echo "$1 $2"
  fi
}
