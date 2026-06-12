#ifndef ___SCRUTINY_BUILD_CONFIG_H___
#define ___SCRUTINY_BUILD_CONFIG_H___

#define SCRUTINY_ENABLE_DATALOGGING 1
#define SCRUTINY_SUPPORT_64BITS 1

#define SCRUTINY_REQUEST_MAX_PROCESS_TIME_US 100000 // If a request takes more than this time to process, it will be nacked.
#define SCRUTINY_COMM_RX_TIMEOUT_US 50000                   // Reset reception state machine when no data is received for that amount of time.
#define SCRUTINY_COMM_HEARTBEAT_TIMEOUT_US 5000000     // Disconnect session if no heartbeat request after this delay

#define SCRUTINY_ACTUAL_PROTOCOL_VERSION SCRUTINY_PROTOCOL_VERSION(1, 0) // protocol version to use

#if SCRUTINY_ENABLE_DATALOGGING
    #define SCRUTINY_DATALOGGING_MAX_SIGNAL 32
    #define SCRUTINY_DATALOGGING_ENCODING SCRUTINY_DATALOGGING_ENCODING_RAW
    #define SCRUTINY_DATALOGGING_BUFFER_32BITS 0
#endif

#endif
