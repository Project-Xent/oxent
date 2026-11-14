param([ValidateSet("package", "publish")][string]$Command = "package")
$ErrorActionPreference = "Stop"
Push-Location $PSScriptRoot
try {
    $dist = if ($env:XENT_DIST) { $env:XENT_DIST } else { "..\fluxent\build\dist" }
    $version = (Select-String -Path Cargo.toml -Pattern '^version = "(.*)"').Matches[0].Groups[1].Value
    foreach ($pair in @(@("x86_64", "x86_64"), @("aarch64", "arm64"))) {
        $dir = "oxent-sys\native\$($pair[0])"
        New-Item -ItemType Directory -Force $dir | Out-Null
        Copy-Item "$dist\xent-$version-$($pair[1])-windows-gnu\lib\libxent.a" $dir
    }
    cargo $Command -p oxent-sys --allow-dirty
    if ($LASTEXITCODE) { exit $LASTEXITCODE }
} finally {
    Pop-Location
}
