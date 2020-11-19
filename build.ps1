cargo build --lib --release 
mkdir $PSScriptRoot/python/afc/bin -ea 0
Copy-Item $PSScriptRoot/target/release/afccapi.dll $PSScriptRoot/python/afc/bin
