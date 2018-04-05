#include <cstdint>
#include <cstdlib>

struct MCUConf;

struct Pins;

extern "C" {

void mcu_conf_free(MCUConf *ptr);

void mcu_conf_free_name(char *name);

char *mcu_conf_get_name(MCUConf *ptr);

Pins *mcu_conf_get_pins(MCUConf *ptr);

MCUConf *mcu_conf_new(const char *path);

void pins_find_pin(Pins *ptr, const char *pin_name);

uint32_t pins_get_size(Pins *ptr);

} // extern "C"
