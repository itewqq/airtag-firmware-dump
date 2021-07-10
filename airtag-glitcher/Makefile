CC = g++
OPT_LEVEL = -O2
EXTRA_CCFLAGS   = -Wall
CCFLAGS = $(OPT_LEVEL) $(EXTRA_CCFLAGS)
LIBS = -lwiringPi
SRC_FILES = glitcher.c

glitcher: $(SRC_FILES)
	$(CC) $(SRC_FILES) $(CCFLAGS) $(LIBS) -o $@