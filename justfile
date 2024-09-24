set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]

build-win:
    pwsh -File ./scripts/pre-build-win.ps1
    cargo build --release
install-win:
    pwsh -File ./scripts/pre-build-win.ps1
    cargo build --release
    Copy-Item -Path {{ justfile_directory() }}\target\release\smrec.exe -Destination {{ env_var_or_default("USERPROFILE", "") }}\.cargo\bin\

prepare-linux:
    ./scripts/pre-build-linux.sh
