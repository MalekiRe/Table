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

struct TableType create_big_number(long first, long second) {
    struct TableType big_number;
    big_number.table_type_tag = NUMBER;
    struct BigDecimal num;
    num.first = first;
    num.second = second;
    big_number.table_type_union.number = num;
    return big_number;
}

int add(int a, int b) {
    return a + b;
}