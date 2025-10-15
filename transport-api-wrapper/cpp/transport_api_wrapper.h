#ifndef TRANSPORT_API_WRAPPER_H
#define TRANSPORT_API_WRAPPER_H

#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

char* locations(const char* query);
void free_string(char* s);

#ifdef __cplusplus
}
#endif

#endif // TRANSPORT_API_WRAPPER_H