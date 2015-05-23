#!/bin/bash

set -o errexit -o nounset

rev=$(git rev-parse --short HEAD)

cargo doc

cd target/doc
echo "<meta http-equiv=refresh content=0;url=geom/index.html>" > index.html

git init
git config user.name "Brandon King"
git config user.email "wektorious@gmail.com"
git remote add upstream "https://$GH_TOKEN@github.com/Vectorious/geom-rs.git"
git fetch upstream
git reset upstream/gh-pages

touch .

git add -A .
git commit -m "rebuild pages at ${rev}"
git push -q upstream HEAD:gh-pages
