#!/usr/bin/env bash
set -euo pipefail

mkdir -p .emscriptencache
EM_CACHE=$(pwd)/.emscriptencache
export EM_CACHE

declare -a languages=(
  "bash"
  "c"
  "c-sharp"
  "clojure"
  "commonlisp"
  "cpp"
  "css"
  "elixir"
  "go"
  "html"
  "java"
  "javascript"
  "json"
  "lua"
  "markdown"
  "nix"
  "ocaml"
  "python"
  "rust"
  "scala"
  "sql"
  "tsx"
  "typescript"
  "zig"
)

# NOTE: $TS_GRAMMARS is a directory that contains all of the source
# code for the grammars. These are provided by nix, and the env
# variable is set in `flake.nix`
for lang in "${languages[@]}"; do
  dir="$TS_GRAMMARS/tree-sitter-${lang}"
  grammar="${dir##*/}"
  if [ "$lang" = "tsx" ]; then
    dir="$TS_GRAMMARS/tree-sitter-typescript"
    grammar="tree-sitter-tsx"
  fi
  out="${grammar}.wasm"
  if [ ! -f "$out" ]; then
    echo "Building $grammar"
    cp -LR "$dir" "./$grammar"
    chmod --recursive +rw "$grammar"
    target="$grammar"
    if [ "$lang" = "markdown" ]; then
      target="$target/tree-sitter-markdown" .
    elif [[ $lang == "ocaml" ]]; then
      target="$target/ocaml"
    elif [[ $lang == "typescript" ]]; then
      target="$target/typescript"
    elif [[ $lang == "tsx" ]]; then
      target="$target/tsx"
    fi
    # NOTE: At time of writing the nixpkgs tree-sitter
    # is out of date. I am using an impure install of
    # tree-sitter 0.22.2 with nix-provided emscripten
    tree-sitter build --wasm "$target" -o "$out"
    rm -rf "$grammar"
  fi
done

rm -rf "$EM_CACHE"
rm -f ./a.*
