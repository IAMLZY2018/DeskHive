use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

// 拖动防抖结构
#[derive(Clone)]
pub struct DragDebouncer {
    last_move_time: Arc<Mutex<Option<Instant>>>,
    pending_position: Arc<Mutex<Option<(i32, i32)>>>,
}

impl DragDebouncer {
    pub fn new() -> Self {
        Self {
            last_move_time: Arc::new(Mutex::new(None)),
            pending_position: Arc::new(Mutex::new(None)),
        }
    }
    
    // 记录拖动事件，但不立即处理
    pub fn on_drag_move(&self, x: i32, y: i32) {
        if let Ok(mut last_time) = self.last_move_time.lock() {
            *last_time = Some(Instant::now());
        }
        if let Ok(mut pending_pos) = self.pending_position.lock() {
            *pending_pos = Some((x, y));
        }
    }
    
    // 检查是否应该处理拖动结束事件
    pub fn should_process_drag_end(&self, debounce_duration: Duration) -> Option<(i32, i32)> {
        if let (Ok(last_time), Ok(mut pending_pos)) = (self.last_move_time.lock(), self.pending_position.lock()) {
            if let (Some(time), Some(pos)) = (*last_time, *pending_pos) {
                if time.elapsed() >= debounce_duration {
                    *pending_pos = None; // 清除待处理位置
                    return Some(pos);
                }
            }
        }
        None
    }
}