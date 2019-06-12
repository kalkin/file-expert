Name:           file-expert
Version:        0.9
Release:        1%{?dist}
Summary:        Expert system for recognizing file types.
License:        GPLv3

URL:            https://github.com/kalkin/file-expert

BuildRequires: python3
BuildRequires: pl >= 8.0.2
BuildRequires: python3-pyyaml

%global debug_package %{nil}
%define _builddir %(pwd)
%define _sourcedir %(pwd)

%description
An expert system for recognizing file types, similar to GitHub/linguist, but
written in Prolog.


%prep
%autosetup -T -D -n .


%build
%configure
%make_build


%install
%make_install


%check
%{_rpmconfigdir}/check-rpaths


%files
%{_bindir}/file-expert
%license
%doc


%changelog


