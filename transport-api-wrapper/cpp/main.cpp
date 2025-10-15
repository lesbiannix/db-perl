#include <iostream>
#include "transport_api_wrapper.h"

int main() {
    const char* query = "berlin";
    char* result = locations(query);
    std::cout << result << std::endl;
    free_string(result);
    return 0;
}