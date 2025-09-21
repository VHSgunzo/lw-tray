# Maintainer: VHSgunzo <vhsgunzo.github.io>

pkgname='lw-tray'
pkgver='0.0.8'
pkgrel='1'
pkgdesc='Lux Wine tray'
arch=('x86_64')
url='https://github.com/VHSgunzo/lw-tray'
license=('MIT')
depends=('lwrap')
source=("$pkgname" "${pkgname}-go")
sha256sums=('SKIP' 'SKIP')

package() {
    install -Dm755 "$pkgname" "$pkgdir/opt/lwrap/bin/$pkgname"
    install -Dm755 "${pkgname}-go" "$pkgdir/opt/lwrap/bin/${pkgname}-go/$pkgname"
}
