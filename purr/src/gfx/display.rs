use crate::gfx::card::Card;
use crate::gfx::render::{copy_to_display, fit_to_frame, resize_to_display};
use anyhow::{Result, anyhow, bail};
use drm::Device as BasicDevice;
use drm::buffer::DrmFourcc;
use drm::control::{AtomicCommitFlags, Device as ControlDevice, atomic, connector, crtc, property};
use drm::{ClientCapability, control};
use image::DynamicImage;
use image::math::Rect;

const DEPTH: u32 = 24;
const BITS_PER_PIXEL: u32 = 32;

pub fn show(
    images: &[DynamicImage],
    start: impl FnOnce() -> Result<()>,
    next: impl Fn() -> Result<()>,
    wait: impl Fn() -> Result<()>,
) -> Result<()> {
    if !Card::has_primary()? {
        bail!("no graphics card found");
    }

    let card = Card::open_primary()?;
    card.set_client_capability(ClientCapability::UniversalPlanes, true)?;
    card.set_client_capability(ClientCapability::Atomic, true)?;

    let resource_handles = card.resource_handles()?;
    let connectors: Vec<connector::Info> = resource_handles
        .connectors()
        .iter()
        .flat_map(|con| card.get_connector(*con, true))
        .collect();
    let crtcs: Vec<crtc::Info> = resource_handles
        .crtcs()
        .iter()
        .flat_map(|crtc| card.get_crtc(*crtc))
        .collect();

    let first_connector = connectors
        .iter()
        .find(|&i| i.state() == connector::State::Connected)
        .ok_or_else(|| anyhow!("no connectors are connected"))?;
    let &mode = first_connector
        .modes()
        .first()
        .ok_or_else(|| anyhow!("no modes found on connector"))?;

    let crtc = crtcs.first().ok_or_else(|| anyhow!("no crtcs found"))?;

    let (width, height) = mode.size();
    let output_rect = Rect {
        x: 0,
        y: 0,
        width: width as u32,
        height: height as u32,
    };

    let planes = card.plane_handles()?;
    let (better_planes, compatible_planes): (
        Vec<control::plane::Handle>,
        Vec<control::plane::Handle>,
    ) = planes
        .iter()
        .filter(|&&plane| {
            card.get_plane(plane)
                .map(|plane_info| {
                    let compatible_crtcs =
                        resource_handles.filter_crtcs(plane_info.possible_crtcs());
                    compatible_crtcs.contains(&crtc.handle())
                })
                .unwrap_or(false)
        })
        .partition(|&&plane| {
            if let Ok(props) = card.get_properties(plane) {
                for (&id, &val) in props.iter() {
                    if let Ok(info) = card.get_property(id)
                        && info.name().to_str().map(|x| x == "type").unwrap_or(false)
                    {
                        return val == (control::PlaneType::Primary as u32).into();
                    }
                }
            }
            false
        });
    let plane = *better_planes
        .first()
        .or(compatible_planes.first())
        .or(planes.first())
        .ok_or_else(|| anyhow!("no planes found"))?;

    let con_props = card
        .get_properties(first_connector.handle())
        .map_err(|e| anyhow!("could not get props of connector: {e}"))?
        .as_hashmap(&card)
        .map_err(|e| anyhow!("could not get a prop from connector: {e}"))?;
    let crtc_props = card
        .get_properties(crtc.handle())
        .map_err(|e| anyhow!("could not get props of crtc: {e}"))?
        .as_hashmap(&card)
        .map_err(|e| anyhow!("could not get a prop from crtc: {e}"))?;
    let plane_props = card
        .get_properties(plane)
        .map_err(|e| anyhow!("could not get props of plane: {e}"))?
        .as_hashmap(&card)
        .map_err(|e| anyhow!("could not get a prop from plane: {e}"))?;

    let mut atomic_req = atomic::AtomicModeReq::new();
    atomic_req.add_property(
        first_connector.handle(),
        con_props["CRTC_ID"].handle(),
        property::Value::CRTC(Some(crtc.handle())),
    );
    let blob = card
        .create_property_blob(&mode)
        .map_err(|e| anyhow!("failed to create blob: {e}"))?;
    atomic_req.add_property(crtc.handle(), crtc_props["MODE_ID"].handle(), blob);
    atomic_req.add_property(
        crtc.handle(),
        crtc_props["ACTIVE"].handle(),
        property::Value::Boolean(true),
    );

    atomic_req.add_property(
        plane,
        plane_props["CRTC_ID"].handle(),
        property::Value::CRTC(Some(crtc.handle())),
    );
    atomic_req.add_property(
        plane,
        plane_props["SRC_X"].handle(),
        property::Value::UnsignedRange(0),
    );
    atomic_req.add_property(
        plane,
        plane_props["SRC_Y"].handle(),
        property::Value::UnsignedRange(0),
    );
    atomic_req.add_property(
        plane,
        plane_props["SRC_W"].handle(),
        property::Value::UnsignedRange((mode.size().0 as u64) << 16),
    );
    atomic_req.add_property(
        plane,
        plane_props["SRC_H"].handle(),
        property::Value::UnsignedRange((mode.size().1 as u64) << 16),
    );
    atomic_req.add_property(
        plane,
        plane_props["CRTC_X"].handle(),
        property::Value::SignedRange(0),
    );
    atomic_req.add_property(
        plane,
        plane_props["CRTC_Y"].handle(),
        property::Value::SignedRange(0),
    );
    atomic_req.add_property(
        plane,
        plane_props["CRTC_W"].handle(),
        property::Value::UnsignedRange(mode.size().0 as u64),
    );
    atomic_req.add_property(
        plane,
        plane_props["CRTC_H"].handle(),
        property::Value::UnsignedRange(mode.size().1 as u64),
    );

    start()?;

    for (index, image) in images.iter().enumerate() {
        let mut frame_request = atomic_req.clone();

        let image_location = fit_to_frame(image, output_rect);
        let resized_image_buffer = resize_to_display(image, image_location);

        let mut dumb_buffer = card
            .create_dumb_buffer(
                (width.into(), height.into()),
                DrmFourcc::Xrgb8888,
                BITS_PER_PIXEL,
            )
            .map_err(|e| anyhow!("failed to create dumb buffer: {e}"))?;

        {
            let mut map = card.map_dumb_buffer(&mut dumb_buffer)?;
            map.fill(0);
            let raw_image_buffer = resized_image_buffer.as_raw();
            copy_to_display(&mut map, raw_image_buffer, output_rect, image_location);
        }

        let frame_buffer = card.add_framebuffer(&dumb_buffer, DEPTH, BITS_PER_PIXEL)?;

        frame_request.add_property(
            plane,
            plane_props["FB_ID"].handle(),
            property::Value::Framebuffer(Some(frame_buffer)),
        );

        card.atomic_commit(AtomicCommitFlags::ALLOW_MODESET, frame_request)
            .map_err(|e| anyhow!("failed to atomic commit: {e}"))?;

        if index != images.len() - 1 {
            next()?;
        } else {
            wait()?;
        }

        let _ = card.destroy_framebuffer(frame_buffer);
        let _ = card.destroy_dumb_buffer(dumb_buffer);
    }
    Ok(())
}
