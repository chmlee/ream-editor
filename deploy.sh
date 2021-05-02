#!/usr/bin/bash

cd static

rm .gitignore

git add -A
git commit

git push git@github.com:chmlee/ream-editor.git master:gh-pages

cd -
