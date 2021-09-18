bin_PROGRAMS = file-expert
file_expert_SOURCES = $(shell find @srcdir@/src -name *.rs) Cargo.toml Cargo.lock

-include ../verbose.mk
-include ../output.mk

CARGO_VERBOSE ::= $(shell test "$(V)" != "1" && echo "-q")

CARGO_TARGET_DIR ?= $(builddir)/target
CARGO_BUILD_FLAGS = $(CARGOFLAGS) $(CARGO_VERBOSE)
CARGO_BUILD_FLAGS += --manifest-path=$(srcdir)/Cargo.toml
CARGO_BUILD_FLAGS += --target-dir=$(CARGO_TARGET_DIR)
CARGO_BUILD_FLAGS += $(if $(CARGO_FAST_BUILD),,--release)
_TARGET_PROFILE_DIR ::= $(CARGO_TARGET_DIR)/$(if $(CARGO_FAST_BUILD),debug,release)

CLEANFILES = file-expert$(EXEEXT)
DISTCLEANFILES = @BKG_RPM_SPECFILE@

# We use the + prefix to pass down the jobserver fds to cargo, but we
# don't use the prefix when make -n is used, so that cargo doesn't run
# in that case)
define RUN_CARGO
$(if $(findstring n,$(filter-out --%, $(MAKEFLAGS))),,+)$(call V_TOOL,CARGO,,$(1)) $(CARGO) $(1) $(CARGO_BUILD_FLAGS)
endef

YAML_FILES = named_patterns.yml languages.yml heuristics.yml skipped.yml

EXTRA_DIST = $(YAML_FILES) @BKG_RPM_SPECFILE@.in
EXTRA_DIST += $(shell find @srcdir@/src -name *.php)

.PHONY: generate-files
generate-files: tests/linguist.rs src/linguist_heuristics.rs
generate-files: src/linguist_extensions.rs src/linguist_interpreters.rs
generate-files: src/linguist_aliases.rs src/linguist_filenames.rs

%.rs: %.rs.php $(YAML_FILES)
	$< > $@
	rustfmt $@

file-expert$(EXEEXT): $(_TARGET_PROFILE_DIR)/file-expert
	$(install_sh) -T -C $< $@

$(_TARGET_PROFILE_DIR)/file-expert: $(SOURCES)
	@$(INFO_BUILD)
	$(call RUN_CARGO,build) --bin $(@F)

mostlyclean-local:
	@$(INFO_CLEAN)
	-$(call RUN_CARGO,clean)

clean-local:
	rm -rf target

.PHONY: check-local
check-local: rust-code-style rust-clippy

.PHONY: rust-clippy
rust-clippy: $(SOURCES)
	$(call RUN_CARGO,clippy) \
		-- -W clippy::all -W clippy::pedantic \
		-A clippy::non_ascii_literal \
		-A clippy::implicit_return \
		-A clippy::expect-used \
		-D warnings

.PHONY: rust-code-style
rust-code-style: $(filter-out Cargo.lock,$(filter-out Cargo.toml,$(SOURCES)))
	$(call V_TOOL,RUSTFMT,,$?)rustfmt $(CARGO_VERBOSE) $^ --check

.PHONY: coverage
coverage: CARGO_BUILD_FLAGS = $(filter-out $(CARGO_VERBOSE),$(CARGOFLAGS))
coverage:
	$(call RUN_CARGO,tarpaulin) -o html

@BKG_RPM_TEMPLATE@
@BKG_DEPLOY_GENTOO@
