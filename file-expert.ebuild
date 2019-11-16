EAPI=6

inherit autotools

DESCRIPTION='Expert system for recognizing file types.'
SRC_URI='file-expert-0.10.1.tar.gz'
SLOT='0'
KEYWORDS='amd64 ~x86'
IUSE=''
DEPEND='>=dev-lang/swi-prolog-8.0.0
        >=dev-lang/python-3.5.0
        >=dev-python/pyyaml-5.1'
RESTRICT='fetch'

src_prepare() {
	default
	eautoreconf
}
src_configure() {
	econf
}
src_compile() {
	emake V=1
}
src_install() {
	emake DESTDIR="${D}" install
}


