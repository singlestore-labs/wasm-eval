# Based on:
# https://philpep.org/blog/a-makefile-for-your-dockerfiles/

VERSION = 1.0
REGISTRY?=psy3.memcompute.com/launch-pad
DOCKERFILES=$(shell find * -type f -name Dockerfile)
NAMES=$(subst /,\:,$(subst /Dockerfile,,$(DOCKERFILES)))
IMAGES=$(addprefix $(subst :,\:,$(REGISTRY))/,$(NAMES))
DEPENDS=.depends.mk
MAKEFLAGS += -rR

.PHONY: all clean push pull run exec check checkrebuild lint pull-base ci $(NAMES) $(IMAGES)

all: $(IMAGES)
clean:
	rm -f $(DEPENDS)

pull-base:
	# used by base-wasm
	docker pull $(REGISTRY)/debian:buster

ci:
	$(MAKE) pull-base checkrebuild lint push all

.PHONY: $(DEPENDS)
$(DEPENDS): $(DOCKERFILES)
	grep '^FROM \$$REGISTRY/' $(DOCKERFILES) | \
		awk -F '/Dockerfile:FROM \\$$REGISTRY/' '{ print $$1 " " $$2 }' | \
		sed 's@[:/]@\\:@g' | awk '{ print "$(subst :,\\:,$(REGISTRY))/" $$1 ": " "$(subst :,\\:,$(REGISTRY))/" $$2 }' > $@

sinclude $(DEPENDS)

$(NAMES): %: $(REGISTRY)/%
ifeq (push,$(filter push,$(MAKECMDGOALS)))
	docker push $<
endif
ifeq (run,$(filter run,$(MAKECMDGOALS)))
	docker run --rm -it $<
endif
ifeq (exec,$(filter exec,$(MAKECMDGOALS)))
	docker run --entrypoint sh --rm -it $<
endif
ifeq (check,$(filter check,$(MAKECMDGOALS)))
	./check_update.sh $(REGISTRY)/$<
endif

$(IMAGES): %:
ifeq (pull,$(filter pull,$(MAKECMDGOALS)))
	docker pull $@
else
	docker build --build-arg REGISTRY=$(REGISTRY) -t $@ $(subst :,/,$(subst $(REGISTRY)/,,$@))
endif
ifeq (checkrebuild,$(filter checkrebuild,$(MAKECMDGOALS)))
	./check_update.sh $(REGISTRY)/$@ || \
	(docker build --no-cache -t $(REGISTRY)/$@ $(subst :,/,$@) && ./check_update.sh $(REGISTRY)/$@)
endif

$(NAMES): %:
ifeq (lint,$(filter lint,$(MAKECMDGOALS)))
	docker run --rm -i psy3.memcompute.com/launch-pad/hadolint:v2.4 < $@/Dockerfile
endif


help:
	@echo "Makefile for building dockerfiles"
	@echo ""
	@echo "Read all Dockerfile within the current directory and generate dependencies automatically."
	@echo ""
	@echo "make all              ; build all images"
	@echo "make wasmer           ; build wasmer image"
	@echo "make push all         ; build and push all images"
	@echo "make push wasmer      ; build and push wasmer image"
	@echo "make run wasmer       ; build and run wasmer image (for testing)"
	@echo "make exec wasmer      ; build and start interactive shell in wasmer image (for debugging)"
	@echo "make pull-base        ; pull base images from docker hub used to bootstrap other images"
	@echo "make ci               ; alias to make pull-base checkrebuild push all"
