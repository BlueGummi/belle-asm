#include "bdump.h"

bool in_sr = false;
int line = 1;
void print_binary(int num, int leading) {
    if (args.binary == 1) {
        printf("\n");
        for (int i = leading - 1; i >= 0; i--) {
            printf("%d", (num >> i) & 1);
        }

        printf("\n");
    }
}

void print_help(const char *bin) { // bin is the name of the binary
    printf("The disassembler for %sBELLE%s\n\n", ANSI_BOLD, ANSI_RESET);
    printf("%sUsage:%s %s [OPTIONS] <FILE>\n\n", ANSI_UNDERLINE, ANSI_RESET, bin);
    printf("%sArguments:%s\n", ANSI_UNDERLINE, ANSI_RESET);
    printf("  <FILE> Path to input\n\n");
    printf("%sOptions:%s\n", ANSI_UNDERLINE, ANSI_RESET);
    printf("  -h, --help       Show this help message and exit\n");
    printf("  -l, --line-num   Enable line numbering\n");
    printf("  -b, --binary     Print binary\n");
    printf("  -c, --colors     Enable colored output\n");
    printf("  -d, --debug      Print debug messages\n");
    printf("  -v, --verbose    Increase verbosity level (use multiple for more)\n");
    exit(0);
}

void print(const char *format, ...) {
    if (args.debug == 1) {
        printf(ANSI_GREEN "DEBUG: " ANSI_RESET);
        va_list arguments;
        va_start(arguments, format);
        vprintf(format, arguments);
        va_end(arguments);
    }
}

void print_output(Instruction *ins) {
    bool colors = args.colors == 1;
    char *op = match_opcode(ins);
    if (args.line_num == 1) {
        if (colors)
            printf("%sline %*d:%s ", ANSI_RED, 3, line, ANSI_RESET);
        else
            printf("line %*d: ", 3, line);
    }
    if (strcmp(op, "sr") != 0 && strcmp(op, "hlt") != 0) {
        if (in_sr && args.debug == 0 && args.binary == 0)
            printf("   ");
        if (colors)
            printf("%s%s%s ", ANSI_BLUE, op, ANSI_RESET);
        else
            printf("%s ", op);
    }
    int memaddr, reg;
    if (strcmp(op, "ret") == 0)
        in_sr = false;
    bool two_reg_args =
        (strcmp(op, "add") == 0 || strcmp(op, "div") == 0 || strcmp(op, "swp") == 0 ||
         strcmp(op, "cmp") == 0 || strcmp(op, "mul") == 0 || strcmp(op, "mov") == 0);
    if (two_reg_args) {
        if (colors)
            printf("%s%%r%d%s, ", ANSI_GREEN, ins->destination, ANSI_RESET);
        else
            printf("%%r%d, ", ins->destination);
        switch (ins->type) { // instruction type
        case 0:
            if (colors)
                printf("%s%%r%d%s\n", ANSI_YELLOW, ins->source, ANSI_RESET);
            else
                printf("%%r%d\n", ins->source);
            break;
        case 1: // literal
            bool sign = false;
            if ((ins->source >> 7) == 1) {
                sign = true;
                ins->source = ins->source & 0b01111111;
            }
            if (colors) {
                if (!sign)
                    printf("%s#%d%s\n", ANSI_YELLOW, ins->source, ANSI_RESET);
                else
                    printf("%s#-%d%s\n", ANSI_YELLOW, ins->source, ANSI_RESET); // negative
            } else {
                if (!sign)
                    printf("#%d\n", ins->source);
                else
                    printf("#-%d\n", ins->source);
            }
            break;
        case 2:
            memaddr = ((ins->source << 1) & 0b1111111) >> 1;
            if (colors)
                printf("%s&$%d%s\n", ANSI_YELLOW, memaddr, ANSI_RESET);
            else
                printf("&$%d\n", memaddr);
            break;
        case 3:
            reg = ((ins->source << 3) & 0b1111111) >> 3;
            if (colors)
                printf("%s&r%d%s\n", ANSI_YELLOW, reg, ANSI_RESET);
            else
                printf("&r%d\n", reg);
            break;
        default:
            printf("Unknown instruction type\n");
            exit(1);
            break;
        }
    } else if (strcmp(op, "sr") == 0) {
        in_sr = true;
        if (colors)
            printf("%s%d:%s\n", ANSI_GREEN, ins->source, ANSI_RESET);
        else
            printf("%d:\n", ins->source);
    } else if (strcmp(op, "jnz") == 0 || strcmp(op, "jge") == 0) { // jump expressions
        if (ins->destination == 4) {
            if (colors)
                printf("%s@%d%s\n", ANSI_GREEN, ins->source, ANSI_RESET);
            else
                printf("@%d\n", ins->source);
        } else {
            ins->destination = (ins->destination << 1) | ins->type;
            ins->destination = (ins->destination << 8) | ins->source;
            ins->source = ins->destination;
            if (colors)
                printf("%s$%d%s\n", ANSI_YELLOW, ins->source, ANSI_RESET);
            else
                printf("$%d\n", ins->source);
        }
    } else if (strcmp(op, "ret") == 0) {
        printf("\n");
    } else if (strcmp(op, "int") == 0) {
        if (colors)
            printf("%s#%d%s\n", ANSI_YELLOW, ins->source, ANSI_RESET);
        else
            printf("#%d\n", ins->source);
    } else if (strcmp(op, "hlt") == 0) { // account for labels
        if (ins->destination == 1) {
            ins->type = ins->type << 8;
            ins->type = ins->type | ins->source;
            if (colors)
                printf("%s.start%s%s $%d%s\n", ANSI_GREEN, ANSI_RESET, ANSI_YELLOW, ins->type,
                       ANSI_RESET);
            else
                printf(".start $%d\n", ins->type);
        }
    } else if (strcmp(op, "ld") == 0) {
        if (colors)
            printf("%s%%r%d%s, ", ANSI_YELLOW, ins->destination, ANSI_RESET);
        else
            printf("%%r%d, ", ins->destination);
        ins->type = ins->type << 8;
        ins->type = ins->type | ins->source;
        ins->source = ins->type;
        if (colors)
            printf("%s$%d%s\n", ANSI_YELLOW, ins->source, ANSI_RESET);
        else
            printf("$%d\n", ins->source);
    } else if (strcmp(op, "st") == 0) {
        int reconstructed = (ins->destination << 9) | (ins->type << 8) | ins->source;
        ins->source = ins->source & 0x07;
        ins->destination = reconstructed & 0xFFF8;
        ins->destination = ins->destination >> 3;
        if (colors)
            printf("%s$%d%s, %s%%r%d%s\n", ANSI_YELLOW, ins->destination, ANSI_RESET, ANSI_YELLOW,
                   ins->source, ANSI_RESET);
        else
            printf("$%d, %%r%d\n", ins->destination, ins->source);
    }
    line++;
}
