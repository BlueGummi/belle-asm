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

void print_help(char *bin) { // bin is the name of the bin
    printf("The disassembler for %sBELLE-ISA%s\n\n", ANSI_BOLD, ANSI_RESET);
    printf("%sUsage:%s %s [OPTIONS] <FILE>\n\n", ANSI_UNDERLINE, ANSI_RESET, bin);
    printf("%s%sArguments:%s\n", ANSI_BOLD, ANSI_UNDERLINE, ANSI_RESET);
    printf("  %s<FILE>%s Path to input\n\n", ANSI_BOLD, ANSI_RESET);
    printf("%s%sOptions:%s\n", ANSI_BOLD, ANSI_UNDERLINE, ANSI_RESET);
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
void print_instruction_header(int line, bool colors) {
    if (colors) {
        printf("%sline %*d:%s ", ANSI_RED, 3, line, ANSI_RESET);
    } else {
        printf("line %*d: ", 3, line);
    }
}
