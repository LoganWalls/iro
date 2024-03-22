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
  if [ "$lang" = "tsx" ]; then
    dir="$TS_GRAMMARS/tree-sitter-typescript"
  elif [ "$lang" = "markdown" ]; then
    dir="$TS_GRAMMARS/tree-sitter-markdown/tree-sitter-markdown"
  fi
  out="$lang"
  src="$out-src"
  if [ ! -d "$out" ]; then
    mkdir "$out"
    echo "Building $out"
    cp -LR "$dir" "./$src"
    chmod --recursive +rw "$src"
    root="$src"
    if [[ $lang == "ocaml" ]]; then
      root="$root/ocaml"
    elif [[ $lang == "typescript" ]]; then
      root="$root/typescript"
    elif [[ $lang == "tsx" ]]; then
      root="$root/tsx"
    fi
    # NOTE: At time of writing the nixpkgs tree-sitter
    # is out of date. I am using an impure install of
    # tree-sitter 0.22.2 with nix-provided emscripten
    tree-sitter build --wasm "$root" -o "$out/grammar.wasm"

    if [[ $lang == "scala" ]]; then
      cp "$src/queries/scala/highlights.scm" "$out/"
    else
      cp "$src/queries/highlights.scm" "$out/"
    fi
    rm -rf "$src"
  fi
done

rm -rf "$EM_CACHE"
rm -f ./a.*
