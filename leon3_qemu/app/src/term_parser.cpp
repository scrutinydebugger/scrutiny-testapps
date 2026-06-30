#include <stdint.h>
#include "uart.hpp"

/* Read a line with echo and backspace support. Strips the trailing CR/LF. */
void uart_readline(char *buf, int maxlen)
{
    int n = 0;
    for (;;)
    {
        char c = uart_getc();
        if (c == '\r' || c == '\n')
            break;
        if ((c == '\b' || c == 0x7f) && n > 0)
        {
            n--;
            uart_puts("\b \b");
            continue;
        }
        if (n < maxlen - 1)
        {
            buf[n++] = c;
            uart_putc(c);   /* echo */
        }
    }
    buf[n] = '\0';
    uart_puts("\r\n");
}

/* Parse a "0x..." hex string. Returns 1 on success, 0 on error. */
int parse_hex(const char *s, uintptr_t *out)
{
    if (s[0] != '0' || (s[1] != 'x' && s[1] != 'X'))
        return 0;
    s += 2;
    uintptr_t val = 0;
    int digits = 0;
    while (*s)
    {
        char c = *s++;
        uint8_t nib;
        if      (c >= '0' && c <= '9') nib = (uint8_t)(c - '0');
        else if (c >= 'a' && c <= 'f') nib = (uint8_t)(c - 'a' + 10);
        else if (c >= 'A' && c <= 'F') nib = (uint8_t)(c - 'A' + 10);
        else return 0;
        val = (val << 4) | nib;
        digits++;
    }
    if (digits == 0) return 0;
    *out = val;
    return 1;
}

/* Split whitespace-delimited tokens in-place. Returns token count. */
int split_tokens(char *line, char **tokens, int max_tokens)
{
    int count = 0;
    char *p = line;
    while (*p && count < max_tokens)
    {
        while (*p == ' ' || *p == '\t') p++;
        if (!*p) break;
        tokens[count++] = p;
        while (*p && *p != ' ' && *p != '\t') p++;
        if (*p) *p++ = '\0';
    }
    return count;
}


void memdump(uintptr_t startAddr, uint32_t length)
{
    uintptr_t addr = startAddr;
    while (addr < startAddr + length)
    {
        const uint8_t *ptr = reinterpret_cast<const uint8_t *>(addr);
        uintptr_t nToPrint = startAddr + length - addr;
        if (nToPrint > 16)
            nToPrint = 16;

        uart_puts("0x");
        uart_puthex((uint32_t)addr, 8);
        uart_puts(": ");
        for (uintptr_t i = 0; i < nToPrint; i++)
            uart_puthex(ptr[i], 2);
        uart_puts("\r\n");

        addr += nToPrint;
    }
}


void process_dump_cmd(void)
{
    uart_puts("Enter address/size pairs (hex, 0x prefix, space-separated):\r\n");

    static char line[256];
    static char *tokens[32];

    uart_readline(line, (int)sizeof(line));

    int argc = split_tokens(line, tokens, 32);
    if (argc == 0)
        return;
    if (argc % 2 != 0)
    {
        uart_puts("error: odd number of arguments\r\n");
        return;
    }

    for (int i = 0; i < argc; i += 2)
    {
        uintptr_t addr, size;
        if (!parse_hex(tokens[i], &addr) || !parse_hex(tokens[i + 1], &size))
        {
            uart_puts("error: expected 0x-prefixed hex values\r\n");
            break;
        }
        memdump(addr, (uint32_t)size);
    }
}