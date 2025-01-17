#!/bin/bash

# Define bindings directories
BINDINGS_DIRS=(
  "src/bindings/core"
  "src/bindings/devops"
  "src/bindings/requests"
  "src/bindings/responses"
)

# Function to generate index files
generate_index() {
  local path="$1"

  # Check if directory exists, if not run cargo test which generates the bindings.
  if [ ! -d "$path" ]; then
    cargo test --manifest-path src-tauri/Cargo.toml
  fi

  # Get list of exports
  local exports=()
  while IFS= read -r -d '' file; do
    local filename=$(basename -- "$file")
    local extension="${filename##*.}"
    local basename="${filename%.*}"
    if [ "$basename" != "index" ] && [ "$extension" = "ts" ]; then
      exports+=("export * from \"./$basename\";")
    fi
  done < <(find "$path" -type f -name "*.ts" -print0)

  # Write exports to index file
  printf '%s\n' "${exports[@]}" > "${path}/index.ts"
}

# Loop through bindings directories and generate index files
for dir in "${BINDINGS_DIRS[@]}"; do
  generate_index "$dir"
done
