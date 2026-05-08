/// Supported UI languages.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Lang {
    Zh,
    En,
}

impl Lang {
        /// Parse a language code string, defaulting to Chinese.
        pub fn from_str(s: &str) -> Self {
        match s {
            "en" => Lang::En,
            _ => Lang::Zh,
        }
    }
}

macro_rules! build_translations {
    ($($key:expr => { zh: $zh:expr, en: $en:expr }),* $(,)?) => {
        pub fn t(key: &str, lang: Lang) -> String {
            match lang {
                Lang::Zh => match key {
                    $($key => $zh.to_string(),)*
                    _ => key.to_string(),
                },
                Lang::En => match key {
                    $($key => $en.to_string(),)*
                    _ => key.to_string(),
                },
            }
        }
    };
}

build_translations! {
    "app.title" => { zh: "Wormhole", en: "Wormhole" },
    "password.unlock" => { zh: "解锁 Wormhole", en: "Unlock Wormhole" },
    "password.enter" => { zh: "输入主密码", en: "Enter master password" },
    "password.set" => { zh: "设置主密码", en: "Set Master Password" },
    "password.set_hint" => { zh: "首次使用，请设置主密码", en: "First time use, please set a master password" },
    "password.confirm" => { zh: "确认主密码", en: "Confirm Master Password" },
    "password.confirm_hint" => { zh: "再次输入主密码", en: "Re-enter master password" },
    "password.min_length" => { zh: "密码至少6位", en: "Password must be at least 6 characters" },
    "password.mismatch" => { zh: "两次密码不一致", en: "Passwords do not match" },
    "password.wrong" => { zh: "密码错误", en: "Wrong password" },
    "password.encrypt_failed" => { zh: "加密失败", en: "Encryption failed" },
    "password.confirm_btn" => { zh: "[Enter] 确认   [Esc] 跳过   [r] 重置", en: "[Enter] Confirm   [Esc] Skip   [r] Reset" },
    "detail.address" => { zh: "地址", en: "Address" },
    "detail.user" => { zh: "用户", en: "User" },
    "detail.auth" => { zh: "认证", en: "Auth" },
    "detail.color" => { zh: "颜色", en: "Color" },
    "detail.notes" => { zh: "备注", en: "Notes" },
    "detail.none" => { zh: "(无)", en: "(none)" },
    "detail.default" => { zh: "默认", en: "Default" },
    "detail.select_hint" => { zh: "选择一个主机查看详情", en: "Select a host to view details" },
    "detail.auth_password" => { zh: "密码", en: "Password" },
    "detail.auth_key" => { zh: "密钥", en: "Key" },
    "detail.auth_interactive" => { zh: "交互式", en: "Interactive" },
    "detail.manual_input" => { zh: "手动输入", en: "Manual input" },
    "detail.not_set" => { zh: "(未设置)", en: "(not set)" },
    "detail.welcome" => { zh: "欢迎使用 Wormhole", en: "Welcome to Wormhole" },
    "detail.welcome_hint" => { zh: "按 [n] 创建你的第一台主机", en: "Press [n] to create your first host" },
    "detail.action.connect" => { zh: "连接", en: "Connect" },
    "detail.action.edit" => { zh: "编辑", en: "Edit" },
    "detail.action.delete" => { zh: "删除", en: "Delete" },
    "detail.action.new_host" => { zh: "新建主机", en: "New Host" },
    "detail.action.new_group" => { zh: "新建分组", en: "New Group" },
    "detail.action.search" => { zh: "搜索", en: "Search" },
    "detail.action.quit" => { zh: "退出", en: "Quit" },
    "detail.tab.overview" => { zh: "概览", en: "Overview" },
    "detail.tab.connection" => { zh: "连接", en: "Connection" },
    "detail.tab.notes" => { zh: "备注", en: "Notes" },
    "detail.tab.switch_hint" => { zh: "Tab 切换标签", en: "Tab to switch" },
    "detail.group.host_count" => { zh: "台主机", en: "hosts" },
    "detail.group.empty" => { zh: "暂无主机", en: "No hosts" },
    "shortcuts.connect" => { zh: "[Enter] 连接", en: "[Enter] Connect" },
    "shortcuts.edit" => { zh: "[e] 编辑", en: "[e] Edit" },
    "shortcuts.delete" => { zh: "[d] 删除", en: "[d] Delete" },
    "shortcuts.new_host" => { zh: "[n] 新建主机", en: "[n] New Host" },
    "shortcuts.new_group" => { zh: "[g] 新建分组", en: "[g] New Group" },
    "shortcuts.search" => { zh: "[/] 搜索", en: "[/] Search" },
    "shortcuts.quit" => { zh: "[q] 退出", en: "[q] Quit" },
    "shortcuts.theme" => { zh: "[t] 切换主题", en: "[t] Switch Theme" },
    "bottom_bar.hint" => { zh: " [/] 搜索   [?] 帮助   [q] 退出", en: " [/] Search   [?] Help   [q] Quit" },
    "help.title" => { zh: "快捷键", en: "Shortcuts" },
    "help.move_down" => { zh: "下移光标", en: "Move down" },
    "help.move_up" => { zh: "上移光标", en: "Move up" },
    "help.switch_panel" => { zh: "左右切换面板", en: "Switch panel focus" },
    "help.connect" => { zh: "连接选中主机", en: "Connect to host" },
    "help.edit" => { zh: "编辑选中项", en: "Edit selected" },
    "help.delete" => { zh: "删除选中项", en: "Delete selected" },
    "help.new_host" => { zh: "新建主机", en: "New host" },
    "help.new_group" => { zh: "新建分组", en: "New group" },
    "help.search" => { zh: "搜索", en: "Search" },
    "help.escape" => { zh: "退出搜索/取消", en: "Exit search/cancel" },
    "help.quit" => { zh: "退出 wormhole", en: "Quit wormhole" },
    "help.help" => { zh: "显示帮助", en: "Show help" },
    "help.theme" => { zh: "切换主题", en: "Switch theme" },
    "help.lang" => { zh: "切换语言", en: "Switch language" },
    "help.reorder" => { zh: "上移/下移选中项", en: "Move selected item up/down" },
    "help.close" => { zh: "按任意键关闭", en: "Press any key to close" },
    "confirm.delete" => { zh: "确认删除该主机？", en: "Delete this host?" },
    "confirm.delete_group" => { zh: "删除该分组", en: "Delete this group" },
    "confirm.delete_group_all" => { zh: "[d] 删除分组及所有主机", en: "[d] Delete group and all hosts" },
    "confirm.delete_group_move" => { zh: "[m] 仅删除分组（主机移至根目录）", en: "[m] Delete group only (hosts move to root)" },
    "confirm.reset" => { zh: "确认重置？所有数据将被清除，无法恢复", en: "Confirm reset? All data will be erased and cannot be recovered" },
    "confirm.yes" => { zh: "[y] 确认   [n/Esc] 取消", en: "[y] Confirm   [n/Esc] Cancel" },
    "editor.title" => { zh: "\u{F03EB} 编辑主机", en: "\u{F03EB} Edit Host" },
    "editor.title_new" => { zh: "\u{F0415} 新建主机", en: "\u{F0415} New Host" },
    "editor.save" => { zh: "[Enter] 保存   [Tab] 下一字段   [Esc] 取消", en: "[Enter] Save   [Tab] Next Field   [Esc] Cancel" },
    "editor.field.display_name" => { zh: "展示名", en: "Name" },
    "editor.field.address" => { zh: "地址", en: "Address" },
    "editor.field.port" => { zh: "端口", en: "Port" },
    "editor.field.username" => { zh: "用户名", en: "Username" },
    "editor.field.auth_type" => { zh: "认证方式", en: "Auth Type" },
    "editor.field.key_path" => { zh: "密钥路径", en: "Key Path" },
    "editor.field.password" => { zh: "密码", en: "Password" },
    "editor.group.none" => { zh: "无分组", en: "No Group" },
    "editor.field.group" => { zh: "分组ID", en: "Group ID" },
    "editor.field.icon" => { zh: "图标", en: "Icon" },
    "editor.field.color" => { zh: "颜色", en: "Color" },
    "editor.field.notes" => { zh: "备注", en: "Notes" },
    "editor.auth.key" => { zh: "\u{F030B} 密钥认证", en: "\u{F030B} Key" },
    "editor.auth.password" => { zh: "\u{F033E} 密码认证", en: "\u{F033E} Password" },
    "editor.auth.interactive" => { zh: "\u{EA85} 交互式输入", en: "\u{EA85} Interactive" },
    "editor.select_hint" => { zh: "← → 切换选项", en: "← → to switch" },
    "group.default_name" => { zh: "分组", en: "Group" },
    "group.create_title" => { zh: "新建分组", en: "New Group" },
    "group.name_hint" => { zh: "输入分组名称", en: "Enter group name" },
    "group.name_empty" => { zh: "名称不能为空", en: "Name cannot be empty" },
    "confirm.skip_password.title" => { zh: "跳过主密码设置？", en: "Skip Master Password?" },
    "confirm.skip_password.warning" => { zh: "跳过后，主机密码将以明文形式存储在本地文件中", en: "If skipped, host passwords will be stored in plain text locally" },
    "confirm.skip_password.path" => { zh: "存储位置：", en: "Storage path:" },
    "theme.title" => { zh: "切换主题", en: "Switch Theme" },
    "theme.hint" => { zh: "[↑↓] 预览   [Enter] 保存   [Esc] 取消", en: "[↑↓] Preview   [Enter] Save   [Esc] Cancel" },
    "lang.title" => { zh: "切换语言", en: "Switch Language" },
    "lang.hint" => { zh: "[↑↓] 预览   [Enter] 保存   [Esc] 取消", en: "[↑↓] Preview   [Enter] Save   [Esc] Cancel" },
}

use std::sync::RwLock;

static CURRENT_LANG: RwLock<Lang> = RwLock::new(Lang::Zh);

/// Set the active language.
pub fn set_lang(lang: Lang) {
    if let Ok(mut guard) = CURRENT_LANG.write() {
        *guard = lang;
    }
}

/// Return the currently active language.
pub fn current_lang() -> Lang {
    CURRENT_LANG.read().map(|g| *g).unwrap_or(Lang::Zh)
}

/// All supported languages in display order.
pub fn all_langs() -> &'static [(Lang, &'static str)] {
    &[(Lang::Zh, "中文"), (Lang::En, "English")]
}

/// Convert a Lang to its config code.
pub fn lang_code(lang: Lang) -> &'static str {
    match lang {
        Lang::Zh => "zh",
        Lang::En => "en",
    }
}

/// Translate a key to the current language, returning the key itself on miss.
pub fn tr(key: &str) -> String {
    t(key, current_lang())
}
