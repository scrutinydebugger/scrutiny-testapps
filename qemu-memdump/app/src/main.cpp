#include <stdint.h>

#include "uart.hpp"
#include "file1.hpp"
#include "file2.hpp"
#include "file3.hpp"
#include "file4.hpp"
#include "file5.hpp"


int main(void)
{
    uart_init();

    file1SetValues();
    file2SetValues();
    file3SetValues();
    file4SetValues();
    file5SetValues();
    funcInFile1(1, 2);
    file2func1();
    file2func1(123);

    uart_puts("Ready to dump\n");

    while(1);

    return 0;
}
