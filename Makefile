
SOURCES_CONLY = src/foo.c
SOURCES_BOTH = src/foo2.c

OBJECTS_CONLY = $(SOURCES_CONLY:%.c=%.o)
OBJECTS_BOTH = $(SOURCES_BOTH:%.c=%.o)

default: test-c test-rs

src/foo.o: include/foo.h src/foo.c

test-c: main.c $(OBJECTS_CONLY) $(OBJECTS_BOTH)
	gcc main.c $(OBJECTS_CONLY) $(OBJECTS_BOTH) -o test-c

# realistically this can be unconditional, no dependencies
target/release/libfoo_rs.a: foo-rs/src/* $(OBJECTS_BOTH)
	cargo build --manifest-path Cargo.toml --release
# cargo build --manifest-path foo-rs/Cargo.toml --release

test-rs: main.c $(OBJECTS_BOTH) target/release/libfoo_rs.a
	gcc main.c $(OBJECTS_BOTH) target/release/libfoo_rs.a -o test-rs

.PHONY:
clean:
	rm -f src/*.o
	rm -f test-c test-rs
	cargo clean --manifest-path foo-rs/Cargo.toml

# Another option: build object files for each rust src file:
#	rustc --emit=obj --crate-type=staticlib <filename>