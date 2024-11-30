void print_operation(const char *op, int destination, bool colors) {
    if (colors) {
        printf("%s%s%s ", ANSI_BLUE, op, ANSI_RESET);
    } else {
        printf("%s ", op);
    }
}

void print_two_reg_args(Instruction *ins, bool colors) {
    if (colors) {
        printf("%s%%r%d%s, ", ANSI_GREEN, ins->destination, ANSI_RESET);
    } else {
        printf("%%r%d, ", ins->destination);
    }

    switch (ins->type) {
    case 0: // register
        if (colors) {
            printf("%s%%r%d%s\n", ANSI_YELLOW, ins->source, ANSI_RESET);
        } else {
            printf("%%r%d\n", ins->source);
        }
        break;
    case 1: // literal
    {
        bool sign = (ins->source >> 7) == 1;
        ins->source &= 0b01111111; // Clear the sign bit
        if (colors) {
            printf("%s#%d%s\n", ANSI_YELLOW, sign ? -ins->source : ins->source, ANSI_RESET);
        } else {
            printf("#%d\n", sign ? -ins->source : ins->source);
        }
    } break;
    case 2: // memory address
    {
        int memaddr = ((ins->source << 1) & 0b1111111) >> 1;
        if (colors) {
            printf("%s&$%d%s\n", ANSI_YELLOW, memaddr, ANSI_RESET);
        } else {
            printf("&$%d\n", memaddr);
        }
    } break;
    case 3: // register indirect
    {
        int reg = ((ins->source << 3) & 0b1111111) >> 3;
        if (colors) {
            printf("%s&r%d%s\n", ANSI_YELLOW, reg, ANSI_RESET);
        } else {
            printf("&r%d\n", reg);
        }
    } break;
    default: fprintf(stderr, "Unknown instruction type\n"); exit(1);
    }
}

void print_jump_instruction(Instruction *ins, bool colors) {
    if (ins->destination == 4) {
        char *sr = to_words(ins->source);
        if (colors) {
            printf("%s@%s%s\n", ANSI_GREEN, sr, ANSI_RESET);
        } else {
            printf("@%s\n", sr);
        }
        free(sr);
    } else {
        ins->destination = (ins->destination << 1) | ins->type;
        ins->destination = (ins->destination << 8) | ins->source;
        ins->source = ins->destination;
        if (colors) {
            printf("%s$%d%s\n", ANSI_YELLOW, ins->source, ANSI_RESET);
        } else {
            printf("$%d\n", ins->source);
        }
    }
}

void print_hlt_instruction(Instruction *ins, bool colors) {
    if (ins->destination == 1) {
        ins->type = (ins->type << 8) | ins->source;
        if (colors) {
            printf("%s.start%s%s $%d%s\n", ANSI_GREEN, ANSI_RESET, ANSI_YELLOW, ins->type,
                   ANSI_RESET);
        } else {
            printf(".start $%d\n", ins->type);
        }
    } else {
        if (colors) {
            printf("%shlt%s\n", ANSI_YELLOW, ANSI_RESET);
        } else {
            printf("hlt\n");
        }
    }
}

void print_output(Instruction *ins) {
    bool colors = args.colors == 1;
    char *op = match_opcode(ins);

    if (args.line_num == 1) {
        print_instruction_header(line, colors);
    }

    if (strcmp(op, "sr") != 0 && strcmp(op, "hlt") != 0) {
        if (in_sr && args.debug == 0 && args.binary == 0) {
            printf("   ");
        }
        print_operation(op, ins->destination, colors);
    }

    bool two_reg_args =
        (strcmp(op, "add") == 0 || strcmp(op, "div") == 0 || strcmp(op, "swp") == 0 ||
         strcmp(op, "cmp") == 0 || strcmp(op, "mul") == 0 || strcmp(op, "mov") == 0);

    if (two_reg_args) {
        print_two_reg_args(ins, colors);
    } else if (strcmp(op, "sr") == 0) {
        in_sr = true;
        char *sr = to_words(ins->source);
        if (colors) {
            printf("%s%s:%s\n", ANSI_GREEN, sr, ANSI_RESET);
        } else {
            printf("%s:\n", sr);
        }
        free(sr);
    } else if (strcmp(op, "jz") == 0 || strcmp(op, "jge") == 0) {
        print_jump_instruction(ins, colors);
    } else if (strcmp(op, "ret") == 0) {
        if (!in_sr && args.verbosity >= 1) {
            if (colors)
                printf(" %s; note: returning outside of a subroutine%s", ANSI_GRAY, ANSI_RESET);
            else
                printf(" ; note: returning outside of a subroutine");
        }
        in_sr = false;
        printf("\n");
    } else if (strcmp(op, "int") == 0) {
        if (colors) {
            printf("%s#%d%s\n", ANSI_YELLOW, ins->source, ANSI_RESET);
        } else {
            printf("#%d\n", ins->source);
        }
    } else if (strcmp(op, "hlt") == 0) {
        print_hlt_instruction(ins, colors);
    } else if (strcmp(op, "ld") == 0) {
        if (colors) {
            printf("%s%%r%d%s, ", ANSI_YELLOW, ins->destination, ANSI_RESET);
        } else {
            printf("%%r%d, ", ins->destination);
        }
        ins->type = (ins->type << 8) | ins->source;
        if (colors) {
            printf("%s$%d%s\n", ANSI_YELLOW, ins->source, ANSI_RESET);
        } else {
            printf("$%d\n", ins->source);
        }
    } else if (strcmp(op, "st") == 0) {
        int reconstructed = (ins->destination << 9) | (ins->type << 8) | ins->source;
        ins->source &= 0x07;
        ins->destination = (reconstructed & 0xFFF8) >> 3;
        if (colors) {
            printf("%s$%d%s, %s%%r%d%s\n", ANSI_YELLOW, ins->destination, ANSI_RESET, ANSI_YELLOW,
                   ins->source, ANSI_RESET);
        } else {
            printf("$%d, %%r%d\n", ins->destination, ins->source);
        }
    } else if (strcmp(op, "set") == 0 || strcmp(op, "cl") == 0) {
        if (colors) {
            printf("%s#%d%s\n", ANSI_YELLOW, ins->source, ANSI_RESET);
        } else {
            printf("#%d\n", ins->source);
        }
    }
    line++;
}
