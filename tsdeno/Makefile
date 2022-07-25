SOURCE_DIR  = src
MAIN        = $(SOURCE_DIR)/index.ts
ARGS        = 
PERMISSIONS = --allow-net \
              --allow-read \
			  --allow-env \
			  --allow-write

run: 
	deno run $(PERMISSIONS) $(MAIN) $(ARGS)

fmt: 
	deno fmt
cache:
	deno cache $(MAIN)

.PHONY: run fmt