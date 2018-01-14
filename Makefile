BUILD_PATH = ./bin
OBJECT = $(BUILD_PATH)/plugin.o
BINS = $(BUILD_PATH)/template.so

RUST_BUILD = cargo build
RUST_BUILD_FLAGS = --release
RUST_LIB = ./target/release/libtemplate.dylib

C_BUILD_FLAGS = -O0 -g -std=c++11 -Wall
C_SRC = ./plugin/plugin.cpp
C_LINK = -shared -fPIC -framework Carbon -framework Cocoa -framework ApplicationServices

all: $(BINS)

install: C_BUILD_FLAGS=-O2 -std=c++11 -Wall
install: clean $(BINS)

.PHONY: all clean install

clean:
	rm -f $(OBJECT)
	rm -f $(BINS)

$(BUILD_PATH)/template.so: $(C_SRC) | $(BUILD_PATH)
	$(RUST_BUILD) $(RUST_BUILD_FLAGS)
	clang++ -o $(OBJECT) -c $^
	clang++ $(BUILD_FLAGS) -o $(BINS) $(OBJECT) $(RUST_LIB) $(C_LINK)
