//__attribute__((import_module("host"), import_name("print"))) void print(char*);
//__attribute__((import_module("host"), import_name("print_num"))) void print_num(int);
//__attribute__((import_module("host"), import_name("exception"))) void exception(char*);
#include "table_std.h"

unsigned int strlen(const char *s)
{
    unsigned int count = 0;
    while(*s!='\0')
    {
        count++;
        s++;
    }
    return count;
}

char* strcpy(char* destination, const char* source)
{
    // return if no memory is allocated to the destination
    if (destination == NULL) {
        return NULL;
    }

    // take a pointer pointing to the beginning of the destination string
    char *ptr = destination;

    // copy the C-string pointed by source into the array
    // pointed by destination
    while (*source != '\0')
    {
        *destination = *source;
        destination++;
        source++;
    }

    // include the terminating null character
    *destination = '\0';

    // the destination is returned by standard `strcpy()`
    return ptr;
}

char* stralloc(const char* source) {
    char* buff = malloc(strlen(source)+1);
    strcpy(buff, source);
    return buff;
}

void print_value(Value value) {
    switch(value.tag) {
        case NONE:
            print("Type: None, Value: None\n");
            break;
        case NUMBER:
            print("Type: Number, Value: ");
            print_num(value.variant.number);
            print("\n");
            break;
        case STRING:
            print("Type: String, Value: ");
            print(value.variant.string);
            print("\n");
            break;
    }
}

Value test() {
    Value ret;
    ret.variant.string = stralloc("yo yo yo everybody");
    ret.tag = STRING;
    return ret;
}

Value Number_new(int num) {
    Value number;
    number.variant.number = num;
    number.tag = NUMBER;
    number.ref_count = 1;
    return number;
}
Value String_new(const char* str) {
    Value string;
    string.variant.string = stralloc(str);
    string.tag = STRING;
    string.ref_count = 1;
    return string;
}

void ref_dec(Value *value) {
    value->ref_count -= 1;
    if(value->ref_count == 0) {
        print("should free this value here but not implemented yet");
    }
}
void ref_inc(Value *value) {
    value->ref_count += 1;
}
