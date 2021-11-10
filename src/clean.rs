extern crate dockworker;
extern crate pretty_bytes;

use dockworker::RemovedImage;
use dockworker::{container::ContainerFilters, Docker};
use log::{debug, info};
use pretty_bytes::converter::convert;
use serde_json;

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct PrunedImagesData {
    ImagesDeleted: Vec<RemovedImage>,
    SpaceReclaimed: i64,
}

impl PrunedImagesData {
    fn sise_verbose(&self) -> String {
        convert(self.SpaceReclaimed as f64)
    }

    fn count(&self) -> usize {
        self.ImagesDeleted.len()
    }
}

pub fn clean(docker_client: &Docker) {
    clean_images(docker_client);
    clean_containers(docker_client);
    clean_untagged_images(docker_client);
}

/// Removes all dandling images
fn clean_images(docker_client: &Docker) {
    info!("Removing dandling images");
    let response = docker_client.prune_image(true).unwrap();
    let response_string = serde_json::to_string(&response).unwrap();
    let response_data: PrunedImagesData = serde_json::from_str(&response_string).unwrap();
    info!(
        "\tRemoved {} images, space reclamed: {}\n",
        response_data.count(),
        response_data.sise_verbose()
    );
}

/// Removes all stopped containers
fn clean_containers(docker_client: &Docker) {
    info!("Removing all stopped containers");
    let filter = ContainerFilters::new();
    let all_contaitersa = Some(true);
    let show_size = Some(true);
    let containers = docker_client
        .list_containers(all_contaitersa, None, show_size, filter)
        .unwrap();
    containers.iter().for_each(|c| {
        debug!(
            "\tRemoving container {}, size: {}",
            c.Image,
            convert(c.SizeRw.unwrap_or(0) as f64)
        );
    });
    info!("Done!");
}

/// Remove all images with ["<none>:<none>"] tag
fn clean_untagged_images(docker_client: &Docker) {
    let untagged_tags = vec!["<none>:<none>"];
    info!("Removing images tagged only with {:?}", untagged_tags);
    let all_images = docker_client.images(true).unwrap();
    let mut size_reclamed = 0;
    all_images.iter().for_each(|image| {
        if image.RepoTags == untagged_tags {
            debug!("Removing image {}", image.Id);
            size_reclamed += image.Size;
        }
    });
    info!("\tSpace reclamed: {}", convert(size_reclamed as f64));
}
