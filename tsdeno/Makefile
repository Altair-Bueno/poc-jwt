SOURCE_DIR  = src
MAIN        = $(SOURCE_DIR)/index.ts
ARGS        = 
PERMISSIONS = --allow-net \
              --allow-read

run: 
	deno run $(PERMISSIONS) $(MAIN) $(ARGS)

fmt: 
	deno fmt

.PHONY: run fmt