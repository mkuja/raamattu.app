#!/bin/bash

# Download versions of Google Chrome and chromedriver.
#
# Usage examples:
#   ./gc-dl.sh versions chrome                          --  List 10 newest chrome downloads.
#   ./gc-dl.sh download chrome 133.0.6911.0 downloads   --  Download version 133.0.6911.0 into downloads directory.
#   ./gc-dl.sh versions chromedriver                    --  List 10 newest chrome downloads.
#   ./gc-dl.sh download chromedriver 133.0.6911.0 downloads/  --  Download chromedriver of a version.
#   ./gc-dl.sh unzip downloads                          -- Unzip everything in downloads to downloads.


set -euo pipefail
IFS=$'\n\t'

URL="https://googlechromelabs.github.io/chrome-for-testing/known-good-versions-with-downloads.json"


VERSION_DATA=$(mktemp)
CHROME_VERSIONS_TO_DOWNLOAD_URLS=$(mktemp)
CHROMEDRIVER_DOWNLOAD_LINKS=$(mktemp)

# Fetch Chrome versions and download links into $CHROME_DOWNLOAD_LINKS
fetch_chrome_download_urls() {
  curl $URL > $VERSION_DATA 2>/dev/null
  if [ $? -ne 0 ]
  then
    logger -p local0.error --std-err Network error: curl failed with code ${@?}
    exit 1
  fi

  local VERSIONS=$(mktemp) # Pasted with download urls down below
  cat $VERSION_DATA | jq '.versions[].version' > $VERSIONS
  if [ $? -ne 0 ]
  then
    logger -p local0.error --std-err Error processing version data. jq exited with ${@?}
    return $?
  fi


  local CHROME_DOWNLOAD_URLS=$(mktemp) # Pasted with versions down below
  cat $VERSION_DATA | jq '.versions[].downloads.chrome | select(. != null)[] | select(.platform == "linux64").url' > $CHROME_DOWNLOAD_URLS
  if [ $? -ne 0 ]
  then
    logger -p local0.error --std-err Error processing version data. jq exited with ${@?}
    return $?
  fi

  paste $VERSIONS $CHROME_DOWNLOAD_URLS > $CHROME_VERSIONS_TO_DOWNLOAD_URLS
  if [ $? -ne 0 ]
  then
    local RET=@?
    logger -p local0.error --std-err Error processing version data. paste exited with ${@?}
    return $RET
  fi
  
  return $?
}

# Fetch chromedriver versions and download links into $CHROMEDRIVER_DOWNLOAD_LINKS
show_10_latest_of_chromedriver() {
  curl $URL > $VERSION_DATA 2>/dev/null
  if [ $? -ne 0 ]
  then
    logger -p local0.error --std-err Network error: curl failed with code ${@?}
    exit 1
  fi

  jq '.versions[].downloads.chromedriver | select(. != null)[] | select(.platform == "linux64").url' $VERSION_DATA > $CHROMEDRIVER_DOWNLOAD_LINKS
  tac $CHROMEDRIVER_DOWNLOAD_LINKS | head | sed -E 's#([^0-9]*)([0-9.]+)([^"]*)"#\2: \1\2\3#'
}


show_10_latest_of_chrome() {
  fetch_chrome_download_urls
  echo "From latest version to older ones..."
  cat $CHROME_VERSIONS_TO_DOWNLOAD_URLS | sed -E 's#"([^"]*)"[[:space:]]+"([^"]*)"#\1: \2#' | tail | tac
}

download_chromedriver() {
  local VERSION=$1
  local SAVE_DIR=$2
  jq '.versions[].downloads.chromedriver | select(. != null)[] | select(.platform == "linux64").url' $VERSION_DATA > $CHROMEDRIVER_DOWNLOAD_LINKS
  local LINK=$(grep "$VERSION" $CHROMEDRIVER_DOWNLOAD_LINKS | sed -E 's#"([^"]*)"#\1#')
  curl -o ${SAVE_DIR}/chromedriver.zip "$LINK"
}

download() {
  fetch_chrome_download_urls
  echo "VARS: " $@
  case $1 in
    chrome)
      local VERSION_AND_LINK=$(grep -e $2 $CHROME_VERSIONS_TO_DOWNLOAD_URLS)
      local VERSION=$(echo $VERSION_AND_LINK | sed -E 's#"([^" ]*).*#\1#')
      local LINK=$(echo $VERSION_AND_LINK | sed -E 's#"([^" ]*)[" :]+([^"]+)"#\2#')
      echo version: $VERSION
      echo link: $LINK
      local VERSION_LEN=$(echo $VERSION | wc -m)
      echo Attempting to fetch $VERSION
      if [[ -d "$3" && $VERSION_LEN -gt 1 ]]
      then
        mkdir -p $3
        curl -o "$3/chrome-$2.zip" "$LINK"
      else
        logger -p local0.error Version doesn\'t exist: $(grep -e $2 $CHROME_VERSIONS_TO_DOWNLOAD_URLS)
      fi
      ;;
    chromedriver)
      shift
      download_chromedriver $@
      ;;
  esac
}

versions() {
  case $1 in
    chrome)
      show_10_latest_of_chrome
      ;;
    chromedriver)
      show_10_latest_of_chromedriver
      ;;
  esac
}

unzip_() {
  if [ -d $1 ]
  then
    cd "$1"
    for FILE in *.zip
    do
      unzip $FILE
    done
  fi
}

main() {
  case $1 in
    download)
      shift
      download $@
      ;;
    unzip)
      shift
      unzip_ $@
      ;;
    versions)
      shift
      versions $@
      ;;
    *)
      echo "Usage: $0 [command] [parameters]

  Script $0 can be used to download, install or show versions of available
  Chrome or Chromedriver.

  Commands:
  download - Download a version of Chrome or Chromedriver
  unzip    - Install a version of Chrome or Chromedriver
  versions - Show versions

  see $0 `[command] help` for help.
    "
      ;;
  esac
}

main $@

exit 0

