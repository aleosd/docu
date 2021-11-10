extern crate chrono;
extern crate dockworker;

use chrono::prelude::*;
use dockworker::container::ContainerFilters;
use dockworker::Docker;
use log::{info, warn};
use pretty_bytes::converter::convert;

fn tsdt(unix_timestamp: i64) -> String {
    let naive = NaiveDateTime::from_timestamp(unix_timestamp, 0);
    return naive.format("%Y-%m-%d %H:%M:%S").to_string();
}

pub fn show_size(docker_client: &Docker) {
    let all_images = docker_client.images(true).unwrap();
    let mut all_images_size = 0;
    all_images.iter().for_each(|image| {
        info!(
            "{:?} {:?}, created at {} - {}",
            image.RepoDigests,
            image.RepoTags,
            tsdt(image.Created as i64),
            convert(image.Size as f64)
        );
        all_images_size += image.Size;
    });
    warn!(
        "Got {} images, {} total size",
        all_images.len(),
        convert(all_images_size as f64)
    );

    let all_contaiters = docker_client
        .list_containers(
            Some(true),
            Some(100),
            Some(true),
            ContainerFilters::default(),
        )
        .unwrap();
    let mut all_containers_size = 0;
    all_contaiters.iter().for_each(|container| {
        let container_size = container.SizeRw.unwrap_or(0);
        warn!(
            "{:?} {:?}, created at {} - {}",
            container.Image,
            container.Status,
            tsdt(container.Created as i64),
            convert(container_size as f64)
        );
        all_containers_size += container_size;
    });
    warn!(
        "Got {} containers, {} total size",
        all_contaiters.len(),
        convert(all_containers_size as f64)
    );
}
