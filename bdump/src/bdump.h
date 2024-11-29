#ifndef BDUMP_H
#define BDUMP_H

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define CHUNK_SIZE 1024

typedef struct {
    int opcode;
    int destination;
    int source;
    int type; // type 0 is reg, reg
              // type 1 is reg, lit
              // type 2 is reg, mptr
              // type 3 is reg, rptr
    int full_ins;
} Instruction;

typedef struct {
    char *input_file;
    int line_num;
    int colors;
    int verbosity;
    int debug;
    int binary;
} CLI;
CLI args = {0};

CLI parse_arguments(int argc, char *argv[]);
Instruction parse_instruction(int instruction);
void print_binary(int num, int leading);
void print_instruction(Instruction *s);
void print_help(const char *bin);
void print(const char *format, ...);
char *match_opcode(Instruction *s);
int main(int argc, char *argv[]);

#endif
#include "consts.h"
#include "print_helpers.c"
#include "print_utils.c"
