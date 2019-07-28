AC_INIT(file-expert, 0.10.1)
AM_INIT_AUTOMAKE([foreign])
AC_CONFIG_MACRO_DIR([m4])

AC_CHECK_PROG(SWIPL, swipl, yes)
if test x"${SWIPL}" != x"yes"; then
    AC_MSG_ERROR([Please install SWI Prolog])
fi

AC_MSG_CHECKING([for SWI Prolog version])
SWIPL_VERSION=$(swipl --version|cut -d" " -f3|cut -d"." -f1)
AC_MSG_RESULT([$SWIPL_VERSION])
if test "${SWIPL_VERSION}" -lt 8; then
    AC_MSG_ERROR([Please install SWI Prolog version ≥ 8])
fi

AC_CHECK_PROG(PYTHON, python3, yes)
if test x"${PYTHON}" != x"yes"; then
    AC_MSG_ERROR([Please install Python 3])
fi

AC_MSG_CHECKING([for PyYAML])
if python3 -c 'import yaml' 2> /dev/null; then
    AC_MSG_RESULT(yes)
else
    AC_MSG_RESULT(no)
    AC_MSG_ERROR([Please install PyYAML])
fi

BKG_ENABLE_XDG
BKG_PACKAGE_INFO(["Expert system for recognizing file types."])
AC_CONFIG_FILES([Makefile file-expert.spec])
AC_OUTPUT

