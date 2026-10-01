/*
Note: this hexpat helped a bit in debugging the csb-editor-rs
It's a TODO since I ended up compiling original CsbEditor with debug instrumentation that mostly helped me debug with pointer-by-pointer percicion

This hexpat is based off of Skyth's SonicAudioTools' Lib
That's why it's also licensed under the MIT license

This hexpat is MIT licensed:
    Copyright (c) 2019 Skyth
    Copyright (c) 2026 Oleg "OSA413" Sokolov
    
    Permission is hereby granted, free of charge, to any person obtaining a copy
    of this software and associated documentation files (the "Software"), to deal
    in the Software without restriction, including without limitation the rights
    to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
    copies of the Software, and to permit persons to whom the Software is
    furnished to do so, subject to the following conditions:
    
    The above copyright notice and this permission notice shall be included in all
    copies or substantial portions of the Software.
    
    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
    FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
    AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
    LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
    OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
    SOFTWARE.
*/

// Also sprach Source\SonicAudioLib\Archives\CriAaxArchive.cs
// und Source\SonicAudioLib\CriMw\CriTableReader.cs

#pragma endian big

import std.ptr;

struct CSBEntry {
    u8 value;
};

struct CSB {
    char magic[0x4];
    u32 length;
    u8 unknown;
    u8 encoding_type;
    u16 row_position;
    u32 string_pool_position;
    u32 data_pool_position;
    u32 relative_table_name_pointer;
    u32 debug = std::ptr::relative_to_pointer(relative_table_name_pointer + string_pool_position + 0x08);
    char table_name[] @ debug;
    u16 field_count;
    u16 row_length;
    u32 row_count;
    CSBEntry entries[row_count * row_length];
};

CSB csb @ 0x00;