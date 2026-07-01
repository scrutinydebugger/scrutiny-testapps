#ifndef _UART_HPP_
#define _UART_HPP_

void uart_init(void);
void uart_putc(char c);
void uart_puts(const char *s);

#endif // _UART_HPP_