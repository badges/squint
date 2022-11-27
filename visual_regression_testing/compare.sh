#!/bin/bash

set -euo pipefail

rm -rf test_images
mkdir -p test_images

rm -rf diffs
mkdir -p diffs

readarray -t test_cases < <(jq -c '.[]' test_cases.json)

# from this point on, continue even if something fails
# we want to continue testing further reference_images, even if one fails
set +e

exitcode=0
for test_case in "${test_cases[@]}"; do
    url=$(jq -r '.url' <<< "$test_case")
    filename=$(jq -r '.filename' <<< "$test_case")
    echo "Testing: $filename";
    curl "$url" --silent -o "test_images/$filename"
    if ./node_modules/.bin/pixelmatch "test_images/$filename" "reference_images/$filename" "diffs/$filename"; then
        # if the images were exactly the same, discard the diff
        rm "diffs/$filename"
    else
        # one or more images has failed
        exitcode=1
    fi
    echo ""
done

exit $exitcode
