use crate::SysClass;
use std::io::Result;
use std::path::{Path, PathBuf};

pub type SlaveIter = Box<dyn Iterator<Item = Result<PathBuf>>>;

/// A block device in /sys/class/block
#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Block {
    path: PathBuf,
}

impl SysClass for Block {
    fn class() -> &'static str {
        "block"
    }

    unsafe fn from_path_unchecked(path: PathBuf) -> Self {
        Self { path }
    }

    fn path(&self) -> &Path {
        &self.path
    }
}

impl Block {
    pub fn has_device(&self) -> bool {
        self.path.join("device").exists()
    }

    pub fn children(&self) -> Result<Vec<Self>> {
        let mut children = Block::all()?
            .into_iter()
            .filter(|x| {
                x.parent_device()
                    .map_or(false, |parent| parent.path() == self.path)
            })
            .collect::<Vec<_>>();
        children.sort_unstable();
        Ok(children)
    }

    pub fn parent_device(&self) -> Option<Block> {
        self.partition().ok().and_then(|partition| {
            let path = self.path().to_str()?;
            let pos = path.len() - partition as usize / 10 - 1;
            let path = Path::new(path.split_at(pos).0).to_path_buf();
            Some(unsafe { Block::from_path_unchecked(path) })
        })
    }

    /// Logical devices have their parent device(s) listed here.
    ///
    /// For example:
    ///
    /// - dm-4 has a slave of dm-0
    /// - dm-0 has a slave of sda3
    /// - sda3 does not have any slaves
    pub fn slaves(&self) -> Option<Result<SlaveIter>> {
        let slaves_path = self.path.join("slaves");
        if slaves_path.exists() {
            let res: Result<SlaveIter> = match slaves_path.read_dir() {
                Ok(iter) => Ok(Box::new(iter.map(|entry| Ok(entry?.path())))),
                Err(why) => Err(why),
            };

            Some(res)
        } else {
            None
        }
    }

    // Base properties

    method!(alignment_offset parse_file u64);

    method!(capability parse_file u8);

    method!(dev read_file String);

    method!(discard_alignment parse_file u64);

    method!(events parse_file u64);

    method!(events_async parse_file u64);

    method!(events_poll_msecs parse_file u64);

    method!(ext_range parse_file u64);

    method!(hidden parse_file u8);

    method!(inflight read_file String);

    method!(partition parse_file u8);

    method!(range parse_file u64);

    method!(removable parse_file u8);

    method!(ro parse_file u8);

    method!(size parse_file u64);

    method!(start parse_file u64);

    method!(stat parse_file u8);

    method!(subsystem parse_file u8);

    method!(uevent read_file String);

    // bdi

    // device

    method!("device/device_blocked", device_blocked parse_file u8);

    method!("device/device_busy", device_busy parse_file u8);

    method!("device/model", device_model read_file String);

    method!("device/rev", device_rev read_file String);

    method!("device/state", device_state read_file String);

    method!("device/vendor", device_vendor read_file String);

    // holders

    // integrity

    // md

    method!("md/array_size", md_array_size read_file String);

    method!("md/array_state", md_array_state read_file String);

    method!("md/chunk_size", md_chunk_size parse_file u64);

    method!("md/component_size", md_component_size parse_file u64);

    method!("md/degraded", md_degraded parse_file u8);

    method!("md/layout", md_layout parse_file u64);

    method!("md/level", md_level read_file String);

    method!("md/metadata_version", md_metadata_version read_file String);

    method!("md/mismatch_cnt", md_mismatch_count parse_file u64);

    method!("md/preread_bypass_threshold", md_preread_bypass_threshold parse_file u64);

    method!("md/raid_disks", md_raid_disks parse_file u64);

    method!("md/reshape_position", md_reshape_position read_file String);

    method!("md/resync_start", md_resync_start read_file String);

    method!("md/safe_mode_delay", md_safe_mode_delay parse_file f64);

    method!("md/stripe_cache_active", md_stripe_cache_active parse_file u8);

    method!("md/stripe_cache_size", md_stripe_cache_size parse_file u64);

    method!("md/suspend_hi", md_suspend_hi parse_file u64);

    method!("md/suspend_lo", md_suspend_lo parse_file u64);

    method!("md/sync_action", md_sync_action read_file String);

    method!("md/sync_completed", md_sync_completed read_file String);

    method!("md/sync_force_parallel", md_sync_force_parallel parse_file u8);

    method!("md/sync_max", md_sync_max read_file String);

    method!("md/sync_min", md_sync_min parse_file u64);

    method!("md/sync_speed", md_sync_speed read_file String);

    method!("md/sync_speed_max", md_sync_speed_max read_file String);

    method!("md/sync_speed_min", md_sync_speed_min read_file String);

    // power

    // trace

    // queue

    method!("queue/add_random", queue_add_random parse_file u64);

    method!("queue/chunk_sectors", queue_chunk_sectors parse_file u64);

    method!("queue/dax", queue_dax parse_file u64);

    method!("queue/discard_granularity", queue_discard_granularity parse_file u64);

    method!("queue/discard_max_bytes", queue_discard_max_bytes parse_file u64);

    method!("queue/discard_max_hw_bytes", queue_discard_max_hw_bytes parse_file u64);

    method!("queue/discard_zeroes_data", queue_discard_zeroes_data parse_file u64);

    method!("queue/fua", queue_fua parse_file u64);

    method!("queue/hw_sector_size", queue_hw_sector_size parse_file u64);

    method!("queue/io_poll", queue_io_poll parse_file u64);

    method!("queue/io_poll_delay", queue_io_poll_delay parse_file u64);

    method!("queue/iostats", queue_iostats parse_file u64);

    method!("queue/logical_block_size", queue_logical_block_size parse_file u64);

    method!("queue/max_discard_segments", queue_max_discard_segments parse_file u64);

    method!("queue/max_hw_sectors_kb", queue_max_hw_sectors_kb parse_file u64);

    method!("queue/max_integrity_segments", queue_max_integrity_segments parse_file u64);

    method!("queue/max_sectors_kb", queue_max_sectors_kb parse_file u64);

    method!("queue/max_segment_size", queue_max_segment_size parse_file u64);

    method!("queue/max_segments", queue_max_segments parse_file u64);

    method!("queue/minimum_io_size", queue_minimum_io_size parse_file u64);

    method!("queue/nomerges", queue_nomerges parse_file u64);

    method!("queue/nr_requests", queue_nr_requests parse_file u64);

    method!("queue/optimal_io_size", queue_optimal_io_size parse_file u64);

    method!("queue/physical_block_size", queue_physical_block_size parse_file u64);

    method!("queue/read_ahead_kb", queue_read_ahead_kb parse_file u64);

    method!("queue/rotational", queue_rotational parse_file u8);

    method!("queue/rq_affinity", queue_rq_affinity parse_file u64);

    // method!("queue/scheduler", queue_scheduler parse_file u64);
    pub fn queue_scheduler(&self) -> Result<BlockScheduler> {
        let mut active = 0;
        let mut schedules = Vec::new();
        for schedule in self.read_file("queue/scheduler")?.split_whitespace() {
            let schedule = if schedule.starts_with('[') {
                active = schedules.len();
                &schedule[1..schedule.len() - 1]
            } else {
                schedule
            };

            schedules.push(schedule.to_owned());
        }

        Ok(BlockScheduler {
            active: active as u8,
            schedules,
        })
    }

    method!("queue/write_cache", queue_write_cache read_file String);

    method!("queue/write_same_max_bytes", queue_write_same_max_bytes parse_file u64);

    method!("queue/write_zeroes_max_bytes", queue_write_zeroes_max_bytes parse_file u64);

    method!("queue/zoned", queue_zoned read_file String);

    // queue/iosched

    method!("queue/iosched/back_seek_max", queue_iosched_back_seek_max parse_file u64);

    method!("queue/iosched/back_seek_penalty", queue_iosched_back_seek_penalty parse_file u64);

    method!("queue/iosched/fifo_expire_async", queue_iosched_fifo_expire_async parse_file u64);

    method!("queue/iosched/fifo_expire_sync", queue_iosched_fifo_expire_sync parse_file u64);

    method!("queue/iosched/group_idle", queue_iosched_group_idle parse_file u64);

    method!("queue/iosched/group_idle_us", queue_iosched_group_idle_us parse_file u64);

    method!("queue/iosched/low_latency", queue_iosched_low_latency parse_file u8);

    method!("queue/iosched/quantum", queue_iosched_quantum parse_file u64);

    method!("queue/iosched/slice_async", queue_iosched_slice_async parse_file u64);

    method!("queue/iosched/slice_async_rq", queue_iosched_slice_async_rq parse_file u64);

    method!("queue/iosched/slice_async_us", queue_iosched_slice_async_us parse_file u64);

    method!("queue/iosched/slice_idle", queue_iosched_slice_idle parse_file u8);

    method!("queue/iosched/slice_idle_us", queue_iosched_slice_idle_us parse_file u64);

    method!("queue/iosched/slice_sync", queue_iosched_slice_sync parse_file u64);

    method!("queue/iosched/slice_sync_us", queue_iosched_slice_sync_us parse_file u64);

    method!("queue/iosched/target_latency", queue_iosched_target_latency parse_file u64);

    method!("queue/iosched/target_latency_us", queue_iosched_target_latency_us parse_file u64);
}

pub struct BlockScheduler {
    schedules: Vec<String>,
    active: u8,
}

impl BlockScheduler {
    pub fn active(&self) -> &str {
        &self.schedules[self.active as usize]
    }

    pub fn schedulers(&self) -> &[String] {
        &self.schedules
    }
}
