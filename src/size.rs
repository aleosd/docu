extern crate dockworker;
extern crate chrono;

use dockworker::Docker;
use log::warn;
use pretty_bytes::converter::convert;
use chrono::prelude::*;

fn tsdt(unix_timestamp: i64) -> String {
    let naive = NaiveDateTime::from_timestamp(unix_timestamp, 0);
    return naive.format("%Y-%m-%d %H:%M:%S").to_string();
}

pub fn show_size(docker_client: &Docker) {
    let all_images = docker_client.images(true).unwrap();
    all_images.iter().for_each(|image| {
        warn!("{:?}, created at {} - {}",
        image.RepoTags,
        tsdt(image.Created as i64),
        convert(image.Size as f64));
    });
}
