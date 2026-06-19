//    file3.cpp
//        Placeholder file for variable extraction test
//
//   - License : MIT - See LICENSE file
//   - Project : Scrutiny Debugger (github.com/scrutinydebugger/scrutiny-embedded)
//
//    Copyright (c) 2021 Scrutiny Debugger

#include "file3.hpp"

union File3Union
{
    uint64_t u64_var;
    uint32_t u32_var;
    uint16_t u16_var;
};

union File3AnonymousBitfieldInUnion
{
    struct
    {
        uint16_t bit1 : 1;
        uint16_t bit2_4 : 3;
        uint16_t bit5_8 : 4;
        uint16_t bit9_13 : 4;
    } bits;
    uint16_t val;
};

namespace FileNamespace
{
    class File3TestClass
    {
      public:
        enum File3Enum
        {
            AAA,
            BBB,
            CCC
        };

        File3Enum m_file3testclass_inclassenum;
        struct
        {
            uint32_t field1;
            uint32_t field2;
            union
            {
                uint64_t field3_u64;
                struct
                {
                    uint16_t p0; // LSB
                    uint16_t p1;
                    uint16_t p2;
                    uint16_t p3;
                } field3_u16;

                struct
                {
                    uint32_t p0; // LSB
                    uint32_t p1;
                } field3_u32;
                struct
                {
                    File3Enum p0 : 5;
                    File3Enum p1 : 7;
                    File3Enum p2 : 10;
                    File3Enum p3 : 10;
                } field3_enum_bitfields;
            } field3;
        } m_file3_complex_struct;
    };
} // namespace FileNamespace

File3Union file3_union;
File3AnonymousBitfieldInUnion file3_anonbitfield_in_union;
FileNamespace::File3TestClass file3_test_class;

void file3SetValues()
{
    file3_union.u64_var = 0x0123456789abcdefu;
    file3_union.u32_var = 0x99887766;
    file3_union.u16_var = 0xAABB;

    file3_anonbitfield_in_union.val = 0x55;
    file3_anonbitfield_in_union.bits.bit1 = 0;
    file3_anonbitfield_in_union.bits.bit5_8 = 7;

    // result should be 0x74

    file3_test_class.m_file3testclass_inclassenum = FileNamespace::File3TestClass::BBB;

    file3_test_class.m_file3_complex_struct.field1 = 0x11223344;
    file3_test_class.m_file3_complex_struct.field2 = 0x55667788;
    file3_test_class.m_file3_complex_struct.field3.field3_u64 = 0x123456789abcdef0;
    file3_test_class.m_file3_complex_struct.field3.field3_u32.p0 = 0xAABBCCDD;
    file3_test_class.m_file3_complex_struct.field3.field3_u16.p1 = 0x7766;
    file3_test_class.m_file3_complex_struct.field3.field3_enum_bitfields.p0 = FileNamespace::File3TestClass::CCC;
    // Should read field3 = 0x123456787766CCC2
}
