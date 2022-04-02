PROGRAM = a.out

OBJS_C := $(patsubst %.c, %.o, $(wildcard *.c))
OBJS_CXX := $(patsubst %.cc, %.o, $(wildcard *.cc))
CFLAGS = -Wall -Wextra -g -O2
CXXFLAGS = -std=c++17 -Wall -Wextra -g -O2

#$(PROGRAM): $(OBJS_C) Makefile
#	$(CC) -o $@ $<

$(PROGRAM): $(OBJS_C) Makefile
	$(CC) -o $@ $<

run: $(PROGRAM)
	./$(PROGRAM)

clean:
	rm -f $(OBJS_C) $(OBJS_CXX) $(PROGRAM)

.PHONY: clean
