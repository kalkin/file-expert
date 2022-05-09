Name:           file-expert
Version:        1.1.0
Release:        1%{?dist}
Summary:        Expert system for recognizing source code files
License:        GPLv3

URL:            https://github.com/kalkin/file-expert

BuildRequires: cargo clippy rustfmt autoconf automake make
BuildRequires: php php-pecl-yaml

%global debug_package %{nil}
%define _builddir %(pwd)
%define _sourcedir %(pwd)

%description
Expert system for recognizing source code files, similar to GitHub/linguist, but
written in Rust.


%prep
%autosetup -T -D -n .


%build
autoreconf -i
%configure
%make_build


%install
%make_install


%check


%files
%{_bindir}/file-expert
%license
%doc


%changelog
* Sun Jul 28 2019 Bahtiar Gadimov (kalkin-) <bahtiar@gadimov.de>
  - Fix recognition for RPM Spec
  - Add runtime requirement for swipl

* Wed Jul 17 2019 Bahtiar Gadimov (kalkin-) <bahtiar@gadimov.de>
  Rework the file guessing logic

