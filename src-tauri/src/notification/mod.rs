use std::sync::{Arc, Mutex};
use std::collections::HashSet;
use tauri::AppHandle;

/// 通知管理器
pub struct NotificationManager {
    app: AppHandle,
    notified_tasks: Arc<Mutex<HashSet<String>>>, // 已通知的任务ID集合
}

impl NotificationManager {
    pub fn new(app: AppHandle) -> Self {
        Self {
            app,
            notified_tasks: Arc::new(Mutex::new(HashSet::new())),
        }
    }

    /// 检查并发送截止时间通知
    pub fn check_and_notify(&self) {
        // 使用阻塞方式加载设置
        let app_handle = self.app.clone();
        let settings = tauri::async_runtime::block_on(async {
            crate::data::load_app_settings(app_handle).await
        });

        let settings = match settings {
            Ok(s) => s,
            Err(e) => {
                log::error!("加载设置失败: {}", e);
                return;
            }
        };

        // 检查是否启用通知
        if !settings.enable_deadline_notification {
            return;
        }

        // 使用阻塞方式加载待办事项数据
        let app_handle = self.app.clone();
        let todo_data = tauri::async_runtime::block_on(async {
            crate::data::load_todo_data_with_groups(app_handle).await
        });

        let todo_data = match todo_data {
            Ok(data) => data,
            Err(e) => {
                log::error!("加载待办事项数据失败: {}", e);
                return;
            }
        };

        let now = chrono::Utc::now().timestamp();
        let notification_threshold = (settings.notification_minutes_before as i64) * 60;

        let mut notified = self.notified_tasks.lock().unwrap();

        // 遍历所有未完成的任务
        for todo in &todo_data.todos {
            // 跳过已完成的任务
            if todo.completed {
                continue;
            }

            // 跳过没有截止时间的任务
            let deadline = match todo.deadline {
                Some(d) => d,
                None => continue,
            };

            // 计算距离截止时间的秒数
            let time_until_deadline = deadline - now;

            // 检查是否在通知时间窗口内（提前时间 ± 30秒）
            let should_notify = time_until_deadline > 0 
                && time_until_deadline <= notification_threshold + 30
                && time_until_deadline >= notification_threshold - 30;

            if should_notify && !notified.contains(&todo.id) {
                // 发送通知
                self.send_notification(todo, time_until_deadline);
                
                // 记录已通知
                notified.insert(todo.id.clone());
                
                log::info!("已发送通知: {}", todo.text);
            }

            // 清理已过期任务的通知记录
            if time_until_deadline < -3600 {
                notified.remove(&todo.id);
            }
        }
    }

    /// 发送系统通知
    fn send_notification(&self, todo: &crate::models::Todo, time_until_deadline: i64) {
        use tauri_plugin_notification::NotificationExt;
        
        // 计算剩余分钟数
        let minutes_left = (time_until_deadline / 60).max(0);
        
        // 格式化截止时间
        let deadline = todo.deadline.unwrap_or(0);
        let deadline_time = chrono::DateTime::from_timestamp(deadline, 0)
            .unwrap_or_else(|| chrono::Utc::now());
        
        let formatted_deadline = deadline_time
            .with_timezone(&chrono::Local)
            .format("%m月%d日 %H时%M分")
            .to_string();

        // 优先级文本
        let priority_text = if todo.priority == 1 {
            "高优先级"
        } else {
            "普通"
        };

        // 构建通知标题
        let title = if minutes_left == 0 {
            "你有一个即将到期的任务，请及时处理".to_string()
        } else {
            format!("你有一个 {} 分钟内到期的任务，请及时处理", minutes_left)
        };

        // 构建通知内容
        let body = format!(
            "目标任务：{}\n截止时间：{}\n优先级：{}",
            todo.text,
            formatted_deadline,
            priority_text
        );

        // 发送系统通知（带默认提示音）
        if let Err(e) = self.app.notification()
            .builder()
            .title(&title)
            .body(&body)
            .show() {
            log::error!("发送通知失败: {}", e);
        }
        
        // 播放系统提示音
        #[cfg(target_os = "windows")]
        {
            use std::io::Write;
            // 使用 BEL 字符触发系统提示音
            let _ = std::io::stdout().write_all(b"\x07");
            let _ = std::io::stdout().flush();
        }
    }


}
