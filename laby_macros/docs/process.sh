#!/bin/sh
# Processes the MDN HTML element docs for rustdoc inclusion.
set -e

# clone if not already
if [ -d "content" ]; then
  (cd "content" && git pull)
else
  git clone "https://github.com/mdn/content" content/
fi

# delete existing processed files
find * -maxdepth 0 ! -name 'README.md' ! -name 'LICENSE' ! -name 'process.sh' -type f -exec rm -f {} +

# incredibly ugly regex but gets the job done (mostly, hopefully)
reg=$(cat <<'EOF'
  s/^---.*?---//ms;
  s/^(##|<table).*//ms;

  s/\[(?<n>.+?)\]\(\/(?<s>.*?)\)/[$+{n}](https:\/\/developer.mozilla.org\/$+{s})/gs;

  s/\{\{\s*(HTMLRef|Specifications|Compat|non-standard_header|SeeCompatTable|EmbedInteractiveExample.*?|EmbedLiveSample.*?)\s*\}\}//gis;

  s/\{\{\s*(Glossary|anch|CSSxref)\s*\(\s*(.*?,)?\s*['"](?<s>.+?)['"]\s*\)\s*\}\}/$+{s}/gis;

  s/\{\{\s*(htmlattrxref|HTMLAttrDef|HTTPHeader|HTTPMethod|domxref|ARIARole|Event)\s*\(\s*['"](?<s>.+?)['"]\s*\)\s*\}\}/`$+{s}`/gis;

  s/\{\{\s*(HTMLElement)\s*\(\s*['"](?<s>.+?)['"]\s*\)\s*\}\}/[`$+{s}`]($+{s}!)/gis;

  s/\{\{\s*(Deprecated_Header)\s*\}\}/# Deprecated/gis;
  s/\{\{\s*(Deprecated_inline)\s*\}\}/*(deprecated)*/gis;

  # hack: truncate bdi doc using indents instead of code fence
  s/^For example,.*?    .*//ms;

  s/^\n+//s;
  s/\n+$/\n/s;
  s/\n\n\n+/\n\n/gms;
EOF
)

# rewrite files
for path in content/files/en-us/web/html/element/*/index.md; do
  perl -0777pe "$reg" "$path" > "$(basename $(dirname "$path")).md"
done

# copy heading elements
h_all="heading_elements.md"
cp "$h_all" "h1.md"
cp "$h_all" "h2.md"
cp "$h_all" "h3.md"
cp "$h_all" "h4.md"
cp "$h_all" "h5.md"
cp "$h_all" "h6.md"

echo "Done processing!"
