
#include "driverlib.h"
#include "device.h"
#include "board.h"
#include "c2000ware_libraries.h"

#include <stdint.h>

#include "file1.hpp"
#include "file2.hpp"
#include "file3.hpp"
#include "file4.hpp"
#include "file5.hpp"

#include "scrutiny_integration.hpp"

inline uint32_t get_timestamp_100ns(void)
{
    // 100 MHZ / 10 = 100ns period
    // Timer count downward, ~ to invert and make it count up
    return ~CPUTimer_getTimerCount(CPUTIMER_TimeBase_BASE);
}


void main(void)
{
	
    Device_init();
    Device_initGPIO();
    Interrupt_initModule();
    Interrupt_initVectorTable();
    Board_init();
    C2000Ware_libraries_init();


    file1SetValues();
    file2SetValues();
    file3SetValues();
    file4SetValues();
    file5SetValues();

    funcInFile1(1,2);
    file2func1();
    file2func1(123);

    EINT;
    ERTM;

    if (scrutiny_init(XDS_SCIA_BASE) == false)
    {
        while (1)
            ;
    }

    CPUTimer_startTimer(CPUTIMER_TimeBase_BASE); // Free running counter, 1 tick per 100ns
    uint32_t last_timestamp = get_timestamp_100ns();
    for(;;)
    {
        uint32_t const timestamp = get_timestamp_100ns();
        uint32_t const timediff = timestamp - last_timestamp;
        scrutiny_idle_update(timediff);
        last_timestamp = timestamp;
    }
}
