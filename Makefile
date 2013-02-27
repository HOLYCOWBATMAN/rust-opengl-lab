LIB_DIR := lib
OPENGL_DIR := /System/Library/Frameworks/OpenGL.framework/Libraries

RUSTC?=rustc
RUSTFLAGS?=--cfg ndebug --cfg ncpuspew -O
RUSTLDFLAGS?=-L $(LIB_DIR) -L $(OPENGL_DIR)

.PHONY: all
all:	rustlab

FAILUREMSG="If this build failed due to missing SDL bindings, please install them from https://github.com/brson/rust-sdl and copy the .dll/.dylib/.so into this directory or use RUSTLDFLAGS."

RUST_SRC = $(shell find src -type f -name '*.rs')

rustlab: src/main.rs $(RUST_SRC)
	$(RUSTC) $(RUSTFLAGS) $(RUSTLDFLAGS) $< -o $@
	echo "$(FAILUREMSG)"
	touch $@

.PHONY: clean
clean:
	rm -f rustlab
