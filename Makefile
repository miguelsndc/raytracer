CC = gcc
CFLAGS = -Wall -Wextra -std=c17
TARGET = main
SOURCE_DIR = src 
SOURCE_FILES = $(wildcard $(addsuffix /*.c,$(SOURCE_DIR)))

OBJ_FILES = $(SOURCE_FILES:.c=.o)

all: $(TARGET)
$(TARGET): $(OBJ_FILES)
	$(CC) $(CFLAGS) -o $@ $^

%.o: %.c
	$(CC) $(CFLAGS) -c -o $@ $^

clean:
	rm $(OBJ_FILES)


