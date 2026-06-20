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
scrutiny::VariableFrequencyLoopHandler idle_loop_handler;
static scrutiny::LoopHandler* loops[] = {
    &idle_loop_handler
};


static struct {
    uint16_t u16;
    uint32_t u32;
    uint64_t u64;
    float f32;
    double f64;
    bool boolean;
} rpv_storage = {123,456,789,1.2,3.4};


bool rpv_write_callback(const scrutiny::RuntimePublishedValue rpv, const scrutiny::AnyType *inval, scrutiny::LoopHandler* caller){
    if (rpv.id == 0x1000 && rpv.type == scrutiny::VariableType::uint16){
        rpv_storage.u16 = inval->uint16;
    } else if (rpv.id == 0x1001 && rpv.type == scrutiny::VariableType::uint32){
        rpv_storage.u32 = inval->uint32;
    } else if (rpv.id == 0x1002 && rpv.type == scrutiny::VariableType::uint64){
        rpv_storage.u64 = inval->uint64;
    } else if (rpv.id == 0x1003 && rpv.type == scrutiny::VariableType::float32){
        rpv_storage.f32 = inval->float32;
    } else if (rpv.id == 0x1004 && rpv.type == scrutiny::VariableType::float64){
        rpv_storage.f64 = inval->float64;
    } else if (rpv.id == 0x1005 && rpv.type == scrutiny::VariableType::boolean){
        rpv_storage.boolean = inval->boolean;
    } else {
        return false;   // failure
    }
    return true;    // success
}

bool rpv_read_callback(scrutiny::RuntimePublishedValue rpv, scrutiny::AnyType *outval, scrutiny::LoopHandler* caller){
    if (rpv.id == 0x1000 && rpv.type == scrutiny::VariableType::uint16){
        outval->uint16 = rpv_storage.u16;
    } else if (rpv.id == 0x1001 && rpv.type == scrutiny::VariableType::uint32){
        outval->uint32 = rpv_storage.u32;
    } else if (rpv.id == 0x1002 && rpv.type == scrutiny::VariableType::uint64){
        outval->uint64 = rpv_storage.u64;
    } else if (rpv.id == 0x1003 && rpv.type == scrutiny::VariableType::float32){
        outval->float32 = rpv_storage.f32;
    } else if (rpv.id == 0x1004 && rpv.type == scrutiny::VariableType::float64){
        outval->float64 = rpv_storage.f64;
    } else if (rpv.id == 0x1005 && rpv.type == scrutiny::VariableType::boolean){
        outval->boolean = rpv_storage.boolean;
    } else {
        return false;   // failure
    }
    return true;    // success
}

scrutiny::RuntimePublishedValue rpvs[] = {
    {0x1000, scrutiny::VariableType::uint16},
    {0x1001, scrutiny::VariableType::uint32},
    {0x1002, scrutiny::VariableType::uint64},
    {0x1003, scrutiny::VariableType::float32},
    {0x1004, scrutiny::VariableType::float64},
    {0x1005, scrutiny::VariableType::boolean}
};

bool scrutiny_init(uint32_t const sci_base)
{
    SCI_DEVICE = sci_base;
    scrutiny::Config config;    // Fine to put this on the stack.
    config.set_buffers(rx_buffer, sizeof(rx_buffer), tx_buffer, sizeof(tx_buffer));
    config.display_name = "C2000 testapp"; // Max 32 chars
    config.set_datalogging_buffers(datalogging_buffer, sizeof(datalogging_buffer));
    config.set_loops(loops, 1);
    config.set_published_values(rpvs, sizeof(rpvs) / sizeof(rpvs[0]), rpv_read_callback, rpv_write_callback);
    return main_handler.init(&config) == scrutiny::Status::SUCCESS;
}

void scrutiny_idle_update(uint32_t const timediff_100ns)
{
    // TI does not provide an API to know how many bytes are avaialble in the FIFO, so we loop byte per byte.
    unsigned char byte;
    
    idle_loop_handler.process(timediff_100ns);
    
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
