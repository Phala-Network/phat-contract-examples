PROGRAMS = \
	disbot \
	httpclient \
	httpserver \
	recv_messages \
	take-a-rest \
	tcpclient \
	tcpserver \
	telebot \
	timer \

.PHONY: install clean

install: $(PROGRAMS:%=install-%)


install-%:
	make install -C $* -f ../build.mk

clean: $(PROGRAMS:%=clean-%)

clean-%:
	make clean -C $* -f ../build.mk