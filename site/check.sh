find . -name '*.md' -o -name '*.mdx' -print0 | xargs -0 cat | aspell list | while read -r word; do
  echo "Spelling error found: $word"
done
