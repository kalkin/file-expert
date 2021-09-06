AC_INIT([file-expert],[1.0.0~alpha.0])
AC_CANONICAL_TARGET
AM_INIT_AUTOMAKE([foreign])
AC_CONFIG_MACRO_DIR([m4])

BKG_ENABLE_XDG
BKG_PACKAGE_GENTOO(["dev-util"])
BKG_PACKAGE_INFO([Expert system for recognizing file types.])
BKG_RPM

AC_PATH_PROG(CARGO, cargo, "")
if test x"${CARGO}" = x""; then
    AC_MSG_ERROR([Please install cargo])
fi

AC_CONFIG_FILES([Makefile])
AC_OUTPUT

