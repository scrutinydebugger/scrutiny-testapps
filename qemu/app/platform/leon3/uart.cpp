#include <stdint.h>
#include "uart.hpp"

#define UART0_BASE  0x80000100U
#define UART_DATA   (*(volatile uint32_t *)(UART0_BASE + 0x00))
#define UART_STATUS (*(volatile uint32_t *)(UART0_BASE + 0x04))
#define UART_CTRL   (*(volatile uint32_t *)(UART0_BASE + 0x08))
#define UART_SCALER (*(volatile uint32_t *)(UART0_BASE + 0x0C))

#define UART_STATUS_DR  (1u << 0)   /* RX data ready */
#define UART_STATUS_TE  (1u << 2)   /* TX FIFO empty — safe to write */

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

void uart_puts(const char *s)
{
    while (*s)
        uart_putc(*s++);
}
