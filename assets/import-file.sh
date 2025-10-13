#!/usr/bin/env bash

# Local testing script to import a file into the local server given the authentication token.
# Usage: ./import-file.sh <TOKEN>
# TOKEN: authentication token for the cookie

if [ $# -ne 1 ]; then
    echo "Usage: $0 <TOKEN>"
    exit 1
fi

TOKEN="$1"
FILE_NAME='Orario Provvisorio 5 ore v5.xml'
BEGIN_TS='2025-09-15T08:00:00'
END_TS='2026-06-10T18:00:00'
MODE='write' # or 'dry-run'

# URL-encode the file name for the query string
ENCODED_FILE_NAME=$(python3 -c 'import urllib.parse,sys; print(urllib.parse.quote(sys.argv[1]))' "$FILE_NAME")

curl "http://localhost:3000/import/file?begin_ts=$BEGIN_TS&end_ts=$END_TS&mode=$MODE&file_name=$ENCODED_FILE_NAME" \
  --request POST \
  --header 'Content-Type: application/xml' \
  --cookie "meucci_presenze_id=$TOKEN" \
  --data-binary "@${FILE_NAME}"
