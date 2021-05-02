#!/usr/bin/bash

cd static

git add
git commit

git push git@github.com:chmlee/ream-editor.git master:gh-pages

cd -
