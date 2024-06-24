fn main() {
    println!(
        "User's Name            whoami::realname():    {}",
        whoami::realname(),
    );
    println!(
        "User's Username        whoami::username():    {}",
        whoami::username(),
    );

    let a = whoami::langs();
    match a {
        Ok(iterator) => {
            let avec: Vec<whoami::Language> = iterator.collect();
            println!("Length: {}",avec.len());
            for language in avec {
                println!("User's Language        whoami::lang():        {:?}", language);
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
    // println!(
    //     "User's Language        whoami::lang():        {:?}",
    //     whoami::langs().into_iter(),
    // );
    println!(
        "Device's Pretty Name   whoami::devicename():  {}",
        whoami::devicename(),
    );
    // println!(
    //     "Device's Hostname      whoami::hostname():    {}",
    //     fallible::hostname(),
    // );
    println!(
        "Device's Platform      whoami::platform():    {}",
        whoami::platform(),
    );
    println!(
        "Device's OS Distro     whoami::distro():      {}",
        whoami::distro(),
    );
    println!(
        "Device's Desktop Env.  whoami::desktop_env(): {}",
        whoami::desktop_env(),
    );
    println!(
        "Device's CPU Arch      whoami::arch():        {}",
        whoami::arch(),
    );
}
