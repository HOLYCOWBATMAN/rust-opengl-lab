LIB_DIR := lib
OPENGL_DIR := /System/Library/Frameworks/OpenGL.framework/Libraries

RUSTC?=rustc
# RUSTFLAGS?=--cfg ndebug --cfg ncpuspew -O
RUSTFLAGS?=--cfg debug --cfg ncpuspew -O
RUSTLDFLAGS?=-L $(LIB_DIR) -L $(OPENGL_DIR)

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

.PHONY: clean
clean:
	rm -f rustlab

run: rustlab
	./rustlab
