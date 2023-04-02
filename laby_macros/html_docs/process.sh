#!/bin/bash
# Processes the MDN HTML element docs for rustdoc inclusion.
set -euxo pipefail

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

  # code blocks get interpreted as rust code by cargo test, so tag them with ignore
  s/```\n(.*?)\n```/```ignore\n$1\n```/gs;

  s/\{\{\s*(HTMLSidebar)\s*\}\}//gis;
  s/\{\{\s*(Deprecated_Header)\s*\}\}/# Deprecated/gis;
  s/\{\{\s*(Deprecated_inline)\s*\}\}/*(deprecated)*/gis;

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
