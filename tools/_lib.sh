
GITHASH=${GITHASH:=$(git log --pretty=format:'%h' -n 1)}
VERSION=${VERSION:=$GITHASH}
INSTAUNIT=${INSTAUNIT:=instaunit}

append_flags () {
  if [ -z "$1" ]; then
    echo "$2"
  else
    echo "$1 $2"
  fi
}

assert_flag () {
  if [ -z "$1" ]; then
    echo "*** $2"
    exit 1
  fi
}
