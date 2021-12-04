CGO_LDFLAGS="-L$(PWD)/../build"

GO="go"
COVERAGE_OUTPUT ?= coverage.out

.PHONY: default vet test

default: vet test

fmt:  ## Run go fmt to format Go files
	$(GO) fmt

test:  ## Run a basic test suite
	CGO_LDFLAGS=$(CGO_LDFLAGS) $(GO) test

cover:  ## Run tests and generate test coverage file, output coverage results and HTML coverage file.
	CGO_LDFLAGS=$(CGO_LDFLAGS) $(GO) test -coverprofile $(COVERAGE_OUTPUT)
	CGO_LDFLAGS=$(CGO_LDFLAGS) $(GO) tool cover -func=$(COVERAGE_OUTPUT)
	CGO_LDFLAGS=$(CGO_LDFLAGS) $(GO) tool cover -html=$(COVERAGE_OUTPUT)
	rm -f $(COVERAGE_OUTPUT)

vet:  ## Go vet all project code
	CGO_LDFLAGS=$(CGO_LDFLAGS) go vet ./...

help:  ## Show This Help
	@for line in $$(cat Makefile | grep "##" | grep -v "grep" | sed  "s/:.*##/:/g" | sed "s/\ /!/g"); do verb=$$(echo $$line | cut -d ":" -f 1); desc=$$(echo $$line | cut -d ":" -f 2 | sed "s/!/\ /g"); printf "%-30s--%s\n" "$$verb" "$$desc"; done

clean:  ## Clean up transient (generated) files
	CGO_LDFLAGS=$(CGO_LDFLAGS) $(GO) clean
	rm -f $(COVERAGE_OUTPUT)
