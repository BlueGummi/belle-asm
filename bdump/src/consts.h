#ifndef CONSTS_H
#define CONSTS_H

#define HLT_OP 0b0000 // we need this
#define ADD_OP 0b0001 // we also need this
#define JGE_OP 0b0010 // maybe optional ?
#define CL_OP 0b0011  // maybe optional ?
#define DIV_OP 0b0100 // we need this
#define RET_OP 0b0101 // we need this
#define LD_OP 0b0110  // we need this
#define ST_OP 0b0111  // we need this
#define SWP_OP 0b1000 // we need this
#define JZ_OP 0b1001 // maybe optional ?
#define CMP_OP 0b1010 // we need this
#define MUL_OP 0b1011 // we need this
#define SET_OP 0b1100 // we need this
#define INT_OP 0b1101 // we need this
#define MOV_OP 0b1110 // we need this
#define SR_OP 0b1111  // we need this

#ifdef __cplusplus
extern "C" {
#endif

static int supports_ansi() {
    const char *term = getenv("TERM");
    return term && (strstr(term, "xterm") || strstr(term, "screen") || strstr(term, "linux"));
}

#if defined(ENABLE_ANSI) && supports_ansi()
    #define ANSI_RESET "\033[0m"
    #define ANSI_BOLD "\033[1m"
    #define ANSI_UNDERLINE "\033[4m"

    #define ANSI_BLACK "\033[30m"
    #define ANSI_RED "\033[31m"
    #define ANSI_GREEN "\033[32m"
    #define ANSI_YELLOW "\033[33m"
    #define ANSI_BLUE "\033[34m"
    #define ANSI_MAGENTA "\033[35m"
    #define ANSI_CYAN "\033[36m"
    #define ANSI_WHITE "\033[37m"
    #define ANSI_GRAY "\033[90m"
    #define ANSI_LIGHT_GRAY "\033[37m"

    #define ANSI_BG_BLACK "\033[40m"
    #define ANSI_BG_RED "\033[41m"
    #define ANSI_BG_GREEN "\033[42m"
    #define ANSI_BG_YELLOW "\033[43m"
    #define ANSI_BG_BLUE "\033[44m"
    #define ANSI_BG_MAGENTA "\033[45m"
    #define ANSI_BG_CYAN "\033[46m"
    #define ANSI_BG_WHITE "\033[47m"
#else
    #define ANSI_RESET ""
    #define ANSI_BOLD ""
    #define ANSI_UNDERLINE ""

    #define ANSI_BLACK ""
    #define ANSI_RED ""
    #define ANSI_GREEN ""
    #define ANSI_YELLOW ""
    #define ANSI_BLUE ""
    #define ANSI_MAGENTA ""
    #define ANSI_CYAN ""
    #define ANSI_WHITE ""
    #define ANSI_GRAY ""
    #define ANSI_LIGHT_GRAY ""

    #define ANSI_BG_BLACK ""
    #define ANSI_BG_RED ""
    #define ANSI_BG_GREEN ""
    #define ANSI_BG_YELLOW ""
    #define ANSI_BG_BLUE ""
    #define ANSI_BG_MAGENTA ""
    #define ANSI_BG_CYAN ""
    #define ANSI_BG_WHITE ""
#endif

#ifdef __cplusplus
}
#endif

#endif
