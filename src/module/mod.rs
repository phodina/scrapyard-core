pub mod peripheral;
pub mod middleware;
pub mod component;

trait Module {
    fn is_enabled() -> bool;
    fn set_enabled(enable: bool);

    fn get_name() -> String;
    fn get_module_name() -> String;

    fn is_configured() -> bool;

    // TODO: Return Result
    fn setup();

    // TODO: Pass config & return Result
    fn import();
    fn export();

    // TODO: Pass param & return param
    fn set_param();
    fn get_param();
}
