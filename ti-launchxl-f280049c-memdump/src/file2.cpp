//    file2.cpp
//        Placeholder file for variable extraction test
//
//   - License : MIT - See LICENSE file
//   - Project : Scrutiny Debugger (github.com/scrutinydebugger/scrutiny-embedded)
//
//    Copyright (c) 2021 Scrutiny Debugger

#include "file2.hpp"

class classAFile2
{
  public:
    int intInClassA;
};

namespace namespaceAFile2
{
    class classBFile2
    {
        class ClassBAFile2
        {
          public:
            int intInClassBA;
            classAFile2 classAInstance;
        };

      public:
        int intInClassB;
        ClassBAFile2 nestedClassInstance;
    };
} // namespace namespaceAFile2


volatile char file2GlobalChar;
volatile int file2GlobalInt;
volatile short file2GlobalShort;
volatile long file2GlobalLong;
volatile unsigned char file2GlobalUnsignedChar;
volatile unsigned int file2GlobalUnsignedInt;
volatile unsigned short file2GlobalUnsignedShort;
volatile unsigned long file2GlobalUnsignedLong;
volatile float file2GlobalFloat;
volatile double file2GlobalDouble;
volatile bool file2GlobalBool;
volatile namespaceAFile2::classBFile2 file2ClassBInstance;
volatile int file2GlobalArray1Int5[5];
volatile float file2GlobalArray2x2Float[2][2];

volatile static char file2StaticChar;
volatile static int file2StaticInt;
volatile static short file2StaticShort;
volatile static long file2StaticLong;
volatile static unsigned char file2StaticUnsignedChar;
volatile static unsigned int file2StaticUnsignedInt;
volatile static unsigned short file2StaticUnsignedShort;
volatile static unsigned long file2StaticUnsignedLong;
volatile static float file2StaticFloat;
volatile static double file2StaticDouble;
volatile static bool file2StaticBool;
volatile static namespaceAFile2::classBFile2 file2ClassBStaticInstance;

namespace NamespaceInFile2
{
    enum EnumA
    {
        eVal1,
        eVal2,
        eVal3 = 100,
        eVal4
    };

    volatile EnumA instance_enumA;
    volatile static EnumA staticInstance_enumA;
} // namespace NamespaceInFile2

volatile NamespaceInFile2::EnumA instance2_enumA;
volatile static NamespaceInFile2::EnumA staticInstance2_enumA;

void file2SetValues()
{
    file2GlobalChar = 20;
    file2GlobalInt = 2000;
    file2GlobalShort = 998;
    file2GlobalLong = 555555;
    file2GlobalUnsignedChar = 254u;
    file2GlobalUnsignedInt = 1234u;
    file2GlobalUnsignedShort = 12345u;
    file2GlobalUnsignedLong = 1234567u;
    file2GlobalFloat = 0.1f;
    file2GlobalDouble = 0.11111111111111;
    file2GlobalBool = false;

    file2StaticChar = -66;
    file2StaticInt = -8745;
    file2StaticShort = -9876;
    file2StaticLong = -12345678;
    file2StaticUnsignedChar = 12u;
    file2StaticUnsignedInt = 34u;
    file2StaticUnsignedShort = 56u;
    file2StaticUnsignedLong = 78u;
    file2StaticFloat = 2.22222f;
    file2StaticDouble = 3.3333;
    file2StaticBool = true;

    NamespaceInFile2::instance_enumA = NamespaceInFile2::eVal2;
    NamespaceInFile2::staticInstance_enumA = NamespaceInFile2::eVal3;
    instance2_enumA = NamespaceInFile2::eVal4;
    staticInstance2_enumA = NamespaceInFile2::eVal1;

    file2GlobalArray1Int5[0] = 1111;
    file2GlobalArray1Int5[1] = 2222;
    file2GlobalArray1Int5[2] = 3333;
    file2GlobalArray1Int5[3] = 4444;
    file2GlobalArray1Int5[4] = 5555;

    file2GlobalArray2x2Float[0][0] = 1.1f;
    file2GlobalArray2x2Float[0][1] = 2.2f;
    file2GlobalArray2x2Float[1][0] = 3.3f;
    file2GlobalArray2x2Float[1][1] = 4.4f;

    file2ClassBInstance.intInClassB = -11111;
    file2ClassBInstance.nestedClassInstance.intInClassBA = -22222;
    file2ClassBInstance.nestedClassInstance.classAInstance.intInClassA = -3333;

    file2ClassBStaticInstance.intInClassB = -4444;
    file2ClassBStaticInstance.nestedClassInstance.intInClassBA = -5555;
    file2ClassBStaticInstance.nestedClassInstance.classAInstance.intInClassA = -6666;
}

void file2func1()
{
    volatile static int file2func1Var = -8877;
    (void)file2func1Var;
}

void file2func1(int x)
{
    (void)x;
    volatile static double file2func1Var = 963258741.123;
    (void)file2func1Var;
}
