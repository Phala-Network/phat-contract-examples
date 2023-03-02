PROGRAMS = \
	httpclient \
	httpserver \
	recv_messages \
	take-a-rest \
	tcpclient \
	tcpserver \
	telebot \
	timer \
	disbot \

.PHONY: install clean

install: $(PROGRAMS:%=install-%)


install-%:
	make install -C $* -f ../build.mk PREFIX=`realpath $(PREFIX)`/sideprogs

clean: $(PROGRAMS:%=clean-%)

clean-%:
	make clean -C $* -f ../build.mk