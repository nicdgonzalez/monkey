# To reference the current Makefile, this must be placed before any `include`.
THIS_DIR := $(dir $(abspath $(lastword $(MAKEFILE_LIST))))

CXX_FILES := $(shell find . -type f -name "*.cc" -o -name "*.hh" \
	     -o -name "*.hpp" -o -name "*.cpp" -o -name "*.h" \
	     -not -path "./build")

SOURCE_FILES := $(filter %.cc %.cpp, $(CXX_FILES))
HEADER_FILES := $(filter %.h %.hh %.hpp, $(CXX_FILES))

.PHONY: all fmt build install uninstall test

all: fmt build test

fmt: $(CXX_FILES)
	@clang-format -i $(CXX_FILES)

build: fmt $(SOURCE_FILES)
	@cmake -B build -S . && make -C build

install: build
	@ln -sf '$(THIS_DIR)build/monkey' '/usr/bin/monkey'

uninstall:
	@unlink '/usr/bin/monkey'

test: build
	@make -C build/tests
	$(THIS_DIR)build/tests/monkey_test
