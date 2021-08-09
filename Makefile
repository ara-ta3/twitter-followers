CARGO=cargo

build: .env
	$(CARGO) build

run: .env
	set -o allexport && . ./$< && $(CARGO) run

.env: env.sample
	cp -f $< $@
