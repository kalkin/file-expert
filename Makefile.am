bin_PROGRAMS = file-expert
file_expert_SOURCES = $(shell find @srcdir@/src -name *.rs)

CARGO_BUILD_FLAGS = $(CARGOFLAGS)
CARGO_BUILD_FLAGS += --manifest-path=$(srcdir)/Cargo.toml
CARGO_BUILD_FLAGS += --target-dir=$(builddir)/target
CARGO_BUILD_FLAGS += --release

# We use the + prefix to pass down the jobserver fds to cargo, but we
# don't use the prefix when make -n is used, so that cargo doesn't run
# in that case)
define RUN_CARGO
$(if $(findstring n,$(filter-out --%, $(MAKEFLAGS))),,+)$(CARGO) $(1) $(CARGO_BUILD_FLAGS)
endef

# This function is intended to be called by:
#
#   $(call CARGO_BUILD,EXTRA_ENV_VAR1=X EXTRA_ENV_VAR2=Y ...)
#
# but, given the idiosyncracies of make, can also be called without arguments:
#
#   $(call CARGO_BUILD)
define CARGO_BUILD
$(call RUN_CARGO,build)
endef

EXTRA_DIST = Cargo.toml Cargo.lock named_patterns.yml languages.yml heuristics.yml @BKG_RPM_SPECFILE@.in
CLEANFILES = @BKG_RPM_SPECFILE@

generate-files: tests/linguist.rs src/linguist_heuristics.rs
generate-files: src/linguist_extensions.rs src/linguist_interpreters.rs
generate-files: src/linguist_aliases.rs src/linguist_filenames.rs

%.rs: %.rs.php
	$< > $@
	rustfmt $@

src/linguist_heuristics.rs: src/linguist_heuristics.rs.php disambiguations.yml named_patterns.yml
	$< > $@
	rustfmt $@

src/linguist_extensions.rs: src/linguist_extensions.rs.php languages.yml
	$< > $@
	rustfmt $@

src/linguist_interpreters.rs: src/linguist_interpreters.rs.php languages.yml
	$< > $@
	rustfmt $@

src/linguist_aliases.rs: src/linguist_aliases.rs.php languages.yml
	$< > $@
	rustfmt $@

src/linguist_filenames.rs: src/linguist_filenames.rs.php languages.yml
	$< > $@
	rustfmt $@

tests/linguist.rs: tests/linguist.rs.php skipped.yml
	$< > $@
	rustfmt $@

file-expert$(EXEEXT): $(SOURCES)
	$(call CARGO_BUILD) --bin $@
	mv $(builddir)/target/release/$(@F) $@

mostlyclean-local:
	$(call RUN_CARGO,clean)
	rm -fv $(builddir)/file-expert$(EXEEXT)

clean-local:
	rm -rf target


@BKG_RPM_TEMPLATE@
@BKG_DEPLOY_GENTOO@
