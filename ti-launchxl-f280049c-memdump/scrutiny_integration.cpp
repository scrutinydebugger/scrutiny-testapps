#include "sci.h"
#include "scrutiny.hpp"
#include <stdint.h>

static uint32_t SCI_DEVICE = 0; // UART on which to talk. Given by main().

// Communication buffer
static unsigned char rx_buffer[128];
static unsigned char tx_buffer[256];    // We generally benefit from a bigger TX buffer to increase memory dump speed.

#pragma DATA_SECTION("ramgs0")
static unsigned char datalogging_buffer[0x2000]; 

static scrutiny::MainHandler main_handler;

bool scrutiny_init(uint32_t const sci_base)
{
    SCI_DEVICE = sci_base;
    scrutiny::Config config;    // Fine to put this on the stack.
    config.set_buffers(rx_buffer, sizeof(rx_buffer), tx_buffer, sizeof(tx_buffer));
    config.display_name = "C2000 testapp"; // Max 32 chars
    config.set_datalogging_buffers(datalogging_buffer, sizeof(datalogging_buffer));
    return main_handler.init(&config) == scrutiny::Status::SUCCESS;
}

void scrutiny_idle_update(uint32_t const timediff_100ns)
{
    // TI does not provide an API to know how many bytes are avaialble in the FIFO, so we loop byte per byte.
    unsigned char byte;

    while (SCI_getRxFIFOStatus(SCI_DEVICE) != SCI_FIFO_RX0)
    {
        byte = SCI_readCharNonBlocking(SCI_DEVICE);
        main_handler.receive_data(&byte, 1);
    }

    main_handler.process(timediff_100ns);

    while (main_handler.data_to_send() > 0 && SCI_getTxFIFOStatus(SCI_DEVICE) != SCI_FIFO_TX16)
    {
        main_handler.pop_data(&byte, 1);
        SCI_writeCharNonBlocking(SCI_DEVICE, byte);
    }
}
