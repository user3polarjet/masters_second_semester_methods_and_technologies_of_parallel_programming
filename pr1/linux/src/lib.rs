#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use nix::{ioctl_none, ioctl_read, ioctl_readwrite, ioctl_write_ptr};

const V4L2_IOC_MAGIC: u8 = b'V';

ioctl_read!(vidioc_querycap, V4L2_IOC_MAGIC, 0, v4l2_capability);
ioctl_readwrite!(vidioc_enum_fmt, V4L2_IOC_MAGIC, 2, v4l2_fmtdesc);
ioctl_readwrite!(vidioc_g_fmt, V4L2_IOC_MAGIC, 4, v4l2_format);
ioctl_readwrite!(vidioc_s_fmt, V4L2_IOC_MAGIC, 5, v4l2_format);
ioctl_readwrite!(vidioc_reqbufs, V4L2_IOC_MAGIC, 8, v4l2_requestbuffers);
ioctl_readwrite!(vidioc_querybuf, V4L2_IOC_MAGIC, 9, v4l2_buffer);
ioctl_read!(vidioc_g_fbuf, V4L2_IOC_MAGIC, 10, v4l2_framebuffer);
ioctl_write_ptr!(vidioc_s_fbuf, V4L2_IOC_MAGIC, 11, v4l2_framebuffer);
ioctl_write_ptr!(vidioc_overlay, V4L2_IOC_MAGIC, 14, i32);
ioctl_readwrite!(vidioc_qbuf, V4L2_IOC_MAGIC, 15, v4l2_buffer);
ioctl_readwrite!(vidioc_expbuf, V4L2_IOC_MAGIC, 16, v4l2_exportbuffer);
ioctl_readwrite!(vidioc_dqbuf, V4L2_IOC_MAGIC, 17, v4l2_buffer);
ioctl_write_ptr!(vidioc_streamon, V4L2_IOC_MAGIC, 18, i32);
ioctl_write_ptr!(vidioc_streamoff, V4L2_IOC_MAGIC, 19, i32);
ioctl_readwrite!(vidioc_g_parm, V4L2_IOC_MAGIC, 21, v4l2_streamparm);
ioctl_readwrite!(vidioc_s_parm, V4L2_IOC_MAGIC, 22, v4l2_streamparm);
ioctl_read!(vidioc_g_std, V4L2_IOC_MAGIC, 23, v4l2_std_id);
ioctl_write_ptr!(vidioc_s_std, V4L2_IOC_MAGIC, 24, v4l2_std_id);
ioctl_readwrite!(vidioc_enumstd, V4L2_IOC_MAGIC, 25, v4l2_standard);
ioctl_readwrite!(vidioc_enuminput, V4L2_IOC_MAGIC, 26, v4l2_input);
ioctl_readwrite!(vidioc_g_ctrl, V4L2_IOC_MAGIC, 27, v4l2_control);
ioctl_readwrite!(vidioc_s_ctrl, V4L2_IOC_MAGIC, 28, v4l2_control);
ioctl_readwrite!(vidioc_g_tuner, V4L2_IOC_MAGIC, 29, v4l2_tuner);
ioctl_write_ptr!(vidioc_s_tuner, V4L2_IOC_MAGIC, 30, v4l2_tuner);
ioctl_read!(vidioc_g_audio, V4L2_IOC_MAGIC, 33, v4l2_audio);
ioctl_write_ptr!(vidioc_s_audio, V4L2_IOC_MAGIC, 34, v4l2_audio);
ioctl_readwrite!(vidioc_queryctrl, V4L2_IOC_MAGIC, 36, v4l2_queryctrl);
ioctl_readwrite!(vidioc_querymenu, V4L2_IOC_MAGIC, 37, v4l2_querymenu);
ioctl_read!(vidioc_g_input, V4L2_IOC_MAGIC, 38, i32);
ioctl_readwrite!(vidioc_s_input, V4L2_IOC_MAGIC, 39, i32);
ioctl_readwrite!(vidioc_g_edid, V4L2_IOC_MAGIC, 40, v4l2_edid);
ioctl_readwrite!(vidioc_s_edid, V4L2_IOC_MAGIC, 41, v4l2_edid);
ioctl_read!(vidioc_g_output, V4L2_IOC_MAGIC, 46, i32);
ioctl_readwrite!(vidioc_s_output, V4L2_IOC_MAGIC, 47, i32);
ioctl_readwrite!(vidioc_enumoutput, V4L2_IOC_MAGIC, 48, v4l2_output);
ioctl_read!(vidioc_g_audout, V4L2_IOC_MAGIC, 49, v4l2_audioout);
ioctl_write_ptr!(vidioc_s_audout, V4L2_IOC_MAGIC, 50, v4l2_audioout);
ioctl_readwrite!(vidioc_g_modulator, V4L2_IOC_MAGIC, 54, v4l2_modulator);
ioctl_write_ptr!(vidioc_s_modulator, V4L2_IOC_MAGIC, 55, v4l2_modulator);
ioctl_readwrite!(vidioc_g_frequency, V4L2_IOC_MAGIC, 56, v4l2_frequency);
ioctl_write_ptr!(vidioc_s_frequency, V4L2_IOC_MAGIC, 57, v4l2_frequency);
ioctl_readwrite!(vidioc_cropcap, V4L2_IOC_MAGIC, 58, v4l2_cropcap);
ioctl_readwrite!(vidioc_g_crop, V4L2_IOC_MAGIC, 59, v4l2_crop);
ioctl_write_ptr!(vidioc_s_crop, V4L2_IOC_MAGIC, 60, v4l2_crop);
ioctl_read!(vidioc_g_jpegcomp, V4L2_IOC_MAGIC, 61, v4l2_jpegcompression);
ioctl_write_ptr!(vidioc_s_jpegcomp, V4L2_IOC_MAGIC, 62, v4l2_jpegcompression);
ioctl_read!(vidioc_querystd, V4L2_IOC_MAGIC, 63, v4l2_std_id);
ioctl_readwrite!(vidioc_try_fmt, V4L2_IOC_MAGIC, 64, v4l2_format);
ioctl_readwrite!(vidioc_enumaudio, V4L2_IOC_MAGIC, 65, v4l2_audio);
ioctl_readwrite!(vidioc_enumaudout, V4L2_IOC_MAGIC, 66, v4l2_audioout);
ioctl_read!(vidioc_g_priority, V4L2_IOC_MAGIC, 67, u32);
ioctl_write_ptr!(vidioc_s_priority, V4L2_IOC_MAGIC, 68, u32);
ioctl_readwrite!(vidioc_g_sliced_vbi_cap, V4L2_IOC_MAGIC, 69, v4l2_sliced_vbi_cap);
ioctl_none!(vidioc_log_status, V4L2_IOC_MAGIC, 70);
ioctl_readwrite!(vidioc_g_ext_ctrls, V4L2_IOC_MAGIC, 71, v4l2_ext_controls);
ioctl_readwrite!(vidioc_s_ext_ctrls, V4L2_IOC_MAGIC, 72, v4l2_ext_controls);
ioctl_readwrite!(vidioc_try_ext_ctrls, V4L2_IOC_MAGIC, 73, v4l2_ext_controls);
ioctl_readwrite!(vidioc_enum_framesizes, V4L2_IOC_MAGIC, 74, v4l2_frmsizeenum);
ioctl_readwrite!(vidioc_enum_frameintervals, V4L2_IOC_MAGIC, 75, v4l2_frmivalenum);
ioctl_read!(vidioc_g_enc_index, V4L2_IOC_MAGIC, 76, v4l2_enc_idx);
ioctl_readwrite!(vidioc_encoder_cmd, V4L2_IOC_MAGIC, 77, v4l2_encoder_cmd);
ioctl_readwrite!(vidioc_try_encoder_cmd, V4L2_IOC_MAGIC, 78, v4l2_encoder_cmd);
ioctl_write_ptr!(vidioc_dbg_s_register, V4L2_IOC_MAGIC, 79, v4l2_dbg_register);
ioctl_readwrite!(vidioc_dbg_g_register, V4L2_IOC_MAGIC, 80, v4l2_dbg_register);
ioctl_write_ptr!(vidioc_s_hw_freq_seek, V4L2_IOC_MAGIC, 82, v4l2_hw_freq_seek);
ioctl_readwrite!(vidioc_s_dv_timings, V4L2_IOC_MAGIC, 87, v4l2_dv_timings);
ioctl_readwrite!(vidioc_g_dv_timings, V4L2_IOC_MAGIC, 88, v4l2_dv_timings);
ioctl_read!(vidioc_dqevent, V4L2_IOC_MAGIC, 89, v4l2_event);
ioctl_write_ptr!(vidioc_subscribe_event, V4L2_IOC_MAGIC, 90, v4l2_event_subscription);
ioctl_write_ptr!(vidioc_unsubscribe_event, V4L2_IOC_MAGIC, 91, v4l2_event_subscription);
ioctl_readwrite!(vidioc_create_bufs, V4L2_IOC_MAGIC, 92, v4l2_create_buffers);
ioctl_readwrite!(vidioc_prepare_buf, V4L2_IOC_MAGIC, 93, v4l2_buffer);
ioctl_readwrite!(vidioc_g_selection, V4L2_IOC_MAGIC, 94, v4l2_selection);
ioctl_readwrite!(vidioc_s_selection, V4L2_IOC_MAGIC, 95, v4l2_selection);
ioctl_readwrite!(vidioc_decoder_cmd, V4L2_IOC_MAGIC, 96, v4l2_decoder_cmd);
ioctl_readwrite!(vidioc_try_decoder_cmd, V4L2_IOC_MAGIC, 97, v4l2_decoder_cmd);
ioctl_readwrite!(vidioc_enum_dv_timings, V4L2_IOC_MAGIC, 98, v4l2_enum_dv_timings);
ioctl_read!(vidioc_query_dv_timings, V4L2_IOC_MAGIC, 99, v4l2_dv_timings);
ioctl_readwrite!(vidioc_dv_timings_cap, V4L2_IOC_MAGIC, 100, v4l2_dv_timings_cap);
ioctl_readwrite!(vidioc_enum_freq_bands, V4L2_IOC_MAGIC, 101, v4l2_frequency_band);
ioctl_readwrite!(vidioc_dbg_g_chip_info, V4L2_IOC_MAGIC, 102, v4l2_dbg_chip_info);
ioctl_readwrite!(vidioc_query_ext_ctrl, V4L2_IOC_MAGIC, 103, v4l2_query_ext_ctrl);
// ioctl_readwrite!(vidioc_remove_bufs, V4L2_IOC_MAGIC, 104, v4l2_remove_buffers);
ioctl_read!(vidioc_subdev_querycap, V4L2_IOC_MAGIC, 0, v4l2_subdev_capability);
ioctl_readwrite!(vidioc_subdev_enum_mbus_code, V4L2_IOC_MAGIC, 2, v4l2_subdev_mbus_code_enum);
ioctl_readwrite!(vidioc_subdev_g_fmt, V4L2_IOC_MAGIC, 4, v4l2_subdev_format);
ioctl_readwrite!(vidioc_subdev_s_fmt, V4L2_IOC_MAGIC, 5, v4l2_subdev_format);
ioctl_readwrite!(vidioc_subdev_g_frame_interval, V4L2_IOC_MAGIC, 21, v4l2_subdev_frame_interval);
ioctl_readwrite!(vidioc_subdev_s_frame_interval, V4L2_IOC_MAGIC, 22, v4l2_subdev_frame_interval);
ioctl_readwrite!(vidioc_subdev_g_crop, V4L2_IOC_MAGIC, 59, v4l2_subdev_crop);
ioctl_readwrite!(vidioc_subdev_s_crop, V4L2_IOC_MAGIC, 60, v4l2_subdev_crop);
ioctl_readwrite!(vidioc_subdev_g_selection, V4L2_IOC_MAGIC, 61, v4l2_subdev_selection);
ioctl_readwrite!(vidioc_subdev_s_selection, V4L2_IOC_MAGIC, 62, v4l2_subdev_selection);
ioctl_readwrite!(vidioc_subdev_enum_frame_size, V4L2_IOC_MAGIC, 74, v4l2_subdev_frame_size_enum);
ioctl_readwrite!(vidioc_subdev_enum_frame_interval, V4L2_IOC_MAGIC, 75, v4l2_subdev_frame_interval_enum);
ioctl_read!(vidioc_subdev_g_client_cap, V4L2_IOC_MAGIC, 101, v4l2_subdev_client_capability);
ioctl_readwrite!(vidioc_subdev_s_client_cap, V4L2_IOC_MAGIC, 102, v4l2_subdev_client_capability);
ioctl_readwrite!(vidioc_subdev_g_routing, V4L2_IOC_MAGIC, 38, v4l2_subdev_routing);
ioctl_readwrite!(vidioc_subdev_s_routing, V4L2_IOC_MAGIC, 39, v4l2_subdev_routing);
ioctl_readwrite!(vidioc_subdev_enum_dv_timings, V4L2_IOC_MAGIC, 98, v4l2_enum_dv_timings);
ioctl_read!(vidioc_subdev_query_dv_timings, V4L2_IOC_MAGIC, 99, v4l2_dv_timings);
ioctl_readwrite!(vidioc_subdev_dv_timings_cap, V4L2_IOC_MAGIC, 100, v4l2_dv_timings_cap);
ioctl_readwrite!(vidioc_subdev_g_dv_timings, V4L2_IOC_MAGIC, 88, v4l2_dv_timings);
ioctl_readwrite!(vidioc_subdev_s_dv_timings, V4L2_IOC_MAGIC, 87, v4l2_dv_timings);
ioctl_read!(vidioc_subdev_querystd, V4L2_IOC_MAGIC, 63, v4l2_std_id);
ioctl_readwrite!(vidioc_subdev_enumstd, V4L2_IOC_MAGIC, 25, v4l2_standard);
ioctl_read!(vidioc_subdev_g_std, V4L2_IOC_MAGIC, 23, v4l2_std_id);
ioctl_write_ptr!(vidioc_subdev_s_std, V4L2_IOC_MAGIC, 24, v4l2_std_id);
ioctl_readwrite!(vidioc_subdev_g_edid, V4L2_IOC_MAGIC, 40, v4l2_edid);
ioctl_readwrite!(vidioc_subdev_s_edid, V4L2_IOC_MAGIC, 41, v4l2_edid);

const MEDIA_IOC_MAGIC: u8 = b'|';

ioctl_readwrite!(media_ioc_device_info, MEDIA_IOC_MAGIC, 0x00, media_device_info);
ioctl_readwrite!(media_ioc_enum_entities, MEDIA_IOC_MAGIC, 0x01, media_entity_desc);
ioctl_readwrite!(media_ioc_enum_links, MEDIA_IOC_MAGIC, 0x02, media_links_enum);
ioctl_readwrite!(media_ioc_setup_link, MEDIA_IOC_MAGIC, 0x03, media_link_desc);
ioctl_readwrite!(media_ioc_g_topology, MEDIA_IOC_MAGIC, 0x04, media_v2_topology);
ioctl_read!(media_ioc_request_alloc, MEDIA_IOC_MAGIC, 0x05, i32);
ioctl_none!(media_request_ioc_queue, MEDIA_IOC_MAGIC, 0x80);
ioctl_none!(media_request_ioc_reinit, MEDIA_IOC_MAGIC, 0x81);

// const DMA_HEAP_IOC_MAGIC: u8 = b'H';

ioctl_readwrite!(dma_heap_ioctl_alloc, DMA_HEAP_IOC_MAGIC, 0x00, dma_heap_allocation_data);

// const DMA_BUF_BASE: u8 = b'b';

ioctl_write_ptr!(dma_buf_ioctl_sync, DMA_BUF_BASE, 0x00, dma_buf_sync);
ioctl_write_ptr!(dma_buf_set_name, DMA_BUF_BASE, 0x01, u64);
ioctl_write_ptr!(dma_buf_set_name_a, DMA_BUF_BASE, 0x01, u32);
ioctl_write_ptr!(dma_buf_set_name_b, DMA_BUF_BASE, 0x01, u64);
ioctl_readwrite!(dma_buf_ioctl_export_sync_file, DMA_BUF_BASE, 0x02, dma_buf_export_sync_file);
ioctl_write_ptr!(dma_buf_ioctl_import_sync_file, DMA_BUF_BASE, 0x03, dma_buf_import_sync_file);

pub const fn v4l2_fourcc(a: char, b: char, c: char, d: char) -> u32 {
    (a as u32) | ((b as u32) << 8) | ((c as u32) << 16) | ((d as u32) << 24)
}

pub const fn v4l2_fourcc_be(a: char, b: char, c: char, d: char) -> u32 {
    v4l2_fourcc(a, b, c, d) | (1 << 31)
}

pub const V4L2_PIX_FMT_RGB332: u32 = v4l2_fourcc('R', 'G', 'B', '1'); /*  8  RGB-3-3-2     */
pub const V4L2_PIX_FMT_RGB444: u32 = v4l2_fourcc('R', '4', '4', '4'); /* 16  xxxxrrrr ggggbbbb */
pub const V4L2_PIX_FMT_ARGB444: u32 = v4l2_fourcc('A', 'R', '1', '2'); /* 16  aaaarrrr ggggbbbb */
pub const V4L2_PIX_FMT_XRGB444: u32 = v4l2_fourcc('X', 'R', '1', '2'); /* 16  xxxxrrrr ggggbbbb */
pub const V4L2_PIX_FMT_RGBA444: u32 = v4l2_fourcc('R', 'A', '1', '2'); /* 16  rrrrgggg bbbbaaaa */
pub const V4L2_PIX_FMT_RGBX444: u32 = v4l2_fourcc('R', 'X', '1', '2'); /* 16  rrrrgggg bbbbxxxx */
pub const V4L2_PIX_FMT_ABGR444: u32 = v4l2_fourcc('A', 'B', '1', '2'); /* 16  aaaabbbb ggggrrrr */
pub const V4L2_PIX_FMT_XBGR444: u32 = v4l2_fourcc('X', 'B', '1', '2'); /* 16  xxxxbbbb ggggrrrr */
pub const V4L2_PIX_FMT_BGRA444: u32 = v4l2_fourcc('G', 'A', '1', '2'); /* 16  bbbbgggg rrrraaaa */
pub const V4L2_PIX_FMT_BGRX444: u32 = v4l2_fourcc('B', 'X', '1', '2'); /* 16  bbbbgggg rrrrxxxx */
pub const V4L2_PIX_FMT_RGB555: u32 = v4l2_fourcc('R', 'G', 'B', 'O'); /* 16  RGB-5-5-5     */
pub const V4L2_PIX_FMT_ARGB555: u32 = v4l2_fourcc('A', 'R', '1', '5'); /* 16  ARGB-1-5-5-5  */
pub const V4L2_PIX_FMT_XRGB555: u32 = v4l2_fourcc('X', 'R', '1', '5'); /* 16  XRGB-1-5-5-5  */
pub const V4L2_PIX_FMT_RGBA555: u32 = v4l2_fourcc('R', 'A', '1', '5'); /* 16  RGBA-5-5-5-1  */
pub const V4L2_PIX_FMT_RGBX555: u32 = v4l2_fourcc('R', 'X', '1', '5'); /* 16  RGBX-5-5-5-1  */
pub const V4L2_PIX_FMT_ABGR555: u32 = v4l2_fourcc('A', 'B', '1', '5'); /* 16  ABGR-1-5-5-5  */
pub const V4L2_PIX_FMT_XBGR555: u32 = v4l2_fourcc('X', 'B', '1', '5'); /* 16  XBGR-1-5-5-5  */
pub const V4L2_PIX_FMT_BGRA555: u32 = v4l2_fourcc('B', 'A', '1', '5'); /* 16  BGRA-5-5-5-1  */
pub const V4L2_PIX_FMT_BGRX555: u32 = v4l2_fourcc('B', 'X', '1', '5'); /* 16  BGRX-5-5-5-1  */
pub const V4L2_PIX_FMT_RGB565: u32 = v4l2_fourcc('R', 'G', 'B', 'P'); /* 16  RGB-5-6-5     */
pub const V4L2_PIX_FMT_RGB555X: u32 = v4l2_fourcc('R', 'G', 'B', 'Q'); /* 16  RGB-5-5-5 BE  */
pub const V4L2_PIX_FMT_ARGB555X: u32 = v4l2_fourcc_be('A', 'R', '1', '5'); /* 16  ARGB-5-5-5 BE */
pub const V4L2_PIX_FMT_XRGB555X: u32 = v4l2_fourcc_be('X', 'R', '1', '5'); /* 16  XRGB-5-5-5 BE */
pub const V4L2_PIX_FMT_RGB565X: u32 = v4l2_fourcc('R', 'G', 'B', 'R'); /* 16  RGB-5-6-5 BE  */

/* RGB formats (3 or 4 bytes per pixel); */
pub const V4L2_PIX_FMT_BGR666: u32 = v4l2_fourcc('B', 'G', 'R', 'H'); /* 18  BGR-6-6-6	  */
pub const V4L2_PIX_FMT_BGR24: u32 = v4l2_fourcc('B', 'G', 'R', '3'); /* 24  BGR-8-8-8     */
pub const V4L2_PIX_FMT_RGB24: u32 = v4l2_fourcc('R', 'G', 'B', '3'); /* 24  RGB-8-8-8     */
pub const V4L2_PIX_FMT_BGR32: u32 = v4l2_fourcc('B', 'G', 'R', '4'); /* 32  BGR-8-8-8-8   */
pub const V4L2_PIX_FMT_ABGR32: u32 = v4l2_fourcc('A', 'R', '2', '4'); /* 32  BGRA-8-8-8-8  */
pub const V4L2_PIX_FMT_XBGR32: u32 = v4l2_fourcc('X', 'R', '2', '4'); /* 32  BGRX-8-8-8-8  */
pub const V4L2_PIX_FMT_BGRA32: u32 = v4l2_fourcc('R', 'A', '2', '4'); /* 32  ABGR-8-8-8-8  */
pub const V4L2_PIX_FMT_BGRX32: u32 = v4l2_fourcc('R', 'X', '2', '4'); /* 32  XBGR-8-8-8-8  */
pub const V4L2_PIX_FMT_RGB32: u32 = v4l2_fourcc('R', 'G', 'B', '4'); /* 32  RGB-8-8-8-8   */
pub const V4L2_PIX_FMT_RGBA32: u32 = v4l2_fourcc('A', 'B', '2', '4'); /* 32  RGBA-8-8-8-8  */
pub const V4L2_PIX_FMT_RGBX32: u32 = v4l2_fourcc('X', 'B', '2', '4'); /* 32  RGBX-8-8-8-8  */
pub const V4L2_PIX_FMT_ARGB32: u32 = v4l2_fourcc('B', 'A', '2', '4'); /* 32  ARGB-8-8-8-8  */
pub const V4L2_PIX_FMT_XRGB32: u32 = v4l2_fourcc('B', 'X', '2', '4'); /* 32  XRGB-8-8-8-8  */
pub const V4L2_PIX_FMT_RGBX1010102: u32 = v4l2_fourcc('R', 'X', '3', '0'); /* 32  RGBX-10-10-10-2 */
pub const V4L2_PIX_FMT_RGBA1010102: u32 = v4l2_fourcc('R', 'A', '3', '0'); /* 32  RGBA-10-10-10-2 */
pub const V4L2_PIX_FMT_ARGB2101010: u32 = v4l2_fourcc('A', 'R', '3', '0'); /* 32  ARGB-2-10-10-10 */

/* RGB formats (6 or 8 bytes per pixel); */
pub const V4L2_PIX_FMT_BGR48_12: u32 = v4l2_fourcc('B', '3', '1', '2'); /* 48  BGR 12-bit per component */
pub const V4L2_PIX_FMT_BGR48: u32 = v4l2_fourcc('B', 'G', 'R', '6'); /* 48  BGR 16-bit per component */
pub const V4L2_PIX_FMT_RGB48: u32 = v4l2_fourcc('R', 'G', 'B', '6'); /* 48  RGB 16-bit per component */
pub const V4L2_PIX_FMT_ABGR64_12: u32 = v4l2_fourcc('B', '4', '1', '2'); /* 64  BGRA 12-bit per component */

/* Grey formats */
pub const V4L2_PIX_FMT_GREY: u32 = v4l2_fourcc('G', 'R', 'E', 'Y'); /*  8  Greyscale     */
pub const V4L2_PIX_FMT_Y4: u32 = v4l2_fourcc('Y', '0', '4', ' '); /*  4  Greyscale     */
pub const V4L2_PIX_FMT_Y6: u32 = v4l2_fourcc('Y', '0', '6', ' '); /*  6  Greyscale     */
pub const V4L2_PIX_FMT_Y10: u32 = v4l2_fourcc('Y', '1', '0', ' '); /* 10  Greyscale     */
pub const V4L2_PIX_FMT_Y12: u32 = v4l2_fourcc('Y', '1', '2', ' '); /* 12  Greyscale     */
pub const V4L2_PIX_FMT_Y012: u32 = v4l2_fourcc('Y', '0', '1', '2'); /* 12  Greyscale     */
pub const V4L2_PIX_FMT_Y14: u32 = v4l2_fourcc('Y', '1', '4', ' '); /* 14  Greyscale     */
pub const V4L2_PIX_FMT_Y16: u32 = v4l2_fourcc('Y', '1', '6', ' '); /* 16  Greyscale     */
pub const V4L2_PIX_FMT_Y16_BE: u32 = v4l2_fourcc_be('Y', '1', '6', ' '); /* 16  Greyscale BE  */

/* Grey bit-packed formats */
pub const V4L2_PIX_FMT_Y10BPACK: u32 = v4l2_fourcc('Y', '1', '0', 'B'); /* 10  Greyscale bit-packed */
pub const V4L2_PIX_FMT_Y10P: u32 = v4l2_fourcc('Y', '1', '0', 'P'); /* 10  Greyscale, MIPI RAW10 packed */
pub const V4L2_PIX_FMT_IPU3_Y10: u32 = v4l2_fourcc('i', 'p', '3', 'y'); /* IPU3 packed 10-bit greyscale */
pub const V4L2_PIX_FMT_Y12P: u32 = v4l2_fourcc('Y', '1', '2', 'P'); /* 12  Greyscale, MIPI RAW12 packed */
pub const V4L2_PIX_FMT_Y14P: u32 = v4l2_fourcc('Y', '1', '4', 'P'); /* 14  Greyscale, MIPI RAW14 packed */

/* Palette formats */
pub const V4L2_PIX_FMT_PAL8: u32 = v4l2_fourcc('P', 'A', 'L', '8'); /*  8  8-bit palette */

/* Chrominance formats */
pub const V4L2_PIX_FMT_UV8: u32 = v4l2_fourcc('U', 'V', '8', ' '); /*  8  UV 4:4 */

/* Luminance+Chrominance formats */
pub const V4L2_PIX_FMT_YUYV: u32 = v4l2_fourcc('Y', 'U', 'Y', 'V'); /* 16  YUV 4:2:2     */
pub const V4L2_PIX_FMT_YYUV: u32 = v4l2_fourcc('Y', 'Y', 'U', 'V'); /* 16  YUV 4:2:2     */
pub const V4L2_PIX_FMT_YVYU: u32 = v4l2_fourcc('Y', 'V', 'Y', 'U'); /* 16 YVU 4:2:2 */
pub const V4L2_PIX_FMT_UYVY: u32 = v4l2_fourcc('U', 'Y', 'V', 'Y'); /* 16  YUV 4:2:2     */
pub const V4L2_PIX_FMT_VYUY: u32 = v4l2_fourcc('V', 'Y', 'U', 'Y'); /* 16  YUV 4:2:2     */
pub const V4L2_PIX_FMT_Y41P: u32 = v4l2_fourcc('Y', '4', '1', 'P'); /* 12  YUV 4:1:1     */
pub const V4L2_PIX_FMT_YUV444: u32 = v4l2_fourcc('Y', '4', '4', '4'); /* 16  xxxxyyyy uuuuvvvv */
pub const V4L2_PIX_FMT_YUV555: u32 = v4l2_fourcc('Y', 'U', 'V', 'O'); /* 16  YUV-5-5-5     */
pub const V4L2_PIX_FMT_YUV565: u32 = v4l2_fourcc('Y', 'U', 'V', 'P'); /* 16  YUV-5-6-5     */
pub const V4L2_PIX_FMT_YUV24: u32 = v4l2_fourcc('Y', 'U', 'V', '3'); /* 24  YUV-8-8-8     */
pub const V4L2_PIX_FMT_YUV32: u32 = v4l2_fourcc('Y', 'U', 'V', '4'); /* 32  YUV-8-8-8-8   */
pub const V4L2_PIX_FMT_AYUV32: u32 = v4l2_fourcc('A', 'Y', 'U', 'V'); /* 32  AYUV-8-8-8-8  */
pub const V4L2_PIX_FMT_XYUV32: u32 = v4l2_fourcc('X', 'Y', 'U', 'V'); /* 32  XYUV-8-8-8-8  */
pub const V4L2_PIX_FMT_VUYA32: u32 = v4l2_fourcc('V', 'U', 'Y', 'A'); /* 32  VUYA-8-8-8-8  */
pub const V4L2_PIX_FMT_VUYX32: u32 = v4l2_fourcc('V', 'U', 'Y', 'X'); /* 32  VUYX-8-8-8-8  */
pub const V4L2_PIX_FMT_YUVA32: u32 = v4l2_fourcc('Y', 'U', 'V', 'A'); /* 32  YUVA-8-8-8-8  */
pub const V4L2_PIX_FMT_YUVX32: u32 = v4l2_fourcc('Y', 'U', 'V', 'X'); /* 32  YUVX-8-8-8-8  */
pub const V4L2_PIX_FMT_M420: u32 = v4l2_fourcc('M', '4', '2', '0'); /* 12  YUV 4:2:0 2 lines y, 1 line uv interleaved */
pub const V4L2_PIX_FMT_YUV48_12: u32 = v4l2_fourcc('Y', '3', '1', '2'); /* 48  YUV 4:4:4 12-bit per component */

/*
 * YCbCr packed format. For each Y2xx format, xx bits of valid data occupy the MSBs
 * of the 16 bit components, and 16-xx bits of zero padding occupy the LSBs.
 */
pub const V4L2_PIX_FMT_Y210: u32 = v4l2_fourcc('Y', '2', '1', '0'); /* 32  YUYV 4:2:2 */
pub const V4L2_PIX_FMT_Y212: u32 = v4l2_fourcc('Y', '2', '1', '2'); /* 32  YUYV 4:2:2 */
pub const V4L2_PIX_FMT_Y216: u32 = v4l2_fourcc('Y', '2', '1', '6'); /* 32  YUYV 4:2:2 */

/* two planes -- one Y, one Cr + Cb interleaved  */
pub const V4L2_PIX_FMT_NV12: u32 = v4l2_fourcc('N', 'V', '1', '2'); /* 12  Y/CbCr 4:2:0  */
pub const V4L2_PIX_FMT_NV21: u32 = v4l2_fourcc('N', 'V', '2', '1'); /* 12  Y/CrCb 4:2:0  */
pub const V4L2_PIX_FMT_NV15: u32 = v4l2_fourcc('N', 'V', '1', '5'); /* 15  Y/CbCr 4:2:0 10-bit packed */
pub const V4L2_PIX_FMT_NV16: u32 = v4l2_fourcc('N', 'V', '1', '6'); /* 16  Y/CbCr 4:2:2  */
pub const V4L2_PIX_FMT_NV61: u32 = v4l2_fourcc('N', 'V', '6', '1'); /* 16  Y/CrCb 4:2:2  */
pub const V4L2_PIX_FMT_NV20: u32 = v4l2_fourcc('N', 'V', '2', '0'); /* 20  Y/CbCr 4:2:2 10-bit packed */
pub const V4L2_PIX_FMT_NV24: u32 = v4l2_fourcc('N', 'V', '2', '4'); /* 24  Y/CbCr 4:4:4  */
pub const V4L2_PIX_FMT_NV42: u32 = v4l2_fourcc('N', 'V', '4', '2'); /* 24  Y/CrCb 4:4:4  */
pub const V4L2_PIX_FMT_P010: u32 = v4l2_fourcc('P', '0', '1', '0'); /* 24  Y/CbCr 4:2:0 10-bit per component */
pub const V4L2_PIX_FMT_P012: u32 = v4l2_fourcc('P', '0', '1', '2'); /* 24  Y/CbCr 4:2:0 12-bit per component */

/* two non contiguous planes - one Y, one Cr + Cb interleaved  */
pub const V4L2_PIX_FMT_NV12M: u32 = v4l2_fourcc('N', 'M', '1', '2'); /* 12  Y/CbCr 4:2:0  */
pub const V4L2_PIX_FMT_NV21M: u32 = v4l2_fourcc('N', 'M', '2', '1'); /* 21  Y/CrCb 4:2:0  */
pub const V4L2_PIX_FMT_NV16M: u32 = v4l2_fourcc('N', 'M', '1', '6'); /* 16  Y/CbCr 4:2:2  */
pub const V4L2_PIX_FMT_NV61M: u32 = v4l2_fourcc('N', 'M', '6', '1'); /* 16  Y/CrCb 4:2:2  */
pub const V4L2_PIX_FMT_P012M: u32 = v4l2_fourcc('P', 'M', '1', '2'); /* 24  Y/CbCr 4:2:0 12-bit per component */

/* three planes - Y Cb, Cr */
pub const V4L2_PIX_FMT_YUV410: u32 = v4l2_fourcc('Y', 'U', 'V', '9'); /*  9  YUV 4:1:0     */
pub const V4L2_PIX_FMT_YVU410: u32 = v4l2_fourcc('Y', 'V', 'U', '9'); /*  9  YVU 4:1:0     */
pub const V4L2_PIX_FMT_YUV411P: u32 = v4l2_fourcc('4', '1', '1', 'P'); /* 12  YVU411 planar */
pub const V4L2_PIX_FMT_YUV420: u32 = v4l2_fourcc('Y', 'U', '1', '2'); /* 12  YUV 4:2:0     */
pub const V4L2_PIX_FMT_YVU420: u32 = v4l2_fourcc('Y', 'V', '1', '2'); /* 12  YVU 4:2:0     */
pub const V4L2_PIX_FMT_YUV422P: u32 = v4l2_fourcc('4', '2', '2', 'P'); /* 16  YVU422 planar */

/* three non contiguous planes - Y, Cb, Cr */
pub const V4L2_PIX_FMT_YUV420M: u32 = v4l2_fourcc('Y', 'M', '1', '2'); /* 12  YUV420 planar */
pub const V4L2_PIX_FMT_YVU420M: u32 = v4l2_fourcc('Y', 'M', '2', '1'); /* 12  YVU420 planar */
pub const V4L2_PIX_FMT_YUV422M: u32 = v4l2_fourcc('Y', 'M', '1', '6'); /* 16  YUV422 planar */
pub const V4L2_PIX_FMT_YVU422M: u32 = v4l2_fourcc('Y', 'M', '6', '1'); /* 16  YVU422 planar */
pub const V4L2_PIX_FMT_YUV444M: u32 = v4l2_fourcc('Y', 'M', '2', '4'); /* 24  YUV444 planar */
pub const V4L2_PIX_FMT_YVU444M: u32 = v4l2_fourcc('Y', 'M', '4', '2'); /* 24  YVU444 planar */

/* Tiled YUV formats */
pub const V4L2_PIX_FMT_NV12_4L4: u32 = v4l2_fourcc('V', 'T', '1', '2'); /* 12  Y/CbCr 4:2:0  4x4 tiles */
pub const V4L2_PIX_FMT_NV12_16L16: u32 = v4l2_fourcc('H', 'M', '1', '2'); /* 12  Y/CbCr 4:2:0 16x16 tiles */
pub const V4L2_PIX_FMT_NV12_32L32: u32 = v4l2_fourcc('S', 'T', '1', '2'); /* 12  Y/CbCr 4:2:0 32x32 tiles */
pub const V4L2_PIX_FMT_NV15_4L4: u32 = v4l2_fourcc('V', 'T', '1', '5'); /* 15 Y/CbCr 4:2:0 10-bit 4x4 tiles */
pub const V4L2_PIX_FMT_P010_4L4: u32 = v4l2_fourcc('T', '0', '1', '0'); /* 12  Y/CbCr 4:2:0 10-bit 4x4 macroblocks */
pub const V4L2_PIX_FMT_NV12_8L128: u32 = v4l2_fourcc('A', 'T', '1', '2'); /* Y/CbCr 4:2:0 8x128 tiles */
pub const V4L2_PIX_FMT_NV12_10BE_8L128: u32 = v4l2_fourcc_be('A', 'X', '1', '2'); /* Y/CbCr 4:2:0 10-bit 8x128 tiles */

/* Tiled YUV formats, non contiguous planes */
pub const V4L2_PIX_FMT_NV12MT: u32 = v4l2_fourcc('T', 'M', '1', '2'); /* 12  Y/CbCr 4:2:0 64x32 tiles */
pub const V4L2_PIX_FMT_NV12MT_16X16: u32 = v4l2_fourcc('V', 'M', '1', '2'); /* 12  Y/CbCr 4:2:0 16x16 tiles */
pub const V4L2_PIX_FMT_NV12M_8L128: u32 = v4l2_fourcc('N', 'A', '1', '2'); /* Y/CbCr 4:2:0 8x128 tiles */
pub const V4L2_PIX_FMT_NV12M_10BE_8L128: u32 = v4l2_fourcc_be('N', 'T', '1', '2'); /* Y/CbCr 4:2:0 10-bit 8x128 tiles */

/* Bayer formats - see http://www.siliconimaging.com/RGB%20Bayer.htm */
pub const V4L2_PIX_FMT_SBGGR8: u32 = v4l2_fourcc('B', 'A', '8', '1'); /*  8  BGBG.. GRGR.. */
pub const V4L2_PIX_FMT_SGBRG8: u32 = v4l2_fourcc('G', 'B', 'R', 'G'); /*  8  GBGB.. RGRG.. */
pub const V4L2_PIX_FMT_SGRBG8: u32 = v4l2_fourcc('G', 'R', 'B', 'G'); /*  8  GRGR.. BGBG.. */
pub const V4L2_PIX_FMT_SRGGB8: u32 = v4l2_fourcc('R', 'G', 'G', 'B'); /*  8  RGRG.. GBGB.. */
pub const V4L2_PIX_FMT_SBGGR10: u32 = v4l2_fourcc('B', 'G', '1', '0'); /* 10  BGBG.. GRGR.. */
pub const V4L2_PIX_FMT_SGBRG10: u32 = v4l2_fourcc('G', 'B', '1', '0'); /* 10  GBGB.. RGRG.. */
pub const V4L2_PIX_FMT_SGRBG10: u32 = v4l2_fourcc('B', 'A', '1', '0'); /* 10  GRGR.. BGBG.. */
pub const V4L2_PIX_FMT_SRGGB10: u32 = v4l2_fourcc('R', 'G', '1', '0'); /* 10  RGRG.. GBGB.. */
/* 10bit raw bayer packed, 5 bytes for every 4 pixels */
pub const V4L2_PIX_FMT_SBGGR10P: u32 = v4l2_fourcc('p', 'B', 'A', 'A');
pub const V4L2_PIX_FMT_SGBRG10P: u32 = v4l2_fourcc('p', 'G', 'A', 'A');
pub const V4L2_PIX_FMT_SGRBG10P: u32 = v4l2_fourcc('p', 'g', 'A', 'A');
pub const V4L2_PIX_FMT_SRGGB10P: u32 = v4l2_fourcc('p', 'R', 'A', 'A');
/* 10bit raw bayer a-law compressed to 8 bits */
pub const V4L2_PIX_FMT_SBGGR10ALAW8: u32 = v4l2_fourcc('a', 'B', 'A', '8');
pub const V4L2_PIX_FMT_SGBRG10ALAW8: u32 = v4l2_fourcc('a', 'G', 'A', '8');
pub const V4L2_PIX_FMT_SGRBG10ALAW8: u32 = v4l2_fourcc('a', 'g', 'A', '8');
pub const V4L2_PIX_FMT_SRGGB10ALAW8: u32 = v4l2_fourcc('a', 'R', 'A', '8');
/* 10bit raw bayer DPCM compressed to 8 bits */
pub const V4L2_PIX_FMT_SBGGR10DPCM8: u32 = v4l2_fourcc('b', 'B', 'A', '8');
pub const V4L2_PIX_FMT_SGBRG10DPCM8: u32 = v4l2_fourcc('b', 'G', 'A', '8');
pub const V4L2_PIX_FMT_SGRBG10DPCM8: u32 = v4l2_fourcc('B', 'D', '1', '0');
pub const V4L2_PIX_FMT_SRGGB10DPCM8: u32 = v4l2_fourcc('b', 'R', 'A', '8');
pub const V4L2_PIX_FMT_SBGGR12: u32 = v4l2_fourcc('B', 'G', '1', '2'); /* 12  BGBG.. GRGR.. */
pub const V4L2_PIX_FMT_SGBRG12: u32 = v4l2_fourcc('G', 'B', '1', '2'); /* 12  GBGB.. RGRG.. */
pub const V4L2_PIX_FMT_SGRBG12: u32 = v4l2_fourcc('B', 'A', '1', '2'); /* 12  GRGR.. BGBG.. */
pub const V4L2_PIX_FMT_SRGGB12: u32 = v4l2_fourcc('R', 'G', '1', '2'); /* 12  RGRG.. GBGB.. */
/* 12bit raw bayer packed, 3 bytes for every 2 pixels */
pub const V4L2_PIX_FMT_SBGGR12P: u32 = v4l2_fourcc('p', 'B', 'C', 'C');
pub const V4L2_PIX_FMT_SGBRG12P: u32 = v4l2_fourcc('p', 'G', 'C', 'C');
pub const V4L2_PIX_FMT_SGRBG12P: u32 = v4l2_fourcc('p', 'g', 'C', 'C');
pub const V4L2_PIX_FMT_SRGGB12P: u32 = v4l2_fourcc('p', 'R', 'C', 'C');
pub const V4L2_PIX_FMT_SBGGR14: u32 = v4l2_fourcc('B', 'G', '1', '4'); /* 14  BGBG.. GRGR.. */
pub const V4L2_PIX_FMT_SGBRG14: u32 = v4l2_fourcc('G', 'B', '1', '4'); /* 14  GBGB.. RGRG.. */
pub const V4L2_PIX_FMT_SGRBG14: u32 = v4l2_fourcc('G', 'R', '1', '4'); /* 14  GRGR.. BGBG.. */
pub const V4L2_PIX_FMT_SRGGB14: u32 = v4l2_fourcc('R', 'G', '1', '4'); /* 14  RGRG.. GBGB.. */
/* 14bit raw bayer packed, 7 bytes for every 4 pixels */
pub const V4L2_PIX_FMT_SBGGR14P: u32 = v4l2_fourcc('p', 'B', 'E', 'E');
pub const V4L2_PIX_FMT_SGBRG14P: u32 = v4l2_fourcc('p', 'G', 'E', 'E');
pub const V4L2_PIX_FMT_SGRBG14P: u32 = v4l2_fourcc('p', 'g', 'E', 'E');
pub const V4L2_PIX_FMT_SRGGB14P: u32 = v4l2_fourcc('p', 'R', 'E', 'E');
pub const V4L2_PIX_FMT_SBGGR16: u32 = v4l2_fourcc('B', 'Y', 'R', '2'); /* 16  BGBG.. GRGR.. */
pub const V4L2_PIX_FMT_SGBRG16: u32 = v4l2_fourcc('G', 'B', '1', '6'); /* 16  GBGB.. RGRG.. */
pub const V4L2_PIX_FMT_SGRBG16: u32 = v4l2_fourcc('G', 'R', '1', '6'); /* 16  GRGR.. BGBG.. */
pub const V4L2_PIX_FMT_SRGGB16: u32 = v4l2_fourcc('R', 'G', '1', '6'); /* 16  RGRG.. GBGB.. */

/* HSV formats */
pub const V4L2_PIX_FMT_HSV24: u32 = v4l2_fourcc('H', 'S', 'V', '3');
pub const V4L2_PIX_FMT_HSV32: u32 = v4l2_fourcc('H', 'S', 'V', '4');

/* compressed formats */
pub const V4L2_PIX_FMT_MJPEG: u32 = v4l2_fourcc('M', 'J', 'P', 'G'); /* Motion-JPEG   */
pub const V4L2_PIX_FMT_JPEG: u32 = v4l2_fourcc('J', 'P', 'E', 'G'); /* JFIF JPEG     */
pub const V4L2_PIX_FMT_DV: u32 = v4l2_fourcc('d', 'v', 's', 'd'); /* 1394          */
pub const V4L2_PIX_FMT_MPEG: u32 = v4l2_fourcc('M', 'P', 'E', 'G'); /* MPEG-1/2/4 Multiplexed */
pub const V4L2_PIX_FMT_H264: u32 = v4l2_fourcc('H', '2', '6', '4'); /* H264 with start codes */
pub const V4L2_PIX_FMT_H264_NO_SC: u32 = v4l2_fourcc('A', 'V', 'C', '1'); /* H264 without start codes */
pub const V4L2_PIX_FMT_H264_MVC: u32 = v4l2_fourcc('M', '2', '6', '4'); /* H264 MVC */
pub const V4L2_PIX_FMT_H263: u32 = v4l2_fourcc('H', '2', '6', '3'); /* H263          */
pub const V4L2_PIX_FMT_MPEG1: u32 = v4l2_fourcc('M', 'P', 'G', '1'); /* MPEG-1 ES     */
pub const V4L2_PIX_FMT_MPEG2: u32 = v4l2_fourcc('M', 'P', 'G', '2'); /* MPEG-2 ES     */
pub const V4L2_PIX_FMT_MPEG2_SLICE: u32 = v4l2_fourcc('M', 'G', '2', 'S'); /* MPEG-2 parsed slice data */
pub const V4L2_PIX_FMT_MPEG4: u32 = v4l2_fourcc('M', 'P', 'G', '4'); /* MPEG-4 part 2 ES */
pub const V4L2_PIX_FMT_XVID: u32 = v4l2_fourcc('X', 'V', 'I', 'D'); /* Xvid           */
pub const V4L2_PIX_FMT_VC1_ANNEX_G: u32 = v4l2_fourcc('V', 'C', '1', 'G'); /* SMPTE 421M Annex G compliant stream */
pub const V4L2_PIX_FMT_VC1_ANNEX_L: u32 = v4l2_fourcc('V', 'C', '1', 'L'); /* SMPTE 421M Annex L compliant stream */
pub const V4L2_PIX_FMT_VP8: u32 = v4l2_fourcc('V', 'P', '8', '0'); /* VP8 */
pub const V4L2_PIX_FMT_VP8_FRAME: u32 = v4l2_fourcc('V', 'P', '8', 'F'); /* VP8 parsed frame */
pub const V4L2_PIX_FMT_VP9: u32 = v4l2_fourcc('V', 'P', '9', '0'); /* VP9 */
pub const V4L2_PIX_FMT_VP9_FRAME: u32 = v4l2_fourcc('V', 'P', '9', 'F'); /* VP9 parsed frame */
pub const V4L2_PIX_FMT_HEVC: u32 = v4l2_fourcc('H', 'E', 'V', 'C'); /* HEVC aka H.265 */
pub const V4L2_PIX_FMT_FWHT: u32 = v4l2_fourcc('F', 'W', 'H', 'T'); /* Fast Walsh Hadamard Transform (vicodec) */
pub const V4L2_PIX_FMT_FWHT_STATELESS: u32 = v4l2_fourcc('S', 'F', 'W', 'H'); /* Stateless FWHT (vicodec) */
pub const V4L2_PIX_FMT_H264_SLICE: u32 = v4l2_fourcc('S', '2', '6', '4'); /* H264 parsed slices */
pub const V4L2_PIX_FMT_HEVC_SLICE: u32 = v4l2_fourcc('S', '2', '6', '5'); /* HEVC parsed slices */
pub const V4L2_PIX_FMT_AV1_FRAME: u32 = v4l2_fourcc('A', 'V', '1', 'F'); /* AV1 parsed frame */
pub const V4L2_PIX_FMT_SPK: u32 = v4l2_fourcc('S', 'P', 'K', '0'); /* Sorenson Spark */
pub const V4L2_PIX_FMT_RV30: u32 = v4l2_fourcc('R', 'V', '3', '0'); /* RealVideo 8 */
pub const V4L2_PIX_FMT_RV40: u32 = v4l2_fourcc('R', 'V', '4', '0'); /* RealVideo 9 & 10 */

/*  Vendor-specific formats   */
pub const V4L2_PIX_FMT_CPIA1: u32 = v4l2_fourcc('C', 'P', 'I', 'A'); /* cpia1 YUV */
pub const V4L2_PIX_FMT_WNVA: u32 = v4l2_fourcc('W', 'N', 'V', 'A'); /* Winnov hw compress */
pub const V4L2_PIX_FMT_SN9C10X: u32 = v4l2_fourcc('S', '9', '1', '0'); /* SN9C10x compression */
pub const V4L2_PIX_FMT_SN9C20X_I420: u32 = v4l2_fourcc('S', '9', '2', '0'); /* SN9C20x YUV 4:2:0 */
pub const V4L2_PIX_FMT_PWC1: u32 = v4l2_fourcc('P', 'W', 'C', '1'); /* pwc older webcam */
pub const V4L2_PIX_FMT_PWC2: u32 = v4l2_fourcc('P', 'W', 'C', '2'); /* pwc newer webcam */
pub const V4L2_PIX_FMT_ET61X251: u32 = v4l2_fourcc('E', '6', '2', '5'); /* ET61X251 compression */
pub const V4L2_PIX_FMT_SPCA501: u32 = v4l2_fourcc('S', '5', '0', '1'); /* YUYV per line */
pub const V4L2_PIX_FMT_SPCA505: u32 = v4l2_fourcc('S', '5', '0', '5'); /* YYUV per line */
pub const V4L2_PIX_FMT_SPCA508: u32 = v4l2_fourcc('S', '5', '0', '8'); /* YUVY per line */
pub const V4L2_PIX_FMT_SPCA561: u32 = v4l2_fourcc('S', '5', '6', '1'); /* compressed GBRG bayer */
pub const V4L2_PIX_FMT_PAC207: u32 = v4l2_fourcc('P', '2', '0', '7'); /* compressed BGGR bayer */
pub const V4L2_PIX_FMT_MR97310A: u32 = v4l2_fourcc('M', '3', '1', '0'); /* compressed BGGR bayer */
pub const V4L2_PIX_FMT_JL2005BCD: u32 = v4l2_fourcc('J', 'L', '2', '0'); /* compressed RGGB bayer */
pub const V4L2_PIX_FMT_SN9C2028: u32 = v4l2_fourcc('S', 'O', 'N', 'X'); /* compressed GBRG bayer */
pub const V4L2_PIX_FMT_SQ905C: u32 = v4l2_fourcc('9', '0', '5', 'C'); /* compressed RGGB bayer */
pub const V4L2_PIX_FMT_PJPG: u32 = v4l2_fourcc('P', 'J', 'P', 'G'); /* Pixart 73xx JPEG */
pub const V4L2_PIX_FMT_OV511: u32 = v4l2_fourcc('O', '5', '1', '1'); /* ov511 JPEG */
pub const V4L2_PIX_FMT_OV518: u32 = v4l2_fourcc('O', '5', '1', '8'); /* ov518 JPEG */
pub const V4L2_PIX_FMT_STV0680: u32 = v4l2_fourcc('S', '6', '8', '0'); /* stv0680 bayer */
pub const V4L2_PIX_FMT_TM6000: u32 = v4l2_fourcc('T', 'M', '6', '0'); /* tm5600/tm60x0 */
pub const V4L2_PIX_FMT_CIT_YYVYUY: u32 = v4l2_fourcc('C', 'I', 'T', 'V'); /* one line of Y then 1 line of VYUY */
pub const V4L2_PIX_FMT_KONICA420: u32 = v4l2_fourcc('K', 'O', 'N', 'I'); /* YUV420 planar in blocks of 256 pixels */
pub const V4L2_PIX_FMT_JPGL: u32 = v4l2_fourcc('J', 'P', 'G', 'L'); /* JPEG-Lite */
pub const V4L2_PIX_FMT_SE401: u32 = v4l2_fourcc('S', '4', '0', '1'); /* se401 janggu compressed rgb */
pub const V4L2_PIX_FMT_S5C_UYVY_JPG: u32 = v4l2_fourcc('S', '5', 'C', 'I'); /* S5C73M3 interleaved UYVY/JPEG */
pub const V4L2_PIX_FMT_Y8I: u32 = v4l2_fourcc('Y', '8', 'I', ' '); /* Greyscale 8-bit L/R interleaved */
pub const V4L2_PIX_FMT_Y12I: u32 = v4l2_fourcc('Y', '1', '2', 'I'); /* Greyscale 12-bit L/R interleaved */
pub const V4L2_PIX_FMT_Y16I: u32 = v4l2_fourcc('Y', '1', '6', 'I'); /* Greyscale 16-bit L/R interleaved */
pub const V4L2_PIX_FMT_Z16: u32 = v4l2_fourcc('Z', '1', '6', ' '); /* Depth data 16-bit */
pub const V4L2_PIX_FMT_MT21C: u32 = v4l2_fourcc('M', 'T', '2', '1'); /* Mediatek compressed block mode  */
pub const V4L2_PIX_FMT_MM21: u32 = v4l2_fourcc('M', 'M', '2', '1'); /* Mediatek 8-bit block mode, two non-contiguous planes */
pub const V4L2_PIX_FMT_MT2110T: u32 = v4l2_fourcc('M', 'T', '2', 'T'); /* Mediatek 10-bit block tile mode */
pub const V4L2_PIX_FMT_MT2110R: u32 = v4l2_fourcc('M', 'T', '2', 'R'); /* Mediatek 10-bit block raster mode */
pub const V4L2_PIX_FMT_INZI: u32 = v4l2_fourcc('I', 'N', 'Z', 'I'); /* Intel Planar Greyscale 10-bit and Depth 16-bit */
pub const V4L2_PIX_FMT_CNF4: u32 = v4l2_fourcc('C', 'N', 'F', '4'); /* Intel 4-bit packed depth confidence information */
pub const V4L2_PIX_FMT_HI240: u32 = v4l2_fourcc('H', 'I', '2', '4'); /* BTTV 8-bit dithered RGB */
pub const V4L2_PIX_FMT_QC08C: u32 = v4l2_fourcc('Q', '0', '8', 'C'); /* Qualcomm 8-bit compressed */
pub const V4L2_PIX_FMT_QC10C: u32 = v4l2_fourcc('Q', '1', '0', 'C'); /* Qualcomm 10-bit compressed */
pub const V4L2_PIX_FMT_AJPG: u32 = v4l2_fourcc('A', 'J', 'P', 'G'); /* Aspeed JPEG */
pub const V4L2_PIX_FMT_HEXTILE: u32 = v4l2_fourcc('H', 'X', 'T', 'L'); /* Hextile compressed */

/* 10bit raw packed, 32 bytes for every 25 pixels, last LSB 6 bits unused */
pub const V4L2_PIX_FMT_IPU3_SBGGR10: u32 = v4l2_fourcc('i', 'p', '3', 'b'); /* IPU3 packed 10-bit BGGR bayer */
pub const V4L2_PIX_FMT_IPU3_SGBRG10: u32 = v4l2_fourcc('i', 'p', '3', 'g'); /* IPU3 packed 10-bit GBRG bayer */
pub const V4L2_PIX_FMT_IPU3_SGRBG10: u32 = v4l2_fourcc('i', 'p', '3', 'G'); /* IPU3 packed 10-bit GRBG bayer */
pub const V4L2_PIX_FMT_IPU3_SRGGB10: u32 = v4l2_fourcc('i', 'p', '3', 'r'); /* IPU3 packed 10-bit RGGB bayer */

/* Raspberry Pi PiSP compressed formats. */
pub const V4L2_PIX_FMT_PISP_COMP1_RGGB: u32 = v4l2_fourcc('P', 'C', '1', 'R'); /* PiSP 8-bit mode 1 compressed RGGB bayer */
pub const V4L2_PIX_FMT_PISP_COMP1_GRBG: u32 = v4l2_fourcc('P', 'C', '1', 'G'); /* PiSP 8-bit mode 1 compressed GRBG bayer */
pub const V4L2_PIX_FMT_PISP_COMP1_GBRG: u32 = v4l2_fourcc('P', 'C', '1', 'g'); /* PiSP 8-bit mode 1 compressed GBRG bayer */
pub const V4L2_PIX_FMT_PISP_COMP1_BGGR: u32 = v4l2_fourcc('P', 'C', '1', 'B'); /* PiSP 8-bit mode 1 compressed BGGR bayer */
pub const V4L2_PIX_FMT_PISP_COMP1_MONO: u32 = v4l2_fourcc('P', 'C', '1', 'M'); /* PiSP 8-bit mode 1 compressed monochrome */
pub const V4L2_PIX_FMT_PISP_COMP2_RGGB: u32 = v4l2_fourcc('P', 'C', '2', 'R'); /* PiSP 8-bit mode 2 compressed RGGB bayer */
pub const V4L2_PIX_FMT_PISP_COMP2_GRBG: u32 = v4l2_fourcc('P', 'C', '2', 'G'); /* PiSP 8-bit mode 2 compressed GRBG bayer */
pub const V4L2_PIX_FMT_PISP_COMP2_GBRG: u32 = v4l2_fourcc('P', 'C', '2', 'g'); /* PiSP 8-bit mode 2 compressed GBRG bayer */
pub const V4L2_PIX_FMT_PISP_COMP2_BGGR: u32 = v4l2_fourcc('P', 'C', '2', 'B'); /* PiSP 8-bit mode 2 compressed BGGR bayer */
pub const V4L2_PIX_FMT_PISP_COMP2_MONO: u32 = v4l2_fourcc('P', 'C', '2', 'M'); /* PiSP 8-bit mode 2 compressed monochrome */

/* Renesas RZ/V2H CRU packed formats. 64-bit units with contiguous pixels */
pub const V4L2_PIX_FMT_RAW_CRU10: u32 = v4l2_fourcc('C', 'R', '1', '0');
pub const V4L2_PIX_FMT_RAW_CRU12: u32 = v4l2_fourcc('C', 'R', '1', '2');
pub const V4L2_PIX_FMT_RAW_CRU14: u32 = v4l2_fourcc('C', 'R', '1', '4');
pub const V4L2_PIX_FMT_RAW_CRU20: u32 = v4l2_fourcc('C', 'R', '2', '0');

/* SDR formats - used only for Software Defined Radio devices */
pub const V4L2_SDR_FMT_CU8: u32 = v4l2_fourcc('C', 'U', '0', '8'); /* IQ u8 */
pub const V4L2_SDR_FMT_CU16LE: u32 = v4l2_fourcc('C', 'U', '1', '6'); /* IQ u16le */
pub const V4L2_SDR_FMT_CS8: u32 = v4l2_fourcc('C', 'S', '0', '8'); /* complex s8 */
pub const V4L2_SDR_FMT_CS14LE: u32 = v4l2_fourcc('C', 'S', '1', '4'); /* complex s14le */
pub const V4L2_SDR_FMT_RU12LE: u32 = v4l2_fourcc('R', 'U', '1', '2'); /* real u12le */
pub const V4L2_SDR_FMT_PCU16BE: u32 = v4l2_fourcc('P', 'C', '1', '6'); /* planar complex u16be */
pub const V4L2_SDR_FMT_PCU18BE: u32 = v4l2_fourcc('P', 'C', '1', '8'); /* planar complex u18be */
pub const V4L2_SDR_FMT_PCU20BE: u32 = v4l2_fourcc('P', 'C', '2', '0'); /* planar complex u20be */

/* Touch formats - used for Touch devices */
pub const V4L2_TCH_FMT_DELTA_TD16: u32 = v4l2_fourcc('T', 'D', '1', '6'); /* 16-bit signed deltas */
pub const V4L2_TCH_FMT_DELTA_TD08: u32 = v4l2_fourcc('T', 'D', '0', '8'); /* 8-bit signed deltas */
pub const V4L2_TCH_FMT_TU16: u32 = v4l2_fourcc('T', 'U', '1', '6'); /* 16-bit unsigned touch data */
pub const V4L2_TCH_FMT_TU08: u32 = v4l2_fourcc('T', 'U', '0', '8'); /* 8-bit unsigned touch data */

/* Meta-data formats */
pub const V4L2_META_FMT_VSP1_HGO: u32 = v4l2_fourcc('V', 'S', 'P', 'H'); /* R-Car VSP1 1-D Histogram */
pub const V4L2_META_FMT_VSP1_HGT: u32 = v4l2_fourcc('V', 'S', 'P', 'T'); /* R-Car VSP1 2-D Histogram */
pub const V4L2_META_FMT_UVC: u32 = v4l2_fourcc('U', 'V', 'C', 'H'); /* UVC Payload Header metadata */
pub const V4L2_META_FMT_D4XX: u32 = v4l2_fourcc('D', '4', 'X', 'X'); /* D4XX Payload Header metadata */
pub const V4L2_META_FMT_UVC_MSXU_1_5: u32 = v4l2_fourcc('U', 'V', 'C', 'M'); /* UVC MSXU metadata */
pub const V4L2_META_FMT_VIVID: u32 = v4l2_fourcc('V', 'I', 'V', 'D'); /* Vivid Metadata */

/* Vendor specific - used for RK_ISP1 camera sub-system */
pub const V4L2_META_FMT_RK_ISP1_PARAMS: u32 = v4l2_fourcc('R', 'K', '1', 'P'); /* Rockchip ISP1 3A Parameters */
pub const V4L2_META_FMT_RK_ISP1_STAT_3A: u32 = v4l2_fourcc('R', 'K', '1', 'S'); /* Rockchip ISP1 3A Statistics */
pub const V4L2_META_FMT_RK_ISP1_EXT_PARAMS: u32 = v4l2_fourcc('R', 'K', '1', 'E'); /* Rockchip ISP1 3a Extensible Parameters */

/* Vendor specific - used for C3_ISP */
pub const V4L2_META_FMT_C3ISP_PARAMS: u32 = v4l2_fourcc('C', '3', 'P', 'M'); /* Amlogic C3 ISP Parameters */
pub const V4L2_META_FMT_C3ISP_STATS: u32 = v4l2_fourcc('C', '3', 'S', 'T'); /* Amlogic C3 ISP Statistics */

/* Vendor specific - used for RaspberryPi PiSP */
pub const V4L2_META_FMT_RPI_BE_CFG: u32 = v4l2_fourcc('R', 'P', 'B', 'C'); /* PiSP BE configuration */
pub const V4L2_META_FMT_RPI_FE_CFG: u32 = v4l2_fourcc('R', 'P', 'F', 'C'); /* PiSP FE configuration */
pub const V4L2_META_FMT_RPI_FE_STATS: u32 = v4l2_fourcc('R', 'P', 'F', 'S'); /* PiSP FE stats */

/* Vendor specific - used for Arm Mali-C55 ISP */
pub const V4L2_META_FMT_MALI_C55_PARAMS: u32 = v4l2_fourcc('C', '5', '5', 'P'); /* ARM Mali-C55 Parameters */
pub const V4L2_META_FMT_MALI_C55_STATS: u32 = v4l2_fourcc('C', '5', '5', 'S'); /* ARM Mali-C55 3A Statistics */
