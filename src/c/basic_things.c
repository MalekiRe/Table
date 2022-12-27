__attribute__((import_module("host"), import_name("print"))) void print(char*);
__attribute__((import_module("host"), import_name("print_num"))) void print_num(long, long);

struct BigDecimal {
    long first;
    long second;
};

union TableTypeUnion {
    char none;
    struct BigDecimal number;
    char* string;
};

enum TableTypeTag {
    NONE = 0,
    NUMBER = 1,
    STRING = 2,
};

struct TableType {
    enum TableTypeTag table_type_tag;
    union TableTypeUnion table_type_union;
};

void print_table_type(struct TableType table_type) {
    switch(table_type.table_type_tag) {
        case NONE: print("none"); break;
        case NUMBER: print("number: "); print_num(table_type.table_type_union.number.first, table_type.table_type_union.number.second); break;
        case STRING: print(table_type.table_type_union.string); break;
    }
}

struct TableType create_big_number(long first, long second) {
    struct TableType big_number;
    big_number.table_type_tag = NUMBER;
    struct BigDecimal num;
    num.first = first;
    num.second = second;
    big_number.table_type_union.number = num;
    return big_number;
};

enum TableOperators {
    ADD,
    SUBTRACT,
    MULTIPLY,
    DIVIDE,
};

struct TableType table_operator(struct TableType lhs, struct TableType rhs, enum TableOperators table_operator) {
    if(lhs.table_type_tag != NUMBER) {
        //printf("error not a number in operator exp\n");
        //exit(-1);
    }
    if(rhs.table_type_tag != NUMBER) {
        //printf("error not a number in operator exp\n");
        //exit(-1);
    }
    switch (table_operator) {
        case ADD:
        lhs.table_type_union.number.first += rhs.table_type_union.number.first;
        lhs.table_type_union.number.second += rhs.table_type_union.number.second;
        break;
        case SUBTRACT:
        lhs.table_type_union.number.first -= rhs.table_type_union.number.first;
        lhs.table_type_union.number.second -= rhs.table_type_union.number.second;
        break;
        case MULTIPLY:
        lhs.table_type_union.number.first *= rhs.table_type_union.number.first;
        lhs.table_type_union.number.second *= rhs.table_type_union.number.second;
        break;
        case DIVIDE:
        lhs.table_type_union.number.first /= rhs.table_type_union.number.first;
        lhs.table_type_union.number.second /= rhs.table_type_union.number.second;
        break;
    }
    return lhs;
}

int add(int a, int b) {
    return a + b;
}