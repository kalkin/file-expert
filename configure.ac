AC_INIT([file-expert],[1.0.0~alpha.0])
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

AC_CACHE_CHECK([for php with yaml support], [ac_cv_path_PHP],
AC_PATH_PROGS_FEATURE_CHECK([PHP], [php],
    [[php -r 'yaml_parse("");' \
        && ac_cv_path_PHP="$ac_path_PHP" ac_path_PHP_found=:]],
    [AC_MSG_ERROR([Please install php with yaml support (I'm sorry)])]))
AC_SUBST([PHP], [$ac_cv_path_PHP])

AC_CONFIG_FILES([Makefile])
AC_OUTPUT

