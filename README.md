# Run code

default features: no extra requirements

tested toolchains:

- stable-x86_64-pc-windows-gnu
- stable-x86_64-pc-windows-msvc

## Features

### fishing

#### Installations

- only available for stable-x86_64-pc-windows-msvc target
- needs visual studio buildchain (download visual studio installer and install MSVC v142 (or newer) - VS 2019 C+ -x64/x86-Buildtools)
- needs vcpkg (clone from github)
- needs tesseract (install using vcpkg)
- needs opencv (install using vcpkg)

#### Environment Variables

SET VCPKG_ROOT=D:\gloria-victis-bot\tessdata

SET VCPKG_DEFAULT_TRIPLET=x64-windows
SET VCPKGRS_DYNAMIC=true

## Notes
The package tesseract:x64-windows provides CMake targets:

    find_package(Tesseract CONFIG REQUIRED)
    target_link_libraries(main PRIVATE libtesseract)
