CC = gcc
CFLAGS = -Wall
SRCDIR = src
OBJ = bdump.o # print_utils.o # math_functions.o
TARGET = bdump

all: $(TARGET)
	rm -f $(SRCDIR)/*.o

$(TARGET): $(SRCDIR)/$(OBJ)
	$(CC) $(SRCDIR)/$(OBJ)-o $(TARGET) $(CFLAGS)

%.o: %.c
	$(CC) $(CFLAGS) -c $< -o $@

clean:
	rm -f $(SRCDIR)/*.o
	rm -f $(OBJ) $(TARGET)
