// if a program has an array pick one at runtime
// but if it returns just a normal int use that 
use json::{object, JsonValue};
use rand::{Rng, thread_rng};

// this is temporary config, later it would be
// fetch at runtime

pub fn create_new_time_conf() -> JsonValue{
    let configs = object!{
        "user_selection":"random",
        "user_times":[],
        "timer_config":{
            "random_config":{
                "random_max":300,
                "random_min":90,
                "random_arr_len": 10
            },
            "default_config":{
                "default": [600],
                "default_random":[10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 120]
            },
            "presets":{
                "programA":[120],
                "programB":[60, 120, 180]
            }
        }

    };
    return configs;
}

pub fn init() -> JsonValue{
    let mut config = create_new_time_conf();
    let user_selection:&str = &(config["user_selection"].to_string())[..];
    match user_selection{
        "default" => {
            return config["timer_config"]["default_config"]["default"].take();
        },
        "default_random" => {
            return config["timer_config"]["default_config"]["default_random"].take();
            // value=config["timer_config"]["default_config"]["default_random"];
            // delay.push(value);
        },
        "random" => {
            let max_rand:i32 = config["timer_config"]["random_config"]["random_max"].take().as_i32().unwrap_or(0);
            let min_rand:i32 = config["timer_config"]["random_config"]["random_min"].take().as_i32().unwrap_or(0);
            let len_rand:i32 = config["timer_config"]["random_config"]["random_arr_len"].take().as_i32().unwrap_or(0);

            let mut values = JsonValue::new_array();
            for _ in 1..len_rand{
                let i = thread_rng().gen_range(min_rand..max_rand);
                values.push(Some(i));
            }

            return values
        },
        _=>{return config}
    }
}

pub fn get_app_time(){
    println!("{}", (init()).dump())
}
