#include "driverlib.h"
#include "device.h"
#include "board.h"
#include "c2000ware_libraries.h"

#include "scrutinytest/scrutinytest.hpp"
#include "scrutiny.hpp"
#include <stdint.h>

static volatile int callback_count = 0;
static volatile int end_bp = 0;

void scrutinytest_failure_callback(){
    callback_count++;
}

void main(void)
{
    Device_init();
    Device_initGPIO();
    Interrupt_initModule();
    Interrupt_initVectorTable();
    Board_init();
    C2000Ware_libraries_init();

    EINT;
    ERTM;

    int result = scrutinytest::main();
    volatile unsigned long pass = scrutinytest::pass_count();
    volatile unsigned long failure = scrutinytest::failure_count();
    volatile unsigned long error = scrutinytest::error_count();
    
    end_bp++;
    return;
}
