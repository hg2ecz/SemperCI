INSTALLDIR=$(HOME)/bin
PROG = yalci

# Prerequest:
# apt-get install   cargo make cmake zlib1g-dev libssl-dev

all:
	cargo build --release

clean:
	cargo clean
	rm -f Cargo.lock

install:
	make all
	@echo This software will be installed to $(INSTALLDIR)
	if [ ! -e $(INSTALLDIR) ]; then mkdir $(INSTALLDIR); fi
	cp -p target/release/$(PROG) $(INSTALLDIR)
	strip $(INSTALLDIR)/$(PROG)

uninstall:
	rm -f $(INSTALLDIR)/$(PROG)
