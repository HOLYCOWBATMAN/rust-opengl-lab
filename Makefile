LIB_DIR := lib
OPENGL_DIR := /System/Library/Frameworks/OpenGL.framework/Libraries

RUSTC?=rustc
RUSTFLAGS?=--cfg debug --cfg ncpuspew -O
RUSTLDFLAGS?=-L $(LIB_DIR) -L $(OPENGL_DIR)

GLCORE_BASE_DIR?=submodules/glcore-rs
GLCORE_LIB_DIR?=$(GLCORE_BASE_DIR)/lib

LMATH_BASE_DIR?=submodules/lmath-rs
LMATH_LIB_DIR?=$(LMATH_BASE_DIR)/lib
LMATH_EXT_DIR?=$(LMATH_BASE_DIR)/extern

NUMERIC_BASE_DIR?=submodules/numeric-rs
NUMERIC_LIB_DIR?=$(NUMERIC_BASE_DIR)/src

GLFW_BASE_DIR?=submodules/glfw-rs
GLFW_LIB_DIR?=$(GLFW_BASE_DIR)/lib

STB_IMAGE_BASE_DIR?=submodules/rust-stb-image
STB_IMAGE_LIB_DIR?=$(STB_IMAGE_BASE_DIR)

.PHONY: all
all:	rustlab

RUST_SRC = $(shell find src -type f -name '*.rs')

rustlab: src/main.rs $(RUST_SRC)
	$(RUSTC) $(RUSTFLAGS) $(RUSTLDFLAGS) $< -o $@
	touch $@

unittest: src/main.rs $(RUST_SRC)
	$(RUSTC) --test $(RUSTFLAGS) $(RUSTLDFLAGS) $< -o $@
	touch $@
	./unittest

.PHONY: all-lib
all-lib:	lib-glcore-rs lib-lmath-rs lib-glfw-rs lib-rust-stb-image

.PHONY: clean-all-lib
clean-all-lib:
	rm -f $(LIB_DIR)/lib*.dylib
	rm -f $(LIB_DIR)/lib*.a
	cd $(GLCORE_BASE_DIR) && $(MAKE) clean
	rm -f $(NUMERIC_LIB_DIR)/lib*.dylib
	rm -rf $(NUMERIC_LIB_DIR)/lib*.dylib.dSYM
	cd $(LMATH_BASE_DIR) && $(MAKE) clean
	cd $(GLFW_BASE_DIR) && $(MAKE) clean
	cd $(STB_IMAGE_BASE_DIR) && $(MAKE) clean

.PHONY: lib-glcore-rs
lib-glcore-rs:
	cd $(GLCORE_BASE_DIR) && $(MAKE) osx-lion
	cp $(GLCORE_LIB_DIR)/lib*.dylib $(LIB_DIR)

.PHONY: lib-numeric-rs
lib-numeric-rs:
	cd $(NUMERIC_BASE_DIR) && rust build --lib src/numeric.rs
	cp $(NUMERIC_LIB_DIR)/lib*.dylib $(LIB_DIR)
	cp $(NUMERIC_LIB_DIR)/lib*.dylib $(LMATH_EXT_DIR)

.PHONY: lib-lmath-rs
lib-lmath-rs:	lib-numeric-rs
	cd $(LMATH_BASE_DIR) && $(MAKE)
	cp $(LMATH_LIB_DIR)/lib*.dylib $(LIB_DIR)

.PHONY: lib-glfw-rs
lib-glfw-rs:
	cd $(GLFW_BASE_DIR) && $(MAKE)
	cp $(GLFW_LIB_DIR)/lib*.dylib $(LIB_DIR)

.PHONY: lib-rust-stb-image
lib-rust-stb-image:
	cd $(STB_IMAGE_BASE_DIR) && ./configure && $(MAKE)
	cp $(STB_IMAGE_LIB_DIR)/lib*.dylib $(LIB_DIR)
	cp $(STB_IMAGE_LIB_DIR)/lib*.a $(LIB_DIR)

.PHONY: clean
clean:
	rm -f rustlab

run: rustlab
	./rustlab
