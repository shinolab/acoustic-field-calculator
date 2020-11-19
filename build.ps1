function ColorEcho($color, $PREFIX, $message) {
    Write-Host $PREFIX -ForegroundColor $color -NoNewline
    Write-Host ":", $message
}

function GetVersion([string]$path) {
    $file = New-Object System.IO.StreamReader($path, [System.Text.Encoding]::GetEncoding("utf-8"))
    $version = ""
    while (($line = $file.ReadLine()) -ne $null) {
        $pat = "^version = ""(.*)"""
        $regex = [regex]$pat
        $regex.Matches($line) | foreach {
            $version = $_.Groups[1].Value
        }
    }
    $file.Close()

    if ($version -eq "") {
        ColorEcho "Red" "Error" "Failed to load version number."
        $host.UI.RawUI.ReadKey() | Out-Null
        Exit
    }
    return $version
}

$version = GetVersion("$PSScriptRoot/acoustic-field-calculator/Cargo.toml")
cargo build --lib --release 
mkdir $PSScriptRoot/python/afc/bin -ea 0
Copy-Item $PSScriptRoot/target/release/afccapi.dll $PSScriptRoot/python/afc/bin/afccapi-$version.dll
