# Maintainer: Your Name <youremail@example.com>
pkgname=catfetch
pkgver=0.1.2
pkgrel=1
pkgdesc="A minimalist CLI cat fetch utility with system info and facts"
arch=('x86_64')
url="https://github.com"
license=('MIT')
depends=('gcc-libs')
makedepends=('cargo')

# Оставляем массив пустым, так как собираем из локальной папки напрямую
source=()
md5sums=()

build() {
  # Переходим в директорию, где лежит стартовый Cargo.toml (корень проекта)
  cd "$startdir"
  cargo build --release --frozen
}

package() {
  cd "$startdir"
  
  # 1. Установка скомпилированного бинарника в системный /usr/bin
  install -Dm755 "target/release/$pkgname" "$pkgdir/usr/bin/$pkgname"
  
  # 2. Установка глобальных ресурсов в /usr/share/catfetch
  install -Dm644 "cats.toml" "$pkgdir/usr/share/$pkgname/cats.toml"
  
  # Копируем всю папку с png-картинками
  mkdir -p "$pkgdir/usr/share/$pkgname/logo"
  cp -r logo/*.png "$pkgdir/usr/share/$pkgname/logo/"
  chmod 644 "$pkgdir/usr/share/$pkgname/logo/"*
}
