#ifndef __AUTORUSTMODULE_H__
#define __AUTORUSTMODULE_H__

#include <stdint.h>
#include "architecture/utilities/bskLogging.h"


typedef struct {
    BSKLogger *bskLogger;  //!< BSK Logging
    double varDouble;
}autoRustModuleConfig;

#ifdef __cplusplus
extern "C" {
#endif
void SelfInit_autoRustModule(autoRustModuleConfig *configData, int64_t moduleID);
void Update_autoRustModule(autoRustModuleConfig *configData, uint64_t callTime, int64_t moduleID);
void Reset_autoRustModule(autoRustModuleConfig *configData, uint64_t callTime, int64_t moduleID);

#ifdef __cplusplus
}
#endif

#endif
