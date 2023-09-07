#!/bin/bash
set -e
cd $(git rev-parse --show-toplevel)

dir=changelogs
latest_tag=$(git describe --tags `git rev-list --tags --max-count=1`)
filepath=$dir/$latest_tag.md
if test -f "$filepath"; then
    cp $filepath $dir/current.md
else
    echo "No changelog found for tag $latest_tag."
    echo "[No changelog found]" > $dir/current.md
fi
