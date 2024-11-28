#include "bdump.h"

void print_binary(int num, int leading) {
    if (args.binary == 1 || args.debug == 1) {
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
        va_list args;
        va_start(args, format);
        vprintf(format, args);
        va_end(args);
    }
}

void print_output(Instruction *ins) {
    char *op = match_opcode(ins);
    bool colors = args.colors == 1;
    if (colors)
        printf("%s%s%s ", ANSI_BLUE, op, ANSI_RESET);
    else
        printf("%s ", op);
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
                ins->source = (ins->source >> 1);
            }
            if (colors) {
                if (!sign)
                    printf("%s#%d%s\n", ANSI_YELLOW, ins->source, ANSI_RESET);
                else
                    printf("%s#-%d%s\n", ANSI_YELLOW, ins->source, ANSI_RESET);
            } else {
                if (!sign)
                    printf("#%d\n", ins->source);
                else
                    printf("#-%d\n", ins->source);
            }
            break;
        case 2:
            int memaddr = ((ins->source << 1) & 0b1111111) >> 1;
            if (colors)
                printf("%s&$%d%s\n", ANSI_YELLOW, memaddr, ANSI_RESET);
            else
                printf("&$%d\n", memaddr);
            break;
        case 3:
            int reg = ((ins->source << 3) & 0b1111111) >> 3;
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
    }
}
