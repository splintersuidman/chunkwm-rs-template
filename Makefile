BUILD_PATH = ./bin
BIN = $(BUILD_PATH)/template.so
LIB = ./target/release/libtemplate.dylib

install: clean $(BUILD_PATH)
	cargo build --release
	cp $(LIB) $(BIN)

clean:
	rm -f $(BIN)

$(BUILD_PATH):
	mkdir -p $(BUILD_PATH)
