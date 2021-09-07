bin_PROGRAMS = file-expert
file_expert_SOURCES = $(shell find @srcdir@/src -name *.rs) Cargo.toml Cargo.lock

CARGO_VERBOSE := $(shell test "$(V)" != "1" && echo "-q")
CARGO_BUILD_FLAGS = $(CARGOFLAGS) $(CARGO_VERBOSE)
CARGO_BUILD_FLAGS += --manifest-path=$(srcdir)/Cargo.toml
CARGO_BUILD_FLAGS += --target-dir=$(builddir)/target
CARGO_BUILD_FLAGS += --release

CLEANFILES = file-expert$(EXEEXT)
DISTCLEANFILES = @BKG_RPM_SPECFILE@

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

EXTRA_DIST = named_patterns.yml languages.yml heuristics.yml @BKG_RPM_SPECFILE@.in

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

file-expert$(EXEEXT): $(builddir)/target/release/file-expert
	$(install_sh) -T -C $< $@

$(builddir)/target/release/file-expert: $(SOURCES)
	$(call CARGO_BUILD) --bin $(@F)

mostlyclean-local:
	$(call RUN_CARGO,clean)

clean-local:
	rm -rf target

.PHONY: check-local
check-local: rust-code-style rust-clippy

.PHONY: rust-clippy
rust-clippy: $(shell git ls-files -- "*.rs" "*.toml")
	$(INFO_CHECK)
	$(call RUN_CARGO,clippy) \
		-- -W clippy::all -W clippy::pedantic \
		-A clippy::non_ascii_literal \
		-A clippy::implicit_return \
		-A clippy::expect-used \
		-D warnings

.PHONY: rust-code-style
rust-code-style: $(filter-out Cargo.lock,$(filter-out Cargo.toml,$(SOURCES)))
	@$(INFO_CHECK)
	rustfmt $(CARGO_VERBOSE) $^ --check

@BKG_RPM_TEMPLATE@
@BKG_DEPLOY_GENTOO@
