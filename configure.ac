AC_INIT(file-expert, 0.9)
AM_INIT_AUTOMAKE([foreign])
AC_CONFIG_MACRO_DIR([m4])

AC_CHECK_PROG(SWIPL, swipl, yes)
if test x"${SWIPL}" != x"yes"; then
    AC_MSG_ERROR([Please install SWI Prolog])
fi

AC_CHECK_PROG(PYTHON, python3, yes)
if test x"${PYTHON}" != x"yes"; then
    AC_MSG_ERROR([Please install Python 3])
fi

BKG_ENABLE_XDG
BKG_PACKAGE_INFO(["Expert system for recognizing different files."])
AC_CONFIG_FILES([Makefile])
AC_OUTPUT

