#include <stdint.h>
#include "uart.hpp"

void uart_init(void)
{
    UART_CTRL   = 0x3;  /* enable TX (bit1) and RX (bit0) */
    UART_SCALER = 0;    /* baud-rate scaler ignored by QEMU */
}

void uart_putc(char c)
{
    while (!(UART_STATUS & UART_STATUS_TE))
        ;
    UART_DATA = (uint32_t)c;
}

char uart_getc(void)
{
    while (!(UART_STATUS & UART_STATUS_DR))
        ;
    return (char)(UART_DATA & 0xFF);
}

void uart_puts(const char *s)
{
    while (*s)
        uart_putc(*s++);
}

void uart_puthex(uint32_t val, int digits)
{
    for (int i = (digits - 1) * 4; i >= 0; i -= 4)
        uart_putc("0123456789abcdef"[(val >> i) & 0xF]);
}