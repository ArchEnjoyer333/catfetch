# Maintainer: ArchEnjoyer333 <https://github.com/ArchEnjoyer333>
pkgname=catfetch
pkgver=0.1.0
pkgrel=1
pkgdesc="A minimalist CLI cat fetch utility with system info and random facts"
arch=('x86_64')
url="https://github.com/ArchEnjoyer333/catfetch"
license=('MIT')
depends=('gcc-libs')
makedepends=('cargo' 'git')

# Собираем напрямую из мастера твоего репозитория
source=("git+https://github.com")
sha256sums=('SKIP')

build() {
  cd "$srcdir/$pkgname"
  cargo build --release --frozen
}

package() {
  cd "$srcdir/$pkgname"
  
  # Установка бинарника
  install -Dm755 "target/release/$pkgname" "$pkgdir/usr/bin/$pkgname"
  
  # Установка ресурсов
  install -Dm644 "cats.toml" "$pkgdir/usr/share/$pkgname/cats.toml"
  mkdir -p "$pkgdir/usr/share/$pkgname/logo"
  cp -r logo/*.png "$pkgdir/usr/share/$pkgname/logo/"
  chmod 644 "$pkgdir/usr/share/$pkgname/logo/"*
}
