# PKGBUILD
pkgname=rayinteract
pkgver=1.0.0
pkgrel=1
pkgdesc="Interactive UI elements library using Raylib"
arch=('x86_64')
url="" 
license=('MIT')
depends=('raylib')
makedepends=('rust' 'cargo')

build() {
    cd "$srcdir/.."
    cargo build --release
}

package() {
    cd "$srcdir/.."
    
    # Install the Rust library
    install -dm755 "$pkgdir/usr/lib"
    install -Dm644 target/release/libraylib_interactive.rlib \
        "$pkgdir/usr/lib/libraylib_interactive.rlib"

    # Install the header files
    install -dm755 "$pkgdir/usr/include/$pkgname"
    for header in src/*.hpp; do
        install -Dm644 "$header" "$pkgdir/usr/include/$pkgname/$(basename "$header")"
    done

    # Install rayinteractlibs.h
    install -Dm644 src/rayinteractlibs.h "$pkgdir/usr/include/$pkgname/rayinteractlibs.h"

    # Install CMake config file
    install -Dm644 "${srcdir}/rayinteract-config.cmake" \
        "$pkgdir/usr/lib/cmake/rayinteract/rayinteract-config.cmake"


    # Install Cargo.toml and src directory (for reference/debugging)
    install -dm755 "$pkgdir/usr/lib/$pkgname"
    install -Dm644 Cargo.toml "$pkgdir/usr/lib/$pkgname/Cargo.toml"
    cp -r src "$pkgdir/usr/lib/$pkgname/"

    # Install license
    install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}
