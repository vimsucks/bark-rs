use crate::cli;
use crate::device;
use bark_client;

pub async fn push(push_args: &cli::PushArgs) {
    let device = find_device(push_args);
    let mut push_builder = bark_client::new_push();

    if let Some(t) = &push_args.title {
        push_builder.with_title(t);
    }
    if let Some(b) = &push_args.body {
        push_builder.with_body(b);
    }
    if let Some(l) = &push_args.level {
        if let Some(ll) = match l {
            cli::PushLevel::Active => Some(bark_client::PushLevel::Active),
            cli::PushLevel::TimeSensitive => Some(bark_client::PushLevel::TimeSensitive),
            cli::PushLevel::Passive => Some(bark_client::PushLevel::Passive),
        } {
            push_builder.with_level(ll);
        }
    }
    if let Some(b) = push_args.badge {
        push_builder.with_badge(b);
    }
    push_builder.with_auto_copy(push_args.auto_copy);
    if let Some(c) = &push_args.copy {
        push_builder.with_copy(c);
    }
    if let Some(s) = &push_args.sound {
        push_builder.with_sound(s);
    }
    if let Some(i) = &push_args.icon {
        push_builder.with_icon(i);
    }
    if let Some(g) = &push_args.group {
        push_builder.with_group(g);
    }
    push_builder.with_archive(push_args.archive);
    if let Some(u) = &push_args.url {
        push_builder.with_url(u);
    }

    println!("{:?}", push_builder);
    let bark_app = bark_client::App::new(&device.server_url, &device.device_key);
    match bark_app.send(&push_builder).await {
        Ok(result) => println!("{:?}", result),
        Err(e) => println!("{}", e),
    }
}

fn find_device(push_args: &cli::PushArgs) -> device::Device {
    match &push_args.device {
        Some(device_name) => device::find_device(&device_name)
            .expect(&format!("device with name \"{}\" not found", &device_name)),
        None => device::default_device().expect("default device not found"),
    }
}
