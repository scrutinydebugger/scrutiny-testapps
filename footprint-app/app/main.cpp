#include <cstdlib>
#include "scrutiny.hpp"

// syscall stub to inhibit GCC warnings
extern "C"{
    void _close(void){}
    void _lseek(void){}
    void _read(void){}
    void _write(void){}
}


scrutiny::MainHandler main_handler;
scrutiny::VariableFrequencyLoopHandler loop_handler1;

volatile uint32_t timediff = 10;
unsigned char txbuf[64];
unsigned char rxbuf[64];

volatile unsigned char sink;

extern "C" void idle(void){
    static unsigned char b_in;
    static unsigned char b_out;

    b_in = sink;
    main_handler.receive_data(&b_in, 1);
    main_handler.process(timediff);
    main_handler.pop_data(&b_out, 1);
    sink = b_out;
}


extern "C" void __attribute__((noinline)) loop1(void){
    loop_handler1.process(timediff);
}

int main(int argc, char* argv[])
{
    (void)argc;
    (void)argv;
    scrutiny::Config config;
    config.set_buffers(rxbuf, sizeof(rxbuf), txbuf, sizeof(txbuf));

    main_handler.init(&config);

    while (1){
        idle();
        loop1();
    }
    
    return EXIT_SUCCESS;
}