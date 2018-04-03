#include <cstdint>
#include <cstdlib>

struct MCUConf;

extern "C" {

void mcu_conf_free(MCUConf *ptr);

MCUConf *mcu_conf_new(const char *path);

} // extern "C"
