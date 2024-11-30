CC = gcc
CFLAGS = -w
SRCDIR = src
OBJ = bdump.o # print_utils.o # math_functions.o
TARGET = bdump

ifeq ($(OS),Windows_NT)
    RM = del
    RM_FLAGS = /Q
else
    RM = rm -f
endif

all: $(TARGET)
	$(RM) $(SRCDIR)\*.o $(RM_FLAGS)

$(TARGET): $(SRCDIR)/$(OBJ)
	$(CC) $(SRCDIR)/$(OBJ) -o $(TARGET) $(CFLAGS)

$(SRCDIR)/%.o: $(SRCDIR)/%.c
	$(CC) $(CFLAGS) -c $< -o $@

clean:
	$(RM) $(SRCDIR)\*.o $(RM_FLAGS)
	$(RM) $(OBJ) $(TARGET)
