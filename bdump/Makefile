CC = gcc
CFLAGS = -w
SRCDIR = src
OBJ = bdump.o # print_utils.o # math_functions.o
TARGET = bdump

ifeq ($(OS),Windows_NT)
    RM = del
    RM_FLAGS = /Q
    OBJ_DIR = $(SRCDIR)\*.o
else
    RM = rm -f
    OBJ_DIR = $(SRCDIR)/*.o
endif

all: $(TARGET)

$(TARGET): $(SRCDIR)/$(OBJ)
	$(CC) $^ -o $@ $(CFLAGS)
	$(RM) $(OBJ_DIR) $(RM_FLAGS)

$(SRCDIR)/%.o: $(SRCDIR)/%.c
	$(CC) $(CFLAGS) -c $< -o $@

clean:
	$(RM) $(OBJ_DIR) $(RM_FLAGS)
	$(RM) $(TARGET)
