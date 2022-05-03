AC_INIT([file-expert],[1.0.0])
AC_CANONICAL_TARGET
AM_INIT_AUTOMAKE([foreign])
AC_CONFIG_MACRO_DIR([m4])

BKG_ENABLE_XDG
BKG_PACKAGE_GENTOO(["dev-util"])
BKG_PACKAGE_INFO([Expert system for recognizing source code files])
BKG_RPM

AC_PATH_PROG(CARGO, cargo, "")
if test x"${CARGO}" = x""; then
    AC_MSG_ERROR([Please install cargo])
fi

AC_PATH_PROG(PHP, php, "no")
AC_MSG_CHECKING(YAML support for PHP)
if "${PHP}" -r 'yaml_parse("");' ; then
    AC_MSG_RESULT(yes)
else
    AC_MSG_ERROR([Please install php with yaml support])
fi

AC_CONFIG_FILES([Makefile])
AC_OUTPUT

