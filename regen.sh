#!/usr/bin/env bash
set -euo pipefail
root="$(cd "$(dirname "$0")" && pwd)"
here="$root/oxent-sys"
xent="$root/.."
mingw="$(dirname "$(dirname "$(command -v x86_64-w64-mingw32-clang)")")"
stub="$(mktemp -d)"
trap 'rm -rf "$stub"' EXIT
for h in x86intrin.h immintrin.h x86gprintrin.h emmintrin.h xmmintrin.h pmmintrin.h mmintrin.h ammintrin.h; do : > "$stub/$h"; done
: "${LIBCLANG_PATH:?point LIBCLANG_PATH at a directory containing libclang.dll}"
enums="$(cd "$xent/xent-kit" && xmake lua tools/gen_bindings.lua enum-types)"
bindgen "$here/wrapper.h" -o "$here/src/bindings.rs" \
  --allowlist-function 'xtk_(scene|txn|ctrl|catalog|batchresult)_.*' \
  --allowlist-function 'flux_app_(create|destroy|run|redraw|onframe|invoke|attachhost)|flux_host_destroy' \
  --allowlist-var 'XTK_.*|XENT_FLEX_.*|FLUX_BACKDROP_.*|FLUX_THEME_.*' --allowlist-type 'XtkEvent|XtkEventPayload|XtkValueView|XtkBatchDiag|XtkCtrlType|XtkStandardPropertyId|XtkStandardEventId|XtkStandardResourceTypeId|XtkEventPhaseMask|XtkEventDisposition|XtkStackDirection|'"$enums" \
  --no-layout-tests --use-core --default-enum-style consts --no-prepend-enum-name \
  -- -std=c17 --target=x86_64-w64-windows-gnu \
  -I"$xent/xent-kit/include" -I"$xent/xent-core/include" -I"$xent/fluxent/include" -I"$stub" \
  -isystem "$mingw/x86_64-w64-mingw32/include" -isystem "$(ls -d "$mingw"/lib/clang/*/include | head -1)"
(cd "$xent/xent-kit" && xmake gen-bindings rust "$root/oxent/src/controls.rs")
(cd "$root" && cargo fmt --all)
