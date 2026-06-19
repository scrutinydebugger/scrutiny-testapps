//    file1.cpp
//        Placeholder file for variable extraction test
//
//   - License : MIT - See LICENSE file
//   - Project : Scrutiny Debugger (github.com/scrutinydebugger/scrutiny-embedded)
//
//    Copyright (c) 2021 Scrutiny Debugger

#include "file1.hpp"

struct StructA
{
    int structAMemberInt;
    unsigned int structAMemberUInt;
    float structAMemberFloat;
    double structAMemberDouble;
    bool structAMemberBool;
};

struct StructB
{
    int structBMemberInt;
    StructA structBMemberStructA;
};

struct StructC
{
    int structCMemberInt;
    struct
    {
        int nestedStructMemberInt;
        float nestedStructMemberFloat;
        struct
        {
            double nestedStructInstance2MemberDouble;
        } nestedStructInstance2;
    } nestedStructInstance;
};

struct StructD
{
    unsigned int bitfieldA : 4;
    unsigned int bitfieldB : 13;
    unsigned int bitfieldC : 8;
    unsigned int bitfieldD;
    unsigned int bitfieldE : 10;
};

volatile char file1GlobalChar;
volatile int file1GlobalInt;
volatile short file1GlobalShort;
volatile long file1GlobalLong;
volatile unsigned char file1GlobalUnsignedChar;
volatile unsigned int file1GlobalUnsignedInt;
volatile unsigned short file1GlobalUnsignedShort;
volatile unsigned long file1GlobalUnsignedLong;
volatile float file1GlobalFloat;
volatile double file1GlobalDouble;
volatile bool file1GlobalBool;

volatile static char file1StaticChar;
volatile static int file1StaticInt;
volatile static short file1StaticShort;
volatile static long file1StaticLong;
volatile static unsigned char file1StaticUnsignedChar;
volatile static unsigned int file1StaticUnsignedInt;
volatile static unsigned short file1StaticUnsignedShort;
volatile static unsigned long file1StaticUnsignedLong;
volatile static float file1StaticFloat;
volatile static double file1StaticDouble;
volatile static bool file1StaticBool;

volatile static StructA file1StructAStaticInstance;

volatile StructA file1StructAInstance;
volatile StructB file1StructBInstance;
volatile StructC file1StructCInstance;
volatile StructD file1StructDInstance;

namespace NamespaceInFile1
{
    namespace NamespaceInFile1Nested1
    {
        volatile unsigned long file1GlobalNestedVar1;
        volatile static unsigned long file1StaticNestedVar1;
    } // namespace NamespaceInFile1Nested1
} // namespace NamespaceInFile1

void file1SetValues()
{
    file1GlobalChar = -10;
    file1GlobalInt = -1000;
    file1GlobalShort = -999;
    file1GlobalLong = -100000;
    file1GlobalUnsignedChar = 55u;
    file1GlobalUnsignedInt = 10001u;
    file1GlobalUnsignedShort = 50000u;
    file1GlobalUnsignedLong = 100002u;
    file1GlobalFloat = 3.1415926f;
    file1GlobalDouble = 1.71;
    file1GlobalBool = true;

    file1StaticChar = 99;
    file1StaticInt = 9876;
    file1StaticShort = -666;
    file1StaticLong = -55555;
    file1StaticUnsignedChar = 44u;
    file1StaticUnsignedInt = 3333u;
    file1StaticUnsignedShort = 22222u;
    file1StaticUnsignedLong = 321321u;
    file1StaticFloat = 1.23456789f;
    file1StaticDouble = 9.87654321;
    file1StaticBool = true;

    file1StructAStaticInstance.structAMemberInt = -789;
    file1StructAStaticInstance.structAMemberUInt = 1472u;
    file1StructAStaticInstance.structAMemberFloat = 88.88f;
    file1StructAStaticInstance.structAMemberDouble = 99.99;
    file1StructAStaticInstance.structAMemberBool = true;

    NamespaceInFile1::NamespaceInFile1Nested1::file1StaticNestedVar1 = 945612345u;

    file1StructAInstance.structAMemberInt = -654;
    file1StructAInstance.structAMemberUInt = 25814;
    file1StructAInstance.structAMemberFloat = 77.77f;
    file1StructAInstance.structAMemberDouble = 66.66;
    file1StructAInstance.structAMemberBool = false;

    file1StructBInstance.structBMemberInt = 5555;
    file1StructBInstance.structBMemberStructA.structAMemberInt = -19999;
    file1StructBInstance.structBMemberStructA.structAMemberUInt = 33333;
    file1StructBInstance.structBMemberStructA.structAMemberFloat = 33.33f;
    file1StructBInstance.structBMemberStructA.structAMemberDouble = 22.22;
    file1StructBInstance.structBMemberStructA.structAMemberBool = true;

    file1StructCInstance.structCMemberInt = 8887;
    file1StructCInstance.nestedStructInstance.nestedStructMemberInt = 22987;
    file1StructCInstance.nestedStructInstance.nestedStructMemberFloat = -147.55f;
    file1StructCInstance.nestedStructInstance.nestedStructInstance2.nestedStructInstance2MemberDouble = 654.654;

    file1StructDInstance.bitfieldA = 13u;
    file1StructDInstance.bitfieldB = 4100u;
    file1StructDInstance.bitfieldC = 222u;
    file1StructDInstance.bitfieldD = 12345u;
    file1StructDInstance.bitfieldE = 777u;

    NamespaceInFile1::NamespaceInFile1Nested1::file1GlobalNestedVar1 = 1111111111u;
}

int funcInFile1(int a, int b)
{
    static volatile long long staticLongInFuncFile1 = -0x123456789abcdef;
    (void)staticLongInFuncFile1;
    return a + b;
}
