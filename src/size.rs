extern crate dockworker;
extern crate chrono;

use dockworker::Docker;
use dockworker::container::ContainerFilters;
use log::warn;
use pretty_bytes::converter::convert;
use chrono::prelude::*;

fn tsdt(unix_timestamp: i64) -> String {
    let naive = NaiveDateTime::from_timestamp(unix_timestamp, 0);
    return naive.format("%Y-%m-%d %H:%M:%S").to_string();
}

pub fn show_size(docker_client: &Docker) {
    let all_images = docker_client.images(true).unwrap();
    let mut all_images_size = 0;
    all_images.iter().for_each(|image| {
        warn!("{:?} {:?}, created at {} - {}",
        image.RepoDigests,
        image.RepoTags,
        tsdt(image.Created as i64),
        convert(image.Size as f64));
        all_images_size += image.Size;
    });
    warn!("Got {} images, {} total size", all_images.len(), convert(all_images_size as f64));

    let all_contaiters = docker_client.list_containers(
        Some(true), Some(100), Some(true),
        ContainerFilters::default()
    ).unwrap();
    all_contaiters.iter().for_each(|container| {
        warn!("{:?} {:?}, created at {} - {}",
        container.Image,
        container.Status,
        tsdt(container.Created as i64),
        convert(container.SizeRw.unwrap() as f64));
    });
}
