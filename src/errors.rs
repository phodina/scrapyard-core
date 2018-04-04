error_chain!{
        foreign_links {
            Io(::std::io::Error);
            SerdeJSON(::serde_json::Error);
            Parse(::std::num::ParseIntError);
        }
    }