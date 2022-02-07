ifneq (,)
This makefile requires GNU Make.
endif

PROGRAM = a.out

C_FILES := $(shell find . -not -path "*/build/*" -a -name "*.c")
OBJS := $(patsubst %.c, %.o, $(C_FILES))
CFLAGS = -Wall -Wextra -g -O2
CPPFLAGS =
LDFLAGS =
LDLIBS =

#CPPFLAGS += -I/home/zfazek/git/seasocks/src/main/c
#LDLIBS += /home/zfazek/git/seasocks/build/src/main/c/libseasocks.a

#CFLAGS += $(shell pkg-config --cflags zlib)
#LDLIBS += $(shell pkg-config --libs zlib)

$(PROGRAM): .depend $(OBJS)
	$(CC) $(CFLAGS) $(OBJS) $(LDFLAGS) -o $(PROGRAM) $(LDLIBS)

depend: .depend

.depend: cmd = $(CC) $(CPPFLAGS) -MM -MF depend $(var); cat depend >> .depend;
.depend:
	@echo "Generating dependencies..."
	@$(foreach var, $(C_FILES), $(cmd))
	@rm -f depend

-include .depend

%.o: %.c Makefile
	$(CC) $(CFLAGS) $(CPPFLAGS) -c $< -o $@

%: %.c
	$(CC) $(CXXFLAGS) -o $@ $<

run: $(PROGRAM)
	./$(PROGRAM)

clean:
	rm -f .depend $(OBJS)

.PHONY: clean depend
